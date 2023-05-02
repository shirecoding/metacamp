use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

mod location;

use location::Continent;
mod transaction;
use transaction::Transaction;


fn filter_transactions_by_continent<'a> (transactions: &'a Vec<Transaction>, continent: &Continent) -> Vec<&'a Transaction> {
    transactions.iter()
        .filter(|&t| t.continent == *continent)
        .collect()
}


fn main() {
    // create file handler using File::open
    let file = File::open("./transactions.csv").unwrap();

    // create BufReader to read file into buffer
    let reader = BufReader::new(file);

    let mut transactions: Vec<Transaction> = Vec::new();
    let mut skipped_lines: Vec<_> = Vec::new();
    let mut total_per_continent: HashMap<location::Country, f64> = HashMap::new();

    // read lines from buffer
    for (idx, line) in reader.lines().enumerate() {
        
        // skip header row
        if idx == 0 {
            continue;
        }

        // parse line to Transaction
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);

        match parsed_transaction {
            Ok(t) => {
                // increment total_per_continent amount (Note: Country derive Copy to prevent partial move)
                *total_per_continent.entry(t.country).or_insert(0.0) += t.amount;

                // collate transactions
                transactions.push(t);
            }
            Err(e) => skipped_lines.push((idx, e, line_str))
        }
    }

    println!("\nValid Transactions:\n");
    for t in transactions.iter() {
        println!("{:?}", t);
    }

    println!("\nSkipped Lines:\n");
    for skipped_line in skipped_lines.iter() {
        println!("{:?}", skipped_line);
    }

    println!("\nTotal Per Continent:\n");
    for (country, amount) in total_per_continent.iter() {
        println!("{:?}: {}", country, amount)
    }

    println!("\nFilter by Continent (Asia):\n");
    for t in filter_transactions_by_continent(&transactions, &Continent::Asia) {
        println!("{:?}", *t);
    }
    
}
