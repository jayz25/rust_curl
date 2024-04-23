use std::{env, process};
use url::Url;

#[derive(Debug)]
enum Flag {
    Header(String),
    Method(String),
    Data(String),
    Url(String),
    Help,
}

fn main() {
    
    let mut args = env::args();
    let mut flags = Vec::new();
    let header_flags: Vec<&str> = vec![
        "-H", "--header"
    ];
    let method_flags: Vec<&str> = vec![
        "-X", "--request"
    ];
    let data_flags: Vec<&str> = vec![
        "-d", "--data"
    ];
    let help_flags: Vec<&str> = vec![
        "-h", "--help"
    ];

    loop {
        match args.next() {
            Some(element) => {
                match element {
                    element if is_valid_url(element.as_str()) => flags.push(Flag::Url(element.to_string())),

                    element if header_flags.contains(&element.as_str()) => {
                        let value =  args.next().expect("The value for -X/--request was not provided");
                        flags.push(Flag::Header(value.clone()));
                    },

                    element if data_flags.contains(&element.as_str()) => {
                        let value =  args.next().expect("The value for -d/--data was not provided");
                        flags.push(Flag::Data(value.clone()));
                    },

                    
                    element if method_flags.contains(&element.as_str()) => {
                        let value =  args.next().expect("The value for -X/--request was not provided");
                        flags.push(Flag::Method(value.clone()));
                    },

                    element if help_flags.contains(&element.as_str()) => {
                        flags.push(Flag::Help);
                    },

                    _ => {
                        println!("Unknown flag passed {}", element);
                    },

                };
            },

            None => {
               break;
            }
            
        }
    }

    let url: &String = flags.iter().find_map(|flag| {
        match flag {
            Flag::Url(flag) => Some(flag),
            _ => None
        }
    }).unwrap_or_else(|| {
        println!("No appropriate Url value provided");
        process::exit(1);
    });

    let headers: Vec<&String> = flags.iter().filter_map(|flag| {
        match flag {
            Flag::Header(flag) => {
                return Some(flag);
            },
            _ => None,
        }
    }).collect();

    let method: &str = flags.iter().find_map(|flag| {
        match flag {
            Flag::Method(flag) => Some(flag.as_str()),
            _ => None,
        }
    }).unwrap_or("GET");

    // TODO: if Flag is your own type, give it a fn as_method(&self) -> Option<&str> to shorten this
    // let method = flags.iter().find_map(|f| f.as_method()).unwrap_or("GET");

    let (protocol, host, domain) = parse_url(url);
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub fn parse_url(url: &String) -> (String, String, String) {
    let mut parts = url.splitn(2, "://");
    let default_port = String::from("80");

    let protocol = match parts.next() {
        Some(proto) => proto,
        None => return ("".to_string(), "".to_string(), default_port)
    };

    let remaining = parts.next().unwrap_or("");

    let (host, port) = if remaining.contains(":") {
        let mut parts_of_host_port = remaining.splitn(2, ":");
        let host = parts_of_host_port.next().unwrap();
        let port = parts_of_host_port.next();
        (host, port)
    } else {
        (remaining, Some("80"))
    };

    (protocol.to_string(), host.to_string(), port.unwrap().to_string())
}

// pub fn construct_get_request(flags: Vec<Flag>) {
//     let default_mothod = String::from("GET");
//     let method = flags.iter().filter_map(|flag| match flag {
//         Flag::Method(value) => Some(value),
//         _ => None,
//     });

//     let headers = flags.iter().filter_map(|flag| match flag {
//         Flag::Method(value) => Some(value),
//         _ => None,
//     }).collect::<Vec<&String>>();

//     let mut res: String = String::from("");
    
//     res += &format!("{} /{} {}\r\n", method, path, protocol);
//     res += &format!("Host: {}\r\n", host);
//     res += "Accept: */*\r\n";
//     res += "Connection: close \r\n";

//     println!(res);
// }