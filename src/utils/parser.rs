use regex::Regex;

pub struct AdocParser {
    adoc_content : String
}
impl AdocParser {
    pub fn new(adoc_content: String) -> Self {
        Self {
            adoc_content: adoc_content
        }
    }
    
    pub fn parse_to_tables(&self) -> Vec<String> {
        return split_orders(&self.adoc_content);
    }
    
    pub fn parse_to_records(&self) -> Vec<String> {
    
    let orders = split_orders(&self.adoc_content);
    
    let mut records: Vec<String> = vec![];
    for tc in orders {
        records.append(&mut split_records(&tc));
    }
    
    return records;
    
    }
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