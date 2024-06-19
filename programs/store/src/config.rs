use anchor_lang::prelude::*;

pub const MAX_CAP: u64       = 1000_000_000_000_000; 
pub const MIN_CAP: u64       = 100_000_000_000;
pub const FIRST_INTEREST: u64       = 50_000_000;
pub const SECOND_INTEREST: u64      = 50_000_000;

pub const EPOC_TAG: &[u8]           = b"EPOC";
pub const CUSTOMER_TAG: &[u8]       = b"CUSTOMER";
pub const PROMOTER_TAG: &[u8]       = b"PROMOTER";
pub const EMPTY_PROMOTER: &str      = "9XwXqTuy86VKLLhzEU5ktSWT4efGPnFFWxUmFzUywsqy";
pub const TREASURY: &str            = "2vrYa73jwsAvkdtPYVaeCbd9yGu9TvVXZgNwyP8nXUY6";

pub const STALENESS_THRESHOLD: u64  = 60;
pub const SOL_USD_PRICEFEED: &str   = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";

pub const PRECISION: u32            = 9;
pub const STABLE_PRECISION: u32     = 3;
pub const USDT: &str                = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
pub const USDC: &str                = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";

const ADMINS: &[&str] = &["D9eS7qChQzpuiF9M3jg3VebZFwSnbnEfDUgb4xcr4Suf", "3LByBjJxGECZtTVPH6WKH2rsbCYGeuKiJUp95NztncYN", "BRm2knMo3Dob5xY6Vh1rYSZfGgFf2ThUk5no5vJGtpoQ"];

pub fn only_admin(address: Pubkey) -> bool {
  return  ADMINS.contains(&address.to_string().as_str());
}
