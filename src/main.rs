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
    let adoc_url = "https://raw.githubusercontent.com/MBSTUPC/tech-companies-in-bangladesh/refs/heads/master/README.adoc";
    let client: Client = Client::new();
    
    let adoc_content = client.get(adoc_url).send().unwrap().text().unwrap();
    
    let orders = split_orders(&adoc_content);
    
    let mut records: Vec<String> = vec![];
    for tc in orders {
        records.append(&mut split_records(&tc));
        
    }
    
    
    let mut companies: Vec<Company> = vec![];
    for record in records {
        companies.push(map_record_to_company(&record));
    }
    
    companies.iter().for_each(|company| println!("{:?}", company));
    
}

fn split_orders(adoc_content: &str) -> Vec<String> {
    let table_re = Regex::new(r"(?ms)\|===\s*\n(.*?)\|===")
        .expect("Invalid Regex!");
    
    let mut orders: Vec<String> = vec![];
    for cap in table_re.captures_iter(&adoc_content) {
        let table_content = cap[1].trim().to_string();
        orders.push(table_content);
    }
    
    return orders;
}

fn split_records(table_content: &str) -> Vec<String> {
    let mut records:Vec<String> = vec![];
    table_content.split("\n\n").for_each(|str| records.push(str.to_string()));
    
    
    // If the first record is the header row, skip it.
    if let Some(first) = records.first() {
        if first.contains("Company Name") {
            return records[1..].to_vec();
        }
    }
    
    return records;
}

fn split_fields(record: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current_field = String::new();

    for line in record.lines() {
        if line.starts_with("|") {
            // If there's an existing field, push it
            if !current_field.is_empty() {
                fields.push(current_field.trim().to_string());
            }
            // Start a new field without the leading '|'
            current_field = line[1..].to_string();
        } else {
            // Append the line (trimmed) to the current field (with a space)
            current_field.push(' ');
            current_field.push_str(line.trim());
        }
    }
    if !current_field.is_empty() {
        fields.push(current_field.trim().to_string());
    }
    fields
}


fn map_record_to_company(record: &str) -> Company {
    
    let matches = split_fields(&record);
    
    let company = Company {
        name: matches[0].clone(),
        location: matches[1].clone(),
        technologies: matches[2]
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(),
        website: matches[3].split("[").next().unwrap().to_string()
    };
    
    return company;
}