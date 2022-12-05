use std::process::{Output, Command};

use encoding::{all::GBK, Encoding};


fn command_wrapper(cmd: &String) -> Output {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg(cmd).output().unwrap()
    } else {
        Command::new("sh").arg("-c").arg(cmd).output().unwrap()
    }
}
fn encoding_wrapper(err_vec: &Vec<u8>) -> String {
    if cfg!(target_os = "windows") {
        GBK.decode(err_vec, encoding::DecoderTrap::Ignore).unwrap()
    } else {
        String::from_utf8_lossy(err_vec).to_string()
    }
}