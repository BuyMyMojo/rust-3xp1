use std::ops::Mul;
use std::process::exit;
use tracing::{Level, instrument, event};
use tracing_subscriber;
use clap::Parser;

/// Simple tool to bruteforce 3x+1 for a second loop. Only takes positive numbers
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Weather to output current number each time
    #[clap(short, long)]
    verbose: bool,

    /// Weather to output current number every step of 3x+1
    #[clap(short, long)]
    double_verbose: bool,

    /// Number of times to try
    #[clap(short, long, default_value_t = 50000)]
    count: u128,

    /// Number to start at
    #[clap(short, long, default_value_t = 195147905179352825856)]
    start: u128,
}

#[instrument]
fn main() {
    tracing_subscriber::fmt::init();
    let args = Args::parse();

    let max_num = args.start + args.count;

    event!(Level::INFO, "Starting 3x+1 process");
    event!(Level::INFO, "Starting number: {}", args.start);
    event!(Level::INFO, "Amount of numbers to try: {}", args.count);

    if args.verbose & !args.double_verbose{
        for current_num in args.start..max_num{
            event!(Level::INFO, "Processing {}", current_num);
            let (smallest_num, largest_num) = txpo(current_num);
            event!(Level::INFO, "Loop found! Smallest number: {}, largest number: {}", smallest_num, largest_num);
        }
    } else if args.double_verbose {
        for current_num in args.start..max_num{
            event!(Level::INFO, "Processing {}", current_num);
            let (smallest_num, largest_num) = verbose_txpo(current_num);
            event!(Level::INFO, "Loop found! Smallest number: {}, largest number: {}", smallest_num, largest_num);
        }
    } else {
        for current_num in args.start..max_num{
            let (_smallest_num, _largest_num) = txpo(current_num);
        }
    }

    event!(Level::INFO, "Finished trying {} numbers", args.count);
}

#[instrument]
fn txpo(mut current_num: u128) -> (u128, u128) {
    let mut largest_num = current_num;
    let mut smallest_num = current_num;
    let mut smallest_num_count = 1;
    while smallest_num_count != 2{

        // if odd
        if current_num % 2 != 0 {
            if current_num < smallest_num{
                smallest_num = current_num;
            }

            if current_num > largest_num{
                largest_num = current_num;
            }

            current_num = current_num.mul(3) + 1;  // For some reason using .mul() is faster then * but .add() is slightly slower then +. it was only a few ms slower to do all 50K but on an extremely large scale it matters
        } else {
            if current_num < smallest_num{
                smallest_num = current_num;
            }

            if current_num > largest_num{
                largest_num = current_num;
            }

            current_num = current_num >> 1;  // Divides by 2 a bit faster
        }

        if current_num == smallest_num {
            smallest_num_count = smallest_num_count + 1;
        }
    }

    if smallest_num != 1{
        event!(Level::ERROR, "Found new loop!");
        exit(1)
    }

    return(smallest_num, largest_num);
}

#[instrument]
fn verbose_txpo(mut current_num: u128) -> (u128, u128) {
    let mut largest_num = current_num;
    let mut smallest_num = current_num;
    let mut smallest_num_count = 1;
    while smallest_num_count != 2{

        // if odd
        if current_num % 2 != 0 {
            if current_num < smallest_num{
                smallest_num = current_num;
            }

            if current_num > largest_num{
                largest_num = current_num;
            }

            event!(Level::INFO, "{} is odd. Smallest number: {}. Largest number: {}", current_num, smallest_num, largest_num);

            current_num = current_num.mul(3) + 1;  // For some reason using .mul() is faster then * but .add() is slightly slower then +. it was only a few ms slower to do all 50K but on an extremely large scale it matters
        } else {
            if current_num < smallest_num{
                smallest_num = current_num;
            }

            if current_num > largest_num{
                largest_num = current_num;
            }

            event!(Level::INFO, "{} is even. Smallest number: {}. Largest number: {}", current_num, smallest_num, largest_num);

            current_num = current_num >> 1;  // Divides by 2 a bit faster
        }

        if current_num == smallest_num {
            smallest_num_count = smallest_num_count + 1;
        }
    }

    if smallest_num != 1{
        event!(Level::ERROR, "Found new loop!");
        exit(1)
    }

    return(smallest_num, largest_num);
}