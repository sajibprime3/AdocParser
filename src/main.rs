use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Company {
    name: String,
    location: String,
    technologies: Vec<String>,
    website: String,
}

fn main() {
    
    let adoc_content = "
|Adventure Dhaka Limited
|Head Office: Autograph Tower, 67-68, Kemal Ataturk Avenue, 17th & 8th Floor, Dhaka, Dhaka 1213
|Golang, JAVA, Swift, Flutter, Javascript, React, Nextjs, DevOps
|https://adventurekk.com/company/about/[Website]
";
    
    let re = Regex::new(r"(?m)^\|(.+)$").unwrap();
    
    let matches: Vec<String> = re
        .captures_iter(&adoc_content)
        .map(|cap| cap[1].trim().to_string())
        .collect();
    
    if matches.len() < 4 {
        eprintln!("Error: Not enough data in AsciiDoc file.");
        return;
    }
    
    let company = Company {
        name: matches[0].clone(),
        location: matches[1].clone(),
        technologies: matches[2]
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(),
        website: matches[3].split("[").next().unwrap().to_string()
    };
    
    println!("Parsed Data: {:?}", company);
    
    
}