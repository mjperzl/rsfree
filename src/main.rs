use std::{
    fmt::Display,
    io::{ErrorKind, Read},
};

#[derive(Debug, Default)]
struct Output {
    total: f64,
    aval: f64,
    free: f64,
    buff: f64,
    percatage: f64,
}

impl Output {
    fn parse_from_file(&mut self) {
        let file = "/proc/meminfo";
        let fh = std::fs::File::open(file);
        match fh {
            Ok(mut meminfo) => {
                let mut mem = String::new();
                meminfo.read_to_string(&mut mem).unwrap();
                for line in mem.lines() {
                    if line.starts_with("MemTotal:") {
                        self.total = match parse_line(line) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error at Mem Total {}", e);
                                panic!()
                            }
                        }
                    } else if line.starts_with("MemFree:") {
                        self.free = match parse_line(line) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error at Mem Free {}", e);
                                panic!()
                            }
                        }
                    } else if line.starts_with("MemAvailable") {
                        self.aval = match parse_line(line) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error at Mem Availible {}", e);
                                panic!()
                            }
                        }
                    } else if line.starts_with("Buffers") {
                        self.buff = match parse_line(line) {
                            Ok(v) => v,
                            Err(e) => {
                                eprintln!("Error at Buffers: {}", e);
                                panic!()
                            }
                        }
                    }
                }
            }
            Err(e) => {
                panic!("Error Opening File: {}", e);
            }
        }
        self.percatage = (self.free / self.total) * 100 as f64;
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Toatal: {:.2}Gb\t", self.total / (1024.0 * 1024.0))?;
        write!(f, "Avalible: {:.2}Gb\t", self.aval / (1024.0 * 1024.0))?;
        write!(f, "Free: {:.2}Gb\t", self.free / (1024.0 * 1024.0))?;
        write!(f, "Bufferd: {:.2}Gb\t", self.buff / (1024.0 * 1024.0))?;
        write!(f, "Free%: {:.4}%", self.percatage)
    }
}

fn parse_line(to_parse: &str) -> Result<f64, std::io::Error> {
    for elem in to_parse.split_whitespace().into_iter() {
        match elem.parse::<f64>() {
            Ok(elem) => return Ok(elem),
            Err(_) => {
                continue;
            }
        }
    }
    Err(std::io::Error::new(
        ErrorKind::InvalidData,
        "Could not parse any value please check your input",
    ))
}
fn main() {
    let mut out = Output::default();
    loop {
        out.parse_from_file();
        println!("{}", out);
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
