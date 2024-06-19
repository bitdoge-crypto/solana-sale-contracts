use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer as SplTransfer};
use crate::config::{ USDC, USDT, PROMOTER_TAG };

use crate::events;
use crate::errors;
use crate::state::promoter::*;

pub fn init_promoter(
  ctx: Context<InitPromoter>,
  first_fee: u64,
  second_fee: u64,
) -> Result<()> {
  let promoter = &mut ctx.accounts.promoter;
  promoter.init(first_fee, second_fee)
}

pub fn set_promoter_fee(
  ctx: Context<SetPromoterReward>,
  first_fee: u64,
  second_fee: u64,
) -> Result<()> {
  let promoter = &mut ctx.accounts.promoter;
  promoter.set_fee(first_fee, second_fee)
}

pub fn enable_promoter(
  ctx: Context<SetPromoterEnabled>,
) -> Result<()> {
  let promoter = &mut ctx.accounts.promoter;
  promoter.enable()
}

pub fn disable_promoter(
  ctx: Context<SetPromoterDisabled>,
) -> Result<()> {
  let promoter = &mut ctx.accounts.promoter;
  promoter.disable()
}

pub fn withdraw_sol(
  ctx: Context<Withdraw>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let promoter = &mut ctx.accounts.promoter;
  
  let sol_fee = promoter.get_sol_fee_amount();
  if sol_fee > 0 {
    promoter.reset_sol_fee_amount().unwrap();

    promoter.sub_lamports(sol_fee).unwrap();
    payer.add_lamports(sol_fee).unwrap();

    emit!(events::WithdrawSolEvent {
      promoter: payer.key(),
      amount: sol_fee,
    });
  }

  Ok(())
}

pub fn withdraw_usdc(
  ctx: Context<WithdrawUSDC>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let promoter = &mut ctx.accounts.promoter;
  
  let promoter_ata = &ctx.accounts.promoter_ata;
  let promoter_pda_ata = &ctx.accounts.promoter_pda_ata;
  let program = &ctx.accounts.asset_program;

  let amount = promoter.get_usdc_amount();
  if amount == 0 {
    return err!(errors::Store::PromoterNoFunds);
  }

  promoter.reset_usdc_amount().unwrap();

  let payer_key = payer.key();
  let bump = &[ctx.bumps.promoter];
  let seeds: &[&[u8]] = &[PROMOTER_TAG, b"_", payer_key.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: promoter_pda_ata.to_account_info(),
    to: promoter_ata.to_account_info(),
    authority: promoter.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::WithdrawUsdcEvent {
    promoter: payer.key(),
    amount: amount,
  });

  Ok(())
}

pub fn withdraw_usdt(
  ctx: Context<WithdrawUSDT>,
) -> Result<()> {
  let payer = &mut ctx.accounts.payer;
  let promoter = &mut ctx.accounts.promoter;
  
  let promoter_ata = &ctx.accounts.promoter_ata;
  let promoter_pda_ata = &ctx.accounts.promoter_pda_ata;
  let program = &ctx.accounts.asset_program;

  let amount = promoter.get_usdt_amount();
  if amount == 0 {
    return err!(errors::Store::PromoterNoFunds);
  }

  promoter.reset_usdt_amount().unwrap();

  let payer_key = payer.key();
  let bump = &[ctx.bumps.promoter];
  let seeds: &[&[u8]] = &[PROMOTER_TAG, b"_", payer_key.as_ref(), bump];
  let signer_seeds = &[&seeds[..]];

  let cpi_accounts = SplTransfer {
    from: promoter_pda_ata.to_account_info(),
    to: promoter_ata.to_account_info(),
    authority: promoter.to_account_info(),
  };
  let ctx = CpiContext::new_with_signer(program.to_account_info(), cpi_accounts, signer_seeds);
  token::transfer(ctx, amount).unwrap();

  emit!(events::WithdrawUsdtEvent {
    promoter: payer.key(),
    amount: amount,
  });

  Ok(())
}

#[derive(Accounts)]
#[instruction(promoter_key: Pubkey)]
pub struct InitPromoter<'info> {
  #[account(
    init,
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
  #[account(mut)]
  pub payer: Signer<'info>,
  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetPromoterReward<'info> {
  #[account(mut)]
  pub promoter: Account<'info, Promoter>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPromoterEnabled<'info> {
  #[account(mut)]
  pub promoter: Account<'info, Promoter>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct SetPromoterDisabled<'info> {
  #[account(mut)]
  pub promoter: Account<'info, Promoter>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
  #[account(
    mut,
    seeds = [
      PROMOTER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawUSDC<'info> {
  #[account(
    mut,
    seeds = [
      PROMOTER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  #[account(
    mut,
    constraint = promoter_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = promoter_ata.owner == payer.key(),
  )]
  pub promoter_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = promoter_pda_ata.mint == USDC.parse::<Pubkey>().unwrap(),
    constraint = promoter_pda_ata.owner == promoter.key(),
  )]
  pub promoter_pda_ata: Account<'info, TokenAccount>,
  pub asset_program: Program<'info, Token>,
  #[account(mut)]
  pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct WithdrawUSDT<'info> {
  #[account(
    mut,
    seeds = [
      PROMOTER_TAG,
      b"_",
      payer.key().as_ref()
    ],
    bump
  )]
  pub promoter: Account<'info, Promoter>,
  #[account(
    mut,
    constraint = promoter_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = promoter_ata.owner == payer.key(),
  )]
  pub promoter_ata: Account<'info, TokenAccount>,
  #[account(
    mut,
    constraint = promoter_pda_ata.mint == USDT.parse::<Pubkey>().unwrap(),
    constraint = promoter_pda_ata.owner == promoter.key(),
  )]
  pub promoter_pda_ata: Account<'info, TokenAccount>,
  pub asset_program: Program<'info, Token>,
  #[account(mut)]
  pub payer: Signer<'info>,
}