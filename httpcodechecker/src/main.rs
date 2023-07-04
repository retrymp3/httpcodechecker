// To use .lines() before, just like last time
use std::io::BufRead;

// We'll return _some_ kind of an error
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the file for input
    let file = std::fs::File::open("urls.txt")?;
    // Make a buffered version so we can read lines
    let buffile = std::io::BufReader::new(file);

    // CSV header
    println!("URL,Status");

    // Create a client so we can make requests
    let client = reqwest::blocking::Client::new();

    for line in buffile.lines() {
        // Error handling on reading the lines in the file
        let line = line?;
        // Make a request and send it, getting a response
        let resp = client.get(&line).send()?;
        // Print the status code
        println!("{},{}", line, resp.status().as_u16());
    }
    Ok(())
}

//Quick bash command to get urls only from httstatuscodes.txt
//cat hostswithhttp.txt| cut -d ":" -f 2,3