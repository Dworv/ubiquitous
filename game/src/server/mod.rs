mod dev;
mod fs;
mod security;
pub mod utils;

pub use dev::Device;
pub use fs::{File, FileContent, FileSystem};
pub use security::{AttackInfo, AttackKind, SecurityState, ServerSecurity, SkillArray};
use std::fmt::Display;

#[derive(Debug)]
pub struct Server {
    pub name: String,
    pub fs: FileSystem,
    pub sec: ServerSecurity,
    pub devices: Vec<Device>,
}
