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

    let url: &str = flags.iter().find_map(|flag| {
        match flag {
            Flag::Url(flag) => Some(flag.as_str()),
            _ => None
        }
    }).unwrap_or_else(|| {
        println!("No appropriate Url value provided");
        process::exit(1);
    });

    let headers: Vec<&str> = flags.iter().filter_map(|flag| {
        match flag {
            Flag::Header(flag) => {
                return Some(flag.as_str());
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

    let (protocol, host, port, path) = parse_url(url);

    let request_string = construct_get_request(protocol, host, port, path, method, headers);
}

pub fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}

pub fn parse_url(url: &str) -> (&str, &str, &str, &str) {
    let default_port = "80";

    let (protocol, remaining_url) = if url.contains("://") {
        let mut parts_of_protocol_url =  url.splitn(2, "://");
        let protocol_part = parts_of_protocol_url.next().unwrap();
        let remaining_url = parts_of_protocol_url.next().unwrap();
        (protocol_part, remaining_url)
    } else {
        ("", url)
    };

    let (host_and_port, path) = if remaining_url.contains("/") {
        let mut parts_of_host_port_path = remaining_url.splitn(2, "/");
        let host_and_port_part = parts_of_host_port_path.next().unwrap();
        let path = parts_of_host_port_path.next().unwrap();
        (host_and_port_part, path)
    } else {
        (remaining_url, "")
    };

    let (host, port) = if host_and_port.contains(":") {
        let mut parts_of_host_port = host_and_port.splitn(2, ":");
        let host_part = parts_of_host_port.next().unwrap();
        let port = parts_of_host_port.next().unwrap();
        (host_part, port)
    } else {
        (host_and_port, default_port)
    };
   

    (protocol, host, port, path)
}

pub fn construct_get_request(protocol: &str, host: &str, port: &str, path: &str, method: &str, headers: Vec<&str>) {
    let mut res: String = String::new();
    
    res += &format!("{} {}:{}/{} \r\n", method, host, port, path);
    res += &format!("Protocol: {}\r\n", protocol);
    res += &format!("Host: {}\r\n", host);
    res += "Accept: */*\r\n";
    res += "Connection: close \r\n";

    println!("{}", res);
}