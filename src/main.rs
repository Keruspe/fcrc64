use anyhow::Result;
use atty::Stream;
use clap::{crate_version, App, Arg};
use crc64fast::Digest;

use std::{
    fs::File,
    io::{self, Read},
};

const BUF_SIZE: usize = 8192;

fn main() -> Result<()> {
    let matches = App::new("fcrc64")
        .version(crate_version!())
        .author("Marc-Antoine Perennou <Marc-Antoine@Perennou.com>")
        .arg(Arg::with_name("INPUT").help("Sets the input file to use"))
        .get_matches();
    let crc64 = if let Some(input) = matches.value_of("INPUT") {
        let mut input = File::open(input)?;
        fcrc64(&mut input)
    } else if atty::is(Stream::Stdin) {
        anyhow::bail!("Couldn't read from stdin: is a tty");
    } else {
        fcrc64(&mut io::stdin())
    }?;
    println!("{}", crc64);
    Ok(())
}

fn fcrc64(input: &mut dyn Read) -> Result<u64> {
    let mut c = Digest::new();
    let mut buffer = [0u8; BUF_SIZE];
    loop {
        let len = input.read(&mut buffer[..])?;
        if len == 0 {
            break;
        }
        c.write(&buffer[..len]);
    }
    Ok(c.sum64())
}
