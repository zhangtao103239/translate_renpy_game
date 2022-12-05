use std::process::{Output, Command};
use anyhow::Result;
use encoding::{all::GBK, Encoding};


fn command_wrapper(cmd: &String) -> Result<Output> {
    if cfg!(target_os = "windows") {
        Ok(Command::new("cmd").arg("/C").arg(cmd).output()?)
    } else {
        Ok(Command::new("sh").arg("-c").arg(cmd).output()?)
    }
}
fn encoding_wrapper(err_vec: &Vec<u8>) -> Result<String> {
    if cfg!(target_os = "windows") {
        Ok(GBK.decode(err_vec, encoding::DecoderTrap::Ignore).unwrap_or_default())
    } else {
        Ok(String::from_utf8_lossy(err_vec).to_string())
    }
}