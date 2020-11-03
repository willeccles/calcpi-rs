use std::env;
use std::thread;

fn calcinrange(start: u64, end: u64, id: u64) -> f64 {
    let mut cycle = start;
    let s: f64 = 2.0 + (start as f64 * 2.0);
    let mut den = s * (s + 1.0) * (s + 2.0);
    let mut v_6s2 = 6.0 * s * s;
    let mut v_24s = 24.0 * s;
    let mut pi: f64 = 0.0;

    let sign = if start % 2 == 0 { 1.0 } else { -1.0 };

    while cycle < end {
        let mut den2 = den;
        v_6s2 += v_24s + 24.0;
        v_24s += 48.0;
        den += v_6s2;
        den2 *= den; // This mult could also be removed
        pi += v_6s2 / den2; // Only one division (per two cycles)!
        v_6s2 += v_24s + 24.0;
        v_24s += 48.0;
        den += v_6s2;
        cycle += 2;
    }

    if cycle == end {
        pi += 1.0 / den;
    }

    println!("Thread {} done.", id);
    return sign * pi * 4.0;
}

fn main() {
    let argv: Vec<String> = env::args().collect();

    let tcount: u64 = if argv.len() < 2 {
        num_cpus::get() as u64
    } else {
        argv[1].parse().unwrap()
    };

    let itercount: u64 = if argv.len() < 3 {
        10_000_000_000u64
    } else {
        argv[2].parse().unwrap()
    };

    println!("Calculating Pi using {} iterations on {} threads.", itercount, tcount);

    let mut threads = vec![];

    for tid in 0..tcount {
        let tstart: u64 = tid * (itercount / tcount);
        let mut tend: u64 = tstart + (itercount / tcount) - 1;
        if tid == tcount - 1 {
            tend += itercount % tcount;
        }
        threads.push(thread::spawn(move || -> f64 {
            calcinrange(tstart, tend, tid)
        }));
        println!("Thread {}: {} to {}", tid, tstart, tend);
    }

    let mut result: f64 = 3.0;
    for t in threads {
        result += t.join().unwrap();
    }

    println!("All threads complete.");

    println!("Result: {:.}", result);
}
