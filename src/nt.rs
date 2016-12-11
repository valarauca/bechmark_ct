
use super::amd64_timer::ticks as tick;
use super::time::precise_time_ns as now;
use std::process::exit;
//Attempts to read CPU clock timing
pub fn get_cpu_timing() {
    let mut count = 0usize;
    let mut running_avg = 0u64;
    //do this a lot
    while count < 1000000 {
        let mut ct = 0u64;
        let mut delta = 0u64;
        let ticks = tick();
        while delta < 10000 {
            ct = tick();
            delta = ct-ticks;
        }
        //thread got bumped from execution
        //bad test
        if delta > 100000 {
            continue;
        }
        running_avg += delta;
        count += 1;
    }
    let mut avg = (running_avg/1000000u64) as f64;
    let error = ((avg - 10000f64)/100f64).abs();
    println!("Targetting 1,000,000 cycle wait");
    println!("Value is always be a bit higher b/c branching");
    println!("Average: {:.*} [cycles]", 2, avg);
    println!("Percent Error: {:.*}[%]", 2, error);
}
