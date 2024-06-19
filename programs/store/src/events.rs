use anchor_lang::prelude::*;

#[event]
pub struct DepositWithSolEvent {
  pub epoc: i16,
  pub customer: Pubkey,
  pub promoter: Pubkey,
  pub sol_amount: u64,
  pub asset_amount: u128,
}

#[event]
pub struct DepositWithUsdtEvent {
  pub epoc: i16,
  pub customer: Pubkey,
  pub promoter: Pubkey,
  pub usdt_amount: u64,
  pub asset_amount: u128,
}

#[event]
pub struct DepositWithUsdcEvent {
  pub epoc: i16,
  pub customer: Pubkey,
  pub promoter: Pubkey,
  pub usdc_amount: u64,
  pub asset_amount: u128,
}

#[event]
pub struct WithdrawSolEvent {
  pub promoter: Pubkey,
  pub amount: u64,
}

#[event]
pub struct WithdrawUsdtEvent {
  pub promoter: Pubkey,
  pub amount: u64,
}

#[event]
pub struct WithdrawUsdcEvent {
  pub promoter: Pubkey,
  pub amount: u64,
}
