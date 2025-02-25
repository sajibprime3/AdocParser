use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct Company {
    name: String,
    location: String,
    technologies: Vec<String>,
    website: Vec<(String, String)>,
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
    record.split("|").for_each(|str| fields.push(str.trim().to_string()));
    
    //if the first slice doesn't contain anything, skip it.
    if fields[0].is_empty() || fields[0].starts_with("\n") {
        return fields[1..].to_vec();
    }
    
    return fields;
}


fn extract_links(text: &str) -> Vec<(String, String)> {
    let link_re = Regex::new(r"(https?://[^\s\[]+)\[([^\]]+)\]").unwrap();
    link_re
        .captures_iter(text)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect()
}


fn map_record_to_company(record: &str) -> Company {
    
    let fields = split_fields(&record);
    
    let company = Company {
        name: fields[0].clone(),
        location: fields[1].clone(),
        technologies: fields[2]
            .split(",")
            .map(|s| s.trim().to_string())
            .collect(),
        website: extract_links(&fields[3])
    };
    
    
    return company;
}