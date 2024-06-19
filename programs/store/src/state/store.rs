use anchor_lang::prelude::*;
use crate::errors;
use crate::config::{ MAX_CAP, MIN_CAP, FIRST_INTEREST, SECOND_INTEREST };

#[derive(Clone, PartialEq, AnchorDeserialize, AnchorSerialize)]
pub enum Status {
  None,
  Enabled,
  Disabled,
}

#[account]
pub struct Store {
  max_cap: u64,
  min_cap: u64,
  first_fee: u64,
  second_fee: u64,
  total_sold: u128,
  epoc: i16,
  status: Status,
  enabled: bool,
}

impl Store {
  pub const MAX_SIZE: usize = (4 * 8) + 16 + 2 + (32 + 1) * 1;

  pub fn init(
    &mut self,
  ) -> Result<()> {
    self.epoc = -1;
    self.max_cap = MAX_CAP;
    self.min_cap = MIN_CAP;
    self.first_fee = FIRST_INTEREST;
    self.second_fee = SECOND_INTEREST;
    self.total_sold = 0;
    self.status = Status::None;
    self.enabled = true;

    Ok(())
  }

  pub fn set_cap(
    &mut self,
    max_cap: u64,
    min_cap: u64,
  ) -> Result<()> {
    if max_cap < min_cap {
      return err!(errors::Store::StoreMinCapTooLarge);
    }

    self.max_cap = max_cap;
    self.min_cap = min_cap;

    Ok(())
  }

  pub fn set_fee(
    &mut self,
    first_fee: u64,
    second_fee: u64,
  ) -> Result<()> {
    if first_fee > 1000_000_000 {
      return err!(errors::Store::StoreMainPromoterRewardTooLarge);
    }

    if second_fee > 1000_000_000 {
      return err!(errors::Store::StoreSecondaryPromoterRewardTooLarge);
    }

    self.first_fee = first_fee;
    self.second_fee = second_fee;

    Ok(())
  }

  pub fn set_enable(
    &mut self,
  ) -> Result<()> {
    if self.status == Status::Enabled || self.status == Status::Disabled {
      return err!(errors::Store::StoreEnabled);
    }

    self.status = Status::Enabled;

    Ok(())
  }

  pub fn set_disable(
    &mut self,
  ) -> Result<()> {
    if self.status != Status::Enabled {
      return err!(errors::Store::StoreDisabled);
    }

    self.status = Status::Disabled;

    Ok(())
  }

  pub fn set_epoc(
    &mut self,
    epoc: i16,
  ) -> Result<()> {
    self.epoc = epoc;

    Ok(())
  }

  pub fn set_total_sold(
    &mut self,
    total_sold: u128,
  ) -> Result<()> {
    self.total_sold += total_sold;

    Ok(())
  }

  pub fn get_epoc(
    &self,
  ) -> i16 {
    self.epoc
  }

  pub fn get_max_cap(
    &self,
  ) -> u128 {
    u128::from(self.max_cap)
  }

  pub fn get_min_cap(
    &self,
  ) -> u128 {
    u128::from(self.min_cap)
  }

  pub fn get_total_sold(
    &self,
  ) -> u128 {
    self.total_sold
  }

  pub fn get_fee(
    &mut self,
  ) -> (u64, u64) {
    (self.first_fee, self.second_fee)
  }

  pub fn is_enabled(
    &self,
  ) -> bool {
    self.status == Status::Enabled
  }
}
