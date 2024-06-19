use anchor_lang::prelude::*;

#[account]
pub struct Customer {
  asset_amount: u128,
}

impl Customer {
  pub const MAX_SIZE: usize = 16;

  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.asset_amount = 0;

    Ok(())
  }

  pub fn set_asset_amount(
    &mut self,
    asset_amount: u128,
  ) -> Result<()> {
    self.asset_amount += asset_amount;

    Ok(())
  }

  pub fn get_asset_amount(
    &mut self,
  ) -> u128 {
    self.asset_amount
  }
}