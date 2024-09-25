use serde::{Serialize, Deserialize};
use std::{fs::File, fs};
use std::io::prelude::*;

type Servicos = Vec<Servico>;

#[derive(Debug, Serialize, Deserialize)]
struct Servico {
    path: Option<String>,
    prefix: Option<String>,
    hasRatelimits: Option<String>,
    safeRegex: Option<SafeRegex>
}

#[derive(Debug, Serialize, Deserialize)]
struct SafeRegex {
    regex: String
}

fn main() {

    let contents = fs::read_to_string("./out.json").unwrap();

    let servicos: Servicos = serde_json::from_str(&contents).unwrap();

    // println!("{:?}", servicos);

    let mut csv = String::new();

    for servico in servicos {

        let path = servico.path.unwrap_or("".to_string());
        let prefix = servico.prefix.unwrap_or("".to_string());
        let safe_regex = match servico.safeRegex {
            Some(sr) => sr.regex,
            None => "".to_string()
        };
        let has_rate_limits = match servico.hasRatelimits {
            Some(_) => "true".to_string(),
            None => "false".to_string()
        };
        println!("{}{}{}, {}", path, prefix, safe_regex, has_rate_limits);

        csv.push_str(&format!("{}{}{},{}\n", path, prefix, safe_regex, has_rate_limits));
    }

    let mut file = File::create("out2.csv").unwrap();
    file.write_all(csv.as_bytes()).unwrap();

}