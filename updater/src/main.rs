mod local_fs_impl;
use clap::Parser;

use crate::local_fs_impl::FSKeyProvider;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output of the installer 
    #[arg(long)]
    installer_to_write: String,

    /// Keys directory
    #[arg(long)]
    key_directory: String,

    /// Current version
    #[arg(long)]
    version: String,

    /// Installation serial
    #[arg(long)]
    serial: String,
}

fn main() {
    let args = Args::parse();
    let key_provider = FSKeyProvider::new(&args.key_directory).unwrap();
}
