
use std::process::exit;
use super::clap::{App,ArgMatches,Arg};
use std::thread;
use std::cell::RefCell;


thread_local!{
    static SLEEP: RefCell<usize> = RefCell::new(16);
    static COUNT: RefCell<u64> = RefCell::new(100);
}



#[derive(Copy,Clone,Debug)]
pub enum CliDo {
    NativeTiming,
    Bench
}

///Read Args from CLI
pub fn get_args<'a>() -> ArgMatches<'a> {
    App::new("Benchmark Consistent Time")
        .author("William Cody Laeder, codylaeder@gmail.com")
        .version("0.0.1")
        .about("Offers Functionality to test Consistent Time")
        .arg(Arg::with_name("check")
             .short("c")
             .long("check")
             .takes_value(false)
             .help("Checks if CPU clock is consistent"))
        .arg(Arg::with_name("bench")
             .short("b")
             .long("benchmark")
             .takes_value(false)
             .conflicts_with("check")
             .help("Run benmarks"))
        .arg(Arg::with_name("iter")
             .short("i")
             .long("iteration")
             .takes_value(true)
             .multiple(false)
             .help("How many times to run each test"))
        .arg(Arg::with_name("sleep")
             .short("s")
             .long("sleep")
             .takes_value(true)
             .multiple(false)
             .help("How long to sleep between iterations. To polute branch predictor cache"))
        .get_matches()
}

///Converto to Enum to tell us functionality
pub fn to_val<'a>(x: &ArgMatches<'a>) -> CliDo {
    if x.is_present("check") {
        return CliDo::NativeTiming;
    }
    if x.is_present("bench") {
        return CliDo::Bench;
    }
    if let Some(i) = x.value_of("iter") {
        let val = match u64::from_str_radix(i,10) {
            Ok(x) => x,
            Err(e) => {
                println!("Could not parse the valued passed by");
                println!("-i or --iteration flag");
                println!("Recieved value: {}",i);
                println!("Need a non-negative whole number");
                println!("Fatal Error: {:?}",e);
                exit(0);
            }
        };
        COUNT.with(|cell|{
            *cell.borrow_mut() = val;
        });
    }
    if let Some(i) = x.value_of("sleep") {
        let val = match usize::from_str_radix(i,10) {
            Ok(x) => x,
            Err(e) => {
                println!("Could not parse the valued passed by");
                println!("-s or --sleep flag");
                println!("Recieved value: {}",i);
                println!("Need a non-negative whole number");
                println!("Fatal Error: {:?}",e);
                exit(0);
            }
        };
        SLEEP.with(|cell|{
            *cell.borrow_mut() = val;
        });

    }
    println!("I did not understand CLI arguments");
    println!("Consider running -h to see help");
    exit(0);
}


pub fn get_sleep() -> usize {
    SLEEP.with(|cell| {
        *cell.borrow()
    })
}

pub fn get_count() -> u64 {
    COUNT.with(|cell| {
        *cell.borrow()
    })
}
