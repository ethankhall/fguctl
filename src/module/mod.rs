mod cli;
mod inputs;
mod output;

pub use cli::ModuleSubCommand;

use std::sync::atomic::AtomicU8;
const SPELL_ID_COUNTER: AtomicU8 = AtomicU8::new(1);
const TABLE_ID_COUNTER: AtomicU8 = AtomicU8::new(1);
