use anchor_lang::prelude::*;

#[error_code]
pub enum Store {
  #[msg("Unauthorized")]
  Unauthorized,
  #[msg("Store already enabled")]
  StoreEnabled,
  #[msg("Store already disabled")]
  StoreDisabled,
  #[msg("Store not enabled")]
  StoreNotEnabled,
  #[msg("Store min cap larger than max cap")]
  StoreMinCapTooLarge,
  #[msg("Store min cap not reached")]
  StoreMinCapNotReached,
  #[msg("Store max cap exceeded")]
  StoreMaxCapExceeded,
  #[msg("Store main promoter fee too large")]
  StoreMainPromoterRewardTooLarge,
  #[msg("Store secondary promoter fee too large")]
  StoreSecondaryPromoterRewardTooLarge,
  #[msg("Epoc supply is too small")]
  EpocSupplyTooSmall,
  #[msg("Epoc already enabled")]
  EpocEnabled,
  #[msg("Epoc already disabled")]
  EpocDisabled,
  #[msg("Epoc not enabled")]
  EpocNotEnabled,
  #[msg("Epoc total supply exceeded")]
  EpocSupplyExceeded,
  #[msg("Inactive epoc account")]
  InactiveEpoc,
  #[msg("Wrong price feed account")]
  WrongPriceFeedId,
  #[msg("Wrong stablecoin account")]
  WrongStablecoin,
  #[msg("Wrong treasury account")]
  WrongTreasury,
  #[msg("Oracle price is down")]
  PriceIsDown,
  #[msg("Promoter no funds")]
  PromoterNoFunds,
}