use clap::Parser;
use std::fmt::{Debug, Display};
use std::io::{BufReader, BufWriter, Read, Write};

use base116::decode::{DecodeConfig, decode_bytes_with};
use base116::encode::{EncodeConfig, encode_to_bytes_with};

mod cli;

fn main() {
    let args = cli::CliArgs::parse();

    match args.command {
        cli::EncodeOption::Encode(ref ioargs) => {
            let encode_config = {
                let mut ec = EncodeConfig::new();
                ec.add_wrapper = ioargs.with_wrapper;
                ec
            };

            encode(ioargs, encode_config);
        }
        cli::EncodeOption::Decode(ref ioargs) => {
            let decode_config = {
                let mut ec = DecodeConfig::new();
                ec.require_wrapper = ioargs.with_wrapper;
                ec.relaxed = true;
                ec
            };

            decode(ioargs, decode_config);
        }
    }
}

fn expect<T, E: Debug>(result: Result<T, E>, msg: impl Display) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("error: {}", msg);
        panic!("error: {}: {:?}", msg, e);
    })
}

fn encode(ioargs: &cli::IOArgs, config: EncodeConfig) {
    let infile = BufReader::new(ioargs.r#in.clone());
    let mut outfile = BufWriter::new(ioargs.out.clone());
    encode_to_bytes_with(
        infile.bytes().map(|b| expect(b, "could not read input")),
        config,
    )
    .for_each(|b| {
        expect(outfile.write_all(&[b]), "could not write to standard output");
    });
}

fn decode(ioargs: &cli::IOArgs, config: DecodeConfig) {
    let infile = BufReader::new(ioargs.r#in.clone());
    let mut outfile = BufWriter::new(ioargs.out.clone());
    decode_bytes_with(
        infile.bytes().map(|b| expect(b, "could not read input")),
        config,
    )
    .for_each(|b| match b {
        Ok(b) => {
            expect(outfile.write_all(&[b]), "could not write to given output");
        }
        Err(e) => {
            eprintln!("input is not valid base-116 data: {}", e);
        }
    });
}
