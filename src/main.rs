
extern crate amd64_timer;
extern crate consistenttime;
extern crate clap;
extern crate time;

mod cli;
use cli::{to_val,get_args,CliDo};

mod nt;
use nt::get_cpu_timing;

mod bench;
use bench::bench;


fn main() {
    
    //handle CLI
    let m = get_args();
    let d = to_val(&m);
    
    match d {
        CliDo::NativeTiming => get_cpu_timing(),
        CliDo::Bench => bench()
    };
}
