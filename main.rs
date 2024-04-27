use std::io;
use std::sync::mpsc;
use std::thread;
//use math::round;
#[warn(clippy::pedantic)]
fn main() {
    println!("Do you want to check if a number is prime, find all the primes under a specific number, find a specified number of prime numbers, or check if a number is a Mersenne prime? (For the first, enter 1, for the second, enter 2, for the third, enter 3, and for the fourth, enter 4.)");
    let mut allorone = String::new();
    io::stdin()
        .read_line(&mut allorone)
        .expect("Failed to read line.");
    let allorone: u8 = allorone
        .trim()
        .parse()
        .expect("Please type a number between 1 and 3 inclusive!");
    if allorone == 1 {
        is_prime();
    } else if allorone == 3 {
        calc_all_primes();
    } else if allorone == 2 {
        calc_primes_to_num();
    } else if allorone == 4 {
        // mersenne();
        println!("presently in development");
    } else {
        println!("That's not an option! Please try again.");
    }
}
fn calc_all_primes() {
    let mut primes = vec![2];
    println!("How many primes?");
    let mut num_of_primes = String::new();
    io::stdin()
        .read_line(&mut num_of_primes)
        .expect("Failed to read line.");
    let num_of_primes: usize = num_of_primes.trim().parse().unwrap_or(0);
    let mut x = 3;
    while primes.len() < num_of_primes {
        if is_prime_3(x) {
            primes.push(x);
        }
        x += 1;
    }
    println!("{:?}", primes);
}

fn is_prime() {
    println!("What number do you want to check the primality of?");
    let mut check = String::new();
    io::stdin()
        .read_line(&mut check)
        .expect("Failed to read line.");
    let check: usize = match check.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let mut i = 2;
    let mut is_num_prime = true;
    let mut divisors = vec![1, check];
    let num_rt = (sqrt(check as f64)).floor();
    let num_rt = num_rt as usize;
    while i < num_rt + 1 {
        if check % i == 0 {
            is_num_prime = false;
            divisors.push(i);
            divisors.push(check / i);
        }
        i += 1;
    }
    if is_num_prime {
        println!("{} is prime!", check);
    } else {
        println!("{} is not prime! Its divisors are: {:?}", check, divisors);
    }
}
// is_prime_2 is for multithreading
fn is_prime_2() {
    println!("What number do you want to check the primality of?");
    let mut check = String::new();
    io::stdin()
        .read_line(&mut check)
        .expect("Failed to read line.");
    let check: usize = match check.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    let (tx, rx) = mpsc::channel();
    let mut divisors = vec![1, check];
    let num_rt = (sqrt(check as f64)).floor();
    let mut num_rt_div = num_rt / (2.0);
    num_rt_div = num_rt_div.floor();
    let num_rt_div = num_rt_div as usize;
    let num_rt = num_rt as usize;
    let handle = thread::spawn(move || {
        for i in 1..num_rt_div + 1 {
            if check % i == 0 {
                divisors.push(i);
                divisors.push(check / i);
            }
        }
        tx.send(divisors).unwrap();
    });
    let mut div2 = vec![1, check];
    for i in num_rt_div..num_rt + 1 {
        if check % i == 0 {
            div2.push(i);
            div2.push(check / i);
        }
    }
    let mut received = rx.recv().unwrap();
    handle.join().unwrap();
    received.append(&mut div2);
    received.sort();
    received.dedup();
    let mut is_num_prime = true;
    if received.len() > 2 {
        is_num_prime = false;
    }
    if is_num_prime {
        println!("{} is prime!", check)
    } else {
        println!("{} is not prime! Its divisors are: {:?}", check, received)
    }
}

fn is_prime_3(check: usize) -> bool {
    let mut i = 2;
    let mut is_num_prime = true;
    let threepointonefour = (sqrt(check as f64)).floor();
    let threepointonefour = threepointonefour as usize;
    while i < threepointonefour + 1 {
        if check % i == 0 {
            is_num_prime = false;
            break;
        }
        i += 1;
    }
    is_num_prime
}
fn calc_primes_to_num() {
    println!("What is the number you want to calculate the primes to?");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: usize = num.trim().parse().expect("Enter an integer!");
    let num = num + 1;
    let mut primes: Vec<bool> = Vec::new();

    let mut ret: Vec<usize> = Vec::new();

    for _i in 1..(num + 1) {
        primes.push(true);
    }
    for i in 2..num {
        if primes[i] {
            ret.push(i);
            for j in ((i.pow(2))..num).step_by(i) {
                primes[j] = false;
            }
        }
    }
    println!("{:?}", ret)
}
fn sqrt(idk: f64) -> f64 {
    idk.powf(0.5)
}
// lucas lehmer test for mersenne primes
/* fn mersenne() -> bool {
    let mut p = String::new();
    io::stdin().read_line(&mut p).expect("Failed to read line");
    let p: u32 = p.trim().parse().expect("Please enter a number!");
    if p == 2 {
        true
    } else if p == 1 {
        false
    } else if is_prime_3(p.try_into().unwrap()) {
        let mut s: u8 = 4;
        let M = 2_u8.pow(p) - 1;
        for _i in 1..(p-1) {
            s = (s.pow(2) - 2) % M;
        }
        if s == 0 {
            true
        } else {
            false
        }
    }
} */
