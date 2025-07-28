use std::f64;
use std::io;

fn main() {
    println!("📊 Quadratic Funding Simulator (Rust)");

    // Get number of donors
    let mut input = String::new();
    println!("Enter number of donors:");
    io::stdin().read_line(&mut input).unwrap();
    let num_donors: usize = input.trim().parse().expect("Enter a number!");

    let mut sum_of_roots = 0.0;
    let mut total_donated = 0.0;

    for i in 1..=num_donors {
        input.clear();
        println!("Enter donation amount for donor #{i}:");
        io::stdin().read_line(&mut input).unwrap();
        let donation: f64 = input.trim().parse().expect("Enter a valid amount!");

        sum_of_roots += donation.sqrt();
        total_donated += donation;
    }

    let matching_fund = sum_of_roots.powi(2);
    let total_funding = total_donated + matching_fund;

    println!("\n--- Summary ---");
    println!("💰 Total Donated: ${:.2}", total_donated);
    println!("🔁 Matching Fund (QF): ${:.2}", matching_fund);
    println!("📦 Total Funding Available: ${:.2}", total_funding);
}
