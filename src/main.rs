use clap::Parser;
use portscan::{Args, Scanner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::try_parse()?;

    let scanner = Scanner::new(&args)?;

    let results = scanner.run().await?;
    let res_len = results.len();

    println!("\nResults:");
    println!("{:<10} {:<15} {:<15} LATENCY", "PORT", "STATE", "SERVICE");
    println!("{}", "-".repeat(50));

    for result in results {
        let service = result.service.unwrap_or("Unknown".to_string());

        println!(
            "{:<10} {:<15} {:<15} {:.2}ms",
            result.port,
            "open",
            service,
            result.latency.as_secs_f64() * 1000.0
        );
    }

    println!("\nScan completed!");
    println!("Found {} open ports", res_len);

    Ok(())
}
