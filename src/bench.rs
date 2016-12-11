use super::cli::{get_sleep, get_count};
use super::consistenttime::{ct_select_u8,ct_copy_u8};
use super::amd64_timer::ticks;
use std::thread::sleep_ms;

pub fn bench() {
    let max_count = get_count();
    let sleep = get_sleep() as u32;

    let mut count = 0u64;
    let mut v1 = 0u64;
    let mut a = 0u8;
    let mut b = 0u8;
    let mut start = 0u64;
    let mut end = 0u64;
    let mut delta = 0u64;

    let mut sum = 0u64;
    while count < max_count {
        v1 = ticks();
        a = ((v1&0x0000FF00000000) >> 4) as u8;
        b = ((v1&0x00FF0000000000) >> 5) as u8;
        start = ticks();
        let _ = ct_select_u8(true,a,b);
        end = ticks();
        delta = end - start;
        count += 1;
        sum += delta;
        sleep_ms(sleep);
    }
    let avg = (sum/max_count) as f64;
    println!("Testing ct_select_u8 TRUE");
    println!("Avg Time: {:.*}[cycles]", 4,avg);

    delta = 0u64;
    sum = 0u64;
    count = 0u64;
    while count < max_count {
        v1 = ticks();
        a = ((v1&0x0000FF00000000) >> 4) as u8;
        b = ((v1&0x00FF0000000000) >> 5) as u8;
        start = ticks();
        let _ = ct_select_u8(false,a,b);
        end = ticks();
        delta = end - start;
        count += 1;
        sum += delta;
        sleep_ms(sleep);
    }
    let avg = (sum/max_count) as f64;
    println!("Testing ct_select_u8 FALSE");
    println!("Avg Time: {:.*}[cycles]", 4,avg);


    delta = 0u64;
    sum = 0u64;
    count = 0u64;
    while count < max_count {
        let mut v = Vec::with_capacity(16348);
        let mut w = Vec::with_capacity(16358);
        for _ in 0..16348 {
            v.push(255u8);
            w.push(0u8);
        }
        start = ticks();
        ct_copy_u8(true,&mut w,&v);
        end = ticks();
        delta = end - start;
        count += 1;
        sum += delta;
        sleep_ms(sleep);
    }
    let avg = sum/max_count;
    println!("Testing ct_copy_u8 TRUE");
    println!("Avg Time: {:?}[cycles]",avg);

    delta = 0u64;
    sum = 0u64;
    count = 0u64;
    while count < max_count {
        let mut v = Vec::with_capacity(16348);
        let mut w = Vec::with_capacity(16358);
        for _ in 0..16348 {
            v.push(255u8);
            w.push(0u8);
        }
        start = ticks();
        ct_copy_u8(false,&mut w,&v);
        end = ticks();
        delta = end - start;
        count += 1;
        sum += delta;
        sleep_ms(sleep);
    }
    let avg = sum/max_count;
    println!("Testing ct_copy_u8 FALSE");
    println!("Avg Time: {:?}[cycles]",avg);

}
