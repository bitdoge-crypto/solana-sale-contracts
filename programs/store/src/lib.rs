use anchor_lang::prelude::*;
use instructions::*;
pub mod config;
pub mod errors;
pub mod events;
pub mod state;
pub mod instructions;

declare_id!("54jF2wtHopafEnsVUbCe1c1Krfcm8jfRjcgrcZ4axsUk");

#[program]
pub mod store {
  use super::*;

  pub fn init(
    ctx: Context<InitStore>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::store::init_store(ctx)
  }

  pub fn set_store_cap(
    ctx: Context<SetStoreCap>,
    max_cap: u64,
    min_cap: u64,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::store::set_store_cap(ctx, max_cap, min_cap)
  }

  pub fn set_store_promoter_fee(
    ctx: Context<SetStoreReward>,
    first_fee: u64,
    second_fee: u64,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::store::set_store_fee(ctx, first_fee, second_fee)
  }

  pub fn enable_store(
    ctx: Context<SetStoreEnabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::store::enable_store(ctx)
  }

  pub fn disable_store(
    ctx: Context<SetStoreDisabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::store::disable_store(ctx)
  }

  pub fn deposit_with_sol(
    ctx: Context<Deposit>,
    promoter_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::store::deposit_with_sol(ctx, promoter_key, amount)
  }

  pub fn deposit_with_usdc(
    ctx: Context<DepositUSDC>,
    promoter_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::store::deposit_with_usdc(ctx, promoter_key, amount)
  }

  pub fn deposit_with_usdt(
    ctx: Context<DepositUSDT>,
    promoter_key: Pubkey,
    amount: u64,
  ) -> Result<()> {
    instructions::store::deposit_with_usdt(ctx, promoter_key, amount)
  }

  pub fn init_epoc(
    ctx: Context<InitEpoc>,
    id: i16,
    price: u64,
    total_supply: u128,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::epoc::init_epoc(ctx, id, price, total_supply)
  }

  pub fn set_epoc_price(
    ctx: Context<SetEpocPrice>,
    price: u64,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::epoc::set_epoc_price(ctx, price)
  }

  pub fn set_epoc_supply(
    ctx: Context<SetEpocSupply>,
    total_supply: u128,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::epoc::set_epoc_supply(ctx, total_supply)
  }

  pub fn enable_epoc(
    ctx: Context<SetEpocEnabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::epoc::enable_epoc(ctx)
  }

  pub fn disable_epoc(
    ctx: Context<SetEpocDisabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::epoc::disable_epoc(ctx)
  }

  pub fn init_promoter(
    ctx: Context<InitPromoter>,
    _promoter_key: Pubkey,
    first_fee: u64,
    second_fee: u64,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::promoter::init_promoter(ctx, first_fee, second_fee)
  }

  pub fn set_promoter_fee(
    ctx: Context<SetPromoterReward>,
    first_fee: u64,
    second_fee: u64,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::promoter::set_promoter_fee(ctx, first_fee, second_fee)
  }

  pub fn enable_promoter(
    ctx: Context<SetPromoterEnabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::promoter::enable_promoter(ctx)
  }

  pub fn disable_promoter(
    ctx: Context<SetPromoterDisabled>,
  ) -> Result<()> {
    if !config::only_admin(ctx.accounts.payer.key()) {
      return err!(errors::Store::Unauthorized);
    }

    instructions::promoter::disable_promoter(ctx)
  }

  pub fn withdraw_sol(
    ctx: Context<Withdraw>,
  ) -> Result<()> {
    instructions::promoter::withdraw_sol(ctx)
  }

  pub fn withdraw_usdc(
    ctx: Context<WithdrawUSDC>,
  ) -> Result<()> {
    instructions::promoter::withdraw_usdc(ctx)
  }

  pub fn withdraw_usdt(
    ctx: Context<WithdrawUSDT>,
  ) -> Result<()> {
    instructions::promoter::withdraw_usdt(ctx)
  }
}
