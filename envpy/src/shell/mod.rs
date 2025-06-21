use std::io;
use std::path::PathBuf;

pub mod bash;

pub use bash::*;

pub(super) trait ProfileRenamer {
    fn get_path(&self) -> PathBuf;
    fn rename(&self, backup: bool) -> io::Result<()>;
    fn already_setup(&self) -> u8;
    fn setup(&self) -> i32;
}
