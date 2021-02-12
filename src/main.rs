use std::process::Command;
use std::{thread, time};

const FILTER_SCHEMES: [&str; 2] = [
    "a5dfb5f0-6a84-4df0-829a-7cd6f7a8880a",
    "e1fe5768-7cc3-4420-9f44-cf3109516724",
];

fn main() {
    let output = Command::new("cmd")
        .args(&["/C", "powercfg list"])
        .output()
        .expect("Unexpected behavior!");

    let output = String::from_utf8_lossy(&output.stdout).to_string();
    let mut schemes = get_schemes_from_string(&output);
    schemes.push(schemes[0].clone());
    let mut active_scheme_found = false;
    for scheme in schemes {
        if active_scheme_found {
            scheme.activate();
            println!("Scheme {} ({}) activated", scheme.guid, scheme.name);
            break;
        }
        active_scheme_found |= scheme.is_active;
    }

    thread::sleep(time::Duration::from_secs(1));
}

#[derive(Debug, Clone)]
struct Scheme {
    name: String,
    guid: String,
    is_active: bool,
}

impl Scheme {
    fn from_string(s: &str) -> Scheme {
        let split: Vec<&str> = s.split_whitespace().collect();
        Scheme {
            name: split[4].replace("(", "").replace(")", ""),
            guid: split[3].to_string(),
            is_active: split.len() == 6,
        }
    }

    fn activate(&self) {
        Command::new("cmd")
            .args(&[
                "/C",
                ("powercfg -setactive ".to_string() + &self.guid).as_str(),
            ])
            .spawn()
            .expect("Unexpected behavior!");
    }
}

fn get_schemes_from_string(s: &String) -> Vec<Scheme> {
    let mut result = Vec::new();

    let scheme_strings: Vec<&str> = s.lines().collect();

    let mut scheme: Scheme;
    for scheme_string in &scheme_strings[3..] {
        scheme = Scheme::from_string(scheme_string);
        if FILTER_SCHEMES.contains(&scheme.guid.as_str()) {
            result.push(scheme);
        }
    }
    result
}
