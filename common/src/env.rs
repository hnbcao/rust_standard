use std::env;
use std::env::VarError;
const ENABLE_MIGRATION: Option<&'static str> = option_env!("ENABLE_MIGRATION");

pub fn enable_migration() -> bool {
    match ENABLE_MIGRATION {
        None => false,
        Some(enable) => enable.parse::<bool>().unwrap_or(false),
    }
}
