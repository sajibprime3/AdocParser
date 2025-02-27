use regex::Regex;
use reqwest::blocking::Client;
use serde::Serialize;
mod utils;
use utils::parser::AdocParser;


#[derive(Serialize, Debug)]
struct CompanyRecord {
    name: String,
    location: String,
    technologies: Vec<String>,
    website: Vec<(String, String)>,
}



fn main() {
    let adoc_url = "https://raw.githubusercontent.com/MBSTUPC/tech-companies-in-bangladesh/refs/heads/master/README.adoc";
    let client: Client = Client::new();
    
    let adoc_content = client.get(adoc_url).send().unwrap().text().unwrap();
    
    let parser =AdocParser::new(adoc_content);
    
    let records = parser.parse_to_records();
    
    let mut companies: Vec<CompanyRecord> = vec![];
    for record in records {
        companies.push(map_record_to_company(&record));
    }
    
    companies.iter().for_each(|company| println!("{:?}", company));
    
    
    
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


fn map_record_to_company(record: &str) -> CompanyRecord {
    
    let fields = split_fields(&record);
    
    let company = CompanyRecord {
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