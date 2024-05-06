use std::{env, io::{Read, Write}, net::TcpStream, process};
use url::Url;

#[derive(Debug)]
enum Flag {
    Header(String),
    Method(String),
    Data(String),
    Url(String),
    Verbose,
    Help,
}

fn main() {
    
    let mut args = env::args();
    args.next();
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
    let verbose_flags: Vec<&str> = vec![
        "-v", "--verbose"
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

                    element if verbose_flags.contains(&&element.as_str()) => {
                        flags.push(Flag::Verbose);
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

    let data = flags.iter().find_map(|flag| {
        match flag  {
            Flag::Data(flag) => Some(flag.as_str()),
            _ => None,
        }
    }).unwrap_or("");

    // TODO: if Flag is your own type, give it a fn as_method(&self) -> Option<&str> to shorten this
    // let method = flags.iter().find_map(|f| f.as_method()).unwrap_or("GET");

    let (protocol, host, port, path) = parse_url(url);

    let raw_request = construct_get_request(protocol, host, port, path, method, headers, data);
    let socket_address = format!("{}:{}", host, port);
    let tcp = TcpStream::connect(socket_address);
    let is_verbose = flags.iter().find_map(|flag| {
        match flag {
            Flag::Verbose => Some(true),
            _ => None
        }
    }).unwrap_or(false);

    match tcp {
        Ok(mut stream) => {
            if is_verbose {
                let lines = raw_request.lines();
                for line in lines {
                    println!("> {}", line);
                }
            }
            stream
                .write_all(raw_request.as_bytes())
                .expect("Failed to write data to stream");
            
            let mut buffer =vec![0; 2048];

            stream
                .read_to_end(&mut buffer)
                .expect("Failed to read response from host");

            let response = String::from_utf8_lossy(&buffer[..]);

            let (response_header, response_data) = parse_response(&response);
            if is_verbose {
                let lines = response_header.split("\r\n");
                for line in lines {
                    println!("< {}", line);
                }
            }
            println!("{}", response_data);
        }
        Err(e) => {
            eprintln!("Failed to establish a connection: {}", e)
        }
    }

}

pub fn parse_response(response: &str) -> (&str, &str) {
    let (response_header, response_data) = response.split_once("\r\n\r\n").unwrap();
    (response_header, response_data)
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

pub fn get_protocol(protocol: &str) -> &str {
    // Temporary: Refactor this, since both arms are same
    match protocol {
        "http" => "HTTP/1.1",
        _ => "HTTP/1.1"
    }
}
pub fn construct_get_request(protocol: &str, host: &str, port: &str, path: &str, method: &str, headers: Vec<&str>, data: &str) -> String{
    let mut res: String = String::new();
    
    res += &format!("{} /{} {}\r\n", method, path, get_protocol(protocol));
    res += &format!("Host: {}\r\n", host);
    res += "Accept: */*\r\n";
    res += "Connection: close \r\n";
    res += "User-Agent: MyRustClient/1.0 \r\n";

    if method == "PUT" || method == "POST" {
        if headers.len() > 0 {
            for header in headers {
                res += header;
            }
        } else {
            res += "Content-Type: application/json\r\n"
        }
        let data_bytes = data.as_bytes();
        res += &format!("Content-Length: {} \r\n\r\n", data_bytes.len());
        res += data;
        res += "\r\n";
    }
    res += "\r\n";
    res
}