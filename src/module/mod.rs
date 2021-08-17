mod cli;
mod inputs;
mod output;

use lazy_static::lazy_static;

pub use cli::ModuleSubCommand;

use std::sync::atomic::AtomicU8;

lazy_static! {
    static ref SPELL_ID_COUNTER: AtomicU8 = AtomicU8::new(1);
    static ref TABLE_ID_COUNTER: AtomicU8 = AtomicU8::new(1);
}
