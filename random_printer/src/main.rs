use clap::Parser;
use log::info;
use rand::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    iteration: u64,
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    loop {
        info!(
            "Every {} seconds I will print a random number: {}",
            args.iteration,
            rand::thread_rng().gen_range(0..=100)
        );
        std::thread::sleep(std::time::Duration::from_secs(args.iteration));
    }
}
