use libsodium_sys as libsodium;

macro_rules! sodium_call {
    ($name:ident, $($args:expr),*) => { ::rosenpass_util::attempt!({
        anyhow::ensure!(unsafe{libsodium::$name($($args),*)} > -1,
            "Error in libsodium's {}.", stringify!($name));
        Ok(())
    })};
    ($name:ident) => { sodium_call!($name, ) };
}

#[inline]
pub fn init() -> anyhow::Result<()> {
    log::trace!("initializing libsodium");
    sodium_call!(sodium_init)
}

pub mod aead;
pub mod alloc;
pub mod hash;
pub mod helpers;
