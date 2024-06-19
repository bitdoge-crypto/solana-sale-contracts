use anchor_lang::{
  prelude::*,
  solana_program::{ program::invoke, system_instruction::transfer },
};
use anchor_spl::token::{ self, Token, TokenAccount, Transfer as SplTransfer };
use pyth_sdk_solana::{ load_price_feed_from_account_info, PriceFeed, Price };
use std::str::FromStr;

use crate::errors;
use crate::events;
use crate::state::store::*;
use crate::state::epoc::Epoc;
use crate::state::promoter::Promoter;
use crate::state::customer::Customer;

use crate::config::{
  SOL_USD_PRICEFEED, TREASURY, USDC, USDT,
  PRECISION, STABLE_PRECISION, PROMOTER_TAG,
  CUSTOMER_TAG, EMPTY_PROMOTER, STALENESS_THRESHOLD
};

pub fn init_store(
  ctx: Context<InitStore>,
) -> Result<()> {
  let store = &mut ctx.accounts.store;
  store.init()
}

pub fn set_store_cap(
  ctx: Context<SetStoreCap>,
  max_cap: u64,
  min_cap: u64,
) -> Result<()> {
  let store = &mut ctx.accounts.store;
  store.set_cap(max_cap, min_cap)
}

pub fn set_store_fee(
  ctx: Context<SetStoreReward>,
  first_fee: u64,
  second_fee: u64,
) -> Result<()> {
  let store = &mut ctx.accounts.store;
  store.set_fee(first_fee, second_fee)
}

pub fn enable_store(
  ctx: Context<SetStoreEnabled>,
) -> Result<()> {
  let store = &mut ctx.accounts.store;
  store.set_enable()
}

pub fn disable_store(
  ctx: Context<SetStoreDisabled>,
) -> Result<()> {
  let store = &mut ctx.accounts.store;
  store.set_disable()
}

pub fn deposit_with_sol(
  ctx: Context<Deposit>,
  promoter_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let to_account_infos = &mut ctx.accounts.to_account_infos();
  let payer = &mut ctx.accounts.payer;
  let store = &mut ctx.accounts.store;
  let epoc = &mut ctx.accounts.epoc;
  let customer = &mut ctx.accounts.customer;
  let promoter = &mut ctx.accounts.promoter;
  let price_info = &ctx.accounts.price_info;
  let treasury_info = &mut ctx.accounts.treasury_info;

  if !store.is_enabled() {
    return err!(errors::Store::StoreNotEnabled);
  }

  if !epoc.is_enabled() {
    return err!(errors::Store::EpocNotEnabled);
  }

  if store.get_epoc() != epoc.get_id() {
    return err!(errors::Store::InactiveEpoc);
  }

  if Pubkey::from_str(TREASURY) != Ok(treasury_info.key()){
    return Err(error!(errors::Store::WrongTreasury))
  };

  if Pubkey::from_str(SOL_USD_PRICEFEED) != Ok(price_info.key()){
    return Err(error!(errors::Store::WrongPriceFeedId))
  };
  
  let (price, expo) = get_price(&price_info).unwrap();
  let usd_amount = u128::from(amount) * price / 10u128.pow(expo);
  let asset_amount = usd_amount * 10u128.pow(PRECISION) / u128::from(epoc.get_price());

  if store.get_max_cap() < usd_amount {
    return err!(errors::Store::StoreMaxCapExceeded);
  }

  if store.get_min_cap() > usd_amount {
    return err!(errors::Store::StoreMinCapNotReached);
  }

  if epoc.get_total_sold() + asset_amount > epoc.get_total_supply() {
    return err!(errors::Store::EpocSupplyExceeded);
  }
  
  let (promoter_sol_amount, promoter_asset_amount) = get_fee(store, promoter_key, promoter, amount, asset_amount).unwrap();
  let mut to_amount = amount;
  if promoter_sol_amount > 0 {
    to_amount = to_amount - promoter_sol_amount;
  }

  let instruction = &transfer(&payer.key(), &treasury_info.key(), to_amount);
  invoke(instruction, to_account_infos).unwrap();

  if promoter_sol_amount > 0 {
    let instruction = &transfer(&payer.key(), &promoter.key(), promoter_sol_amount);
    invoke(instruction, to_account_infos).unwrap();
  }

  // Updating store details
  store.set_total_sold(asset_amount).unwrap();

  // Updating epoc details
  epoc.set_total_sold(asset_amount).unwrap();

  // Updating customer details
  customer.set_asset_amount(asset_amount).unwrap();

  // Updating promoter details
  if Pubkey::from_str(EMPTY_PROMOTER) != Ok(promoter_key){
    promoter.set_sol_fee_amount(promoter_sol_amount).unwrap();
    promoter.set_asset_amount(promoter_asset_amount).unwrap();
  };

  emit!(events::DepositWithSolEvent {
    epoc: epoc.get_id(),
    customer: payer.key(),
    promoter: promoter_key,
    sol_amount: amount,
    asset_amount: asset_amount,
  });
  Ok(())
}

pub fn deposit_with_usdc(
  ctx: Context<DepositUSDC>,
  promoter_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let store = &mut ctx.accounts.store;
  let epoc = &mut ctx.accounts.epoc;
  let customer = &mut ctx.accounts.customer;
  let promoter = &mut ctx.accounts.promoter;

  let customer_ata = &ctx.accounts.customer_ata;
  let treasury_ata = &ctx.accounts.treasury_ata;
  let promoter_pda_ata = &ctx.accounts.promoter_pda_ata;
  let asset_program = &ctx.accounts.asset_program;

  if !store.is_enabled() {
    return err!(errors::Store::StoreNotEnabled);
  }

  if !epoc.is_enabled() {
    return err!(errors::Store::EpocNotEnabled);
  }

  if store.get_epoc() != epoc.get_id() {
    return err!(errors::Store::InactiveEpoc);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let asset_amount = usd_amount * 10u128.pow(PRECISION) / u128::from(epoc.get_price());

  if store.get_max_cap() < usd_amount {
    return err!(errors::Store::StoreMaxCapExceeded);
  }

  if store.get_min_cap() > usd_amount {
    return err!(errors::Store::StoreMinCapNotReached);
  }

  if epoc.get_total_sold() + asset_amount > epoc.get_total_supply() {
    return err!(errors::Store::EpocSupplyExceeded);
  }

  let (promoter_stable_fee_amount, promoter_asset_amount) = get_fee(store, promoter_key, promoter, amount, asset_amount).unwrap();
  let mut to_amount = amount;
  if promoter_stable_fee_amount > 0 {
    to_amount = to_amount - promoter_stable_fee_amount;
  }

  let cpi_accounts = SplTransfer {
    from: customer_ata.to_account_info(),
    to: treasury_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = asset_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if promoter_stable_fee_amount > 0 {
    let cpi_accounts = SplTransfer {
      from: customer_ata.to_account_info(),
      to: promoter_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = asset_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), promoter_stable_fee_amount).unwrap();
  }

  // Updating store details
  store.set_total_sold(asset_amount).unwrap();

  // Updating epoc details
  epoc.set_total_sold(asset_amount).unwrap();

  // Updating customer details
  customer.set_asset_amount(asset_amount).unwrap();

  // Updating promoter details
  if Pubkey::from_str(EMPTY_PROMOTER) != Ok(promoter_key){
    promoter.set_usdc_amount(promoter_stable_fee_amount).unwrap();
    promoter.set_asset_amount(promoter_asset_amount).unwrap();
  };

  emit!(events::DepositWithUsdcEvent {
    epoc: epoc.get_id(),
    customer: payer.key(),
    promoter: promoter_key,
    usdc_amount: amount,
    asset_amount: asset_amount,
  });

  Ok(())
}

pub fn deposit_with_usdt(
  ctx: Context<DepositUSDT>,
  promoter_key: Pubkey,
  amount: u64,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let store = &mut ctx.accounts.store;
  let epoc = &mut ctx.accounts.epoc;
  let customer = &mut ctx.accounts.customer;
  let promoter = &mut ctx.accounts.promoter;

  let customer_ata = &ctx.accounts.customer_ata;
  let treasury_ata = &ctx.accounts.treasury_ata;
  let promoter_pda_ata = &ctx.accounts.promoter_pda_ata;
  let asset_program = &ctx.accounts.asset_program;

  if !store.is_enabled() {
    return err!(errors::Store::StoreNotEnabled);
  }

  if !epoc.is_enabled() {
    return err!(errors::Store::EpocNotEnabled);
  }

  if store.get_epoc() != epoc.get_id() {
    return err!(errors::Store::InactiveEpoc);
  }

  let usd_amount = u128::from(amount) * 10u128.pow(STABLE_PRECISION);
  let asset_amount = usd_amount * 10u128.pow (PRECISION) / u128::from(epoc.get_price());

  if store.get_max_cap() < usd_amount {
    return err!(errors::Store::StoreMaxCapExceeded);
  }

  if store.get_min_cap() > usd_amount {
    return err!(errors::Store::StoreMinCapNotReached);
  }

  if epoc.get_total_sold() + asset_amount > epoc.get_total_supply() {
    return err!(errors::Store::EpocSupplyExceeded);
  }

  let (promoter_stable_fee_amount, promoter_asset_amount) = get_fee(store, promoter_key, promoter, amount, asset_amount).unwrap();
  let mut to_amount = amount;
  if promoter_stable_fee_amount > 0 {
    to_amount = to_amount - promoter_stable_fee_amount;
  }

  let cpi_accounts = SplTransfer {
    from: customer_ata.to_account_info(),
    to: treasury_ata.to_account_info(),
    authority: payer.to_account_info(),
  };
  let cpi_program = asset_program.to_account_info();
  token::transfer(CpiContext::new(cpi_program, cpi_accounts), to_amount).unwrap();
  
  if promoter_stable_fee_amount > 0 {
    let cpi_accounts = SplTransfer {
      from: customer_ata.to_account_info(),
      to: promoter_pda_ata.to_account_info(),
      authority: payer.to_account_info(),
    };
    let cpi_program = asset_program.to_account_info();
    token::transfer(CpiContext::new(cpi_program, cpi_accounts), promoter_stable_fee_amount).unwrap();
  }

  // Updating store details
  store.set_total_sold(asset_amount).unwrap();

  // Updating epoc details
  epoc.set_total_sold(asset_amount).unwrap();

  // Updating customer details
  customer.set_asset_amount(asset_amount).unwrap();

  // Updating promoter details
  if Pubkey::from_str(EMPTY_PROMOTER) != Ok(promoter_key){
    promoter.set_usdt_amount(promoter_stable_fee_amount).unwrap();
    promoter.set_asset_amount(promoter_asset_amount).unwrap();
  };

  emit!(events::DepositWithUsdtEvent {
    epoc: epoc.get_id(),
    customer: payer.key(),
    promoter: promoter_key,
    usdt_amount: amount,
    asset_amount: asset_amount,
  });

  Ok(())
}

pub fn get_price(price_info: &AccountInfo)
  -> Result<(u128, u32)>
{
  let price_feed: PriceFeed = load_price_feed_from_account_info( &price_info ).unwrap();
  let current_timestamp = Clock::get()?.unix_timestamp;
  let current_price: Price = price_feed.get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD).unwrap();

  let price = u64::try_from(current_price.price).unwrap();
  let expo = u32::try_from(-current_price.expo).unwrap();
  Ok((u128::from(price), expo))
}

pub fn get_fee(
  store: &mut Account<Store>,
  promoter_key: Pubkey,
  promoter: &mut Account<Promoter>,
  amount: u64,
  asset_amount: u128,
)
  -> Result<(u64, u128)>
{
  if Pubkey::from_str(EMPTY_PROMOTER) == Ok(promoter_key){
    return Ok((0, 0));
  };

  let (store_main_fee, store_secondary_fee) = store.get_fee();
  let (promoter_main_fee, promoter_secondary_fee) = promoter.get_fee();

  let first_fee = u64::max(store_main_fee, promoter_main_fee);
  let second_fee = u64::max(store_secondary_fee, promoter_secondary_fee);

  let amount = amount * first_fee / 10u64.pow(PRECISION);
  let asset_amount = asset_amount * u128::from(second_fee) / 10u128.pow(PRECISION);

  Ok((amount, asset_amount))
}

#[derive(Accounts)]
pub struct InitStore<'info> {
  #[account(
    init,
    payer = payer,
    space = 8 + Store::MAX_SIZE,
    seeds = [],
    bump,
  )]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(max_cap: u64, min_cap: u64)]
pub struct SetStoreCap<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(first_fee: u64, second_fee: u64)]
pub struct SetStoreReward<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetStoreEnabled<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetStoreDisabled<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(promoter_key: Pubkey, amount: u64)]
pub struct Deposit<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Customer::MAX_SIZE,
    seeds = [
      CUSTOMER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub customer: Account<'info, Customer>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Promoter::MAX_SIZE,
    seeds = [
      PROMOTER_TAG,
      b"_",
      promoter_key.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  /// CHECK : We will manually check this against the Pubkey of the price feed
  pub price_info : AccountInfo<'info>,
  /// CHECK : We will manually check this against the Pubkey of the treasury
  #[account(mut)]
  pub treasury_info : AccountInfo<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(promoter_key: Pubkey, amount: u64)]
pub struct DepositUSDC<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Customer::MAX_SIZE,
    seeds = [
      CUSTOMER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub customer: Account<'info, Customer>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Promoter::MAX_SIZE,
    seeds = [
      PROMOTER_TAG,
      b"_",
      promoter_key.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  #[account(
    mut,
    constraint = customer_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = customer_ata.owner == payer.key(),
  )]
  pub customer_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = treasury_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = treasury_ata.owner == TREASURY.parse::<Pubkey>().unwrap(),
  )]
  pub treasury_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = promoter_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = promoter_pda_ata.owner == promoter.key(),
  )]
  pub promoter_pda_ata: Account<'info, TokenAccount>,
  pub asset_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(promoter_key: Pubkey, amount: u64)]
pub struct DepositUSDT<'info> {
  #[account(mut)]
  pub store: Account<'info, Store>,
  #[account(mut)]
  pub payer: Signer<'info>,
  #[account(mut)]
  pub epoc: Account<'info, Epoc>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Customer::MAX_SIZE,
    seeds = [
      CUSTOMER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub customer: Account<'info, Customer>,
  #[account(
    init_if_needed,
    payer = payer,
    space = 8 + Promoter::MAX_SIZE,
    seeds = [
      PROMOTER_TAG,
      b"_",
      promoter_key.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  #[account(
    mut,
    constraint = customer_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = customer_ata.owner == payer.key(),
  )]
  pub customer_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = treasury_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = treasury_ata.owner == TREASURY.parse::<Pubkey>().unwrap(),
  )]
  pub treasury_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = promoter_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = promoter_pda_ata.owner == promoter.key(),
  )]
  pub promoter_pda_ata: Account<'info, TokenAccount>,
  pub asset_program: Program<'info, Token>,
  pub system_program: Program<'info, System>,
}
