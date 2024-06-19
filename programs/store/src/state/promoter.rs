use anchor_lang::prelude::*;

#[account]
pub struct Promoter {
  first_fee: u64,
  second_fee: u64,

  sol_amount: u64,
  usdt_amount: u64,
  usdc_amount: u64,
  asset_amount: u128,

  enabled: bool,
}

impl Promoter {
  pub const MAX_SIZE: usize = (5 * 8) + 16 + 1;

  pub fn init(
    &mut self,
    main_promoter_fee: u64,
    secondary_promoter_fee: u64,
  ) -> Result<()> {
    self.first_fee = main_promoter_fee;
    self.second_fee = secondary_promoter_fee;

    self.sol_amount = 0;
    self.usdt_amount = 0;
    self.usdc_amount = 0;
    self.asset_amount = 0;

    self.enabled = true;

    Ok(())
  }

  pub fn set_fee(
    &mut self,
    first_fee: u64,
    second_fee: u64,
  ) -> Result<()> {
    self.first_fee = first_fee;
    self.second_fee = second_fee;

    Ok(())
  }

  pub fn set_sol_fee_amount(
    &mut self,
    fee_amount: u64,
  ) -> Result<()> {
    self.sol_amount += fee_amount;

    Ok(())
  }

  pub fn reset_sol_fee_amount(
    &mut self,
  ) -> Result<()> {
    self.sol_amount = 0;

    Ok(())
  }

  pub fn set_usdt_amount(
    &mut self,
    usdt_amount: u64,
  ) -> Result<()> {
    self.usdt_amount += usdt_amount;

    Ok(())
  }

  pub fn reset_usdt_amount(
    &mut self,
  ) -> Result<()> {
    self.usdt_amount = 0;

    Ok(())
  }

  pub fn set_usdc_amount(
    &mut self,
    usdc_amount: u64,
  ) -> Result<()> {
    self.usdc_amount += usdc_amount;

    Ok(())
  }

  pub fn reset_usdc_amount(
    &mut self,
  ) -> Result<()> {
    self.usdc_amount = 0;

    Ok(())
  }

  pub fn set_asset_amount(
    &mut self,
    asset_amount: u128,
  ) -> Result<()> {
    self.asset_amount += asset_amount;

    Ok(())
  }

  pub fn get_fee(
    &mut self,
  ) -> (u64, u64) {
    (self.first_fee, self.second_fee)
  }

  pub fn get_sol_fee_amount(
    &mut self,
  ) -> u64 {
    self.sol_amount
  }

  pub fn get_usdt_amount(
    &mut self,
  ) -> u64 {
    self.usdt_amount
  }

  pub fn get_usdc_amount(
    &mut self,
  ) -> u64 {
    self.usdc_amount
  }

  pub fn get_asset_amount(
    &mut self,
  ) -> u128 {
    self.asset_amount
  }

  pub fn enable(
    &mut self,
  ) -> Result<()> {
    self.enabled = true;

    Ok(())
  }

  pub fn disable(
    &mut self,
  ) -> Result<()> {
    self.enabled = false;

    Ok(())
  }
}