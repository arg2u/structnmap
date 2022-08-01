//! # Structnmap
//!
//! `structnmap` is a tool for parsing nmap xml and structing it to files by service name.

use std::{collections::HashMap, fs::File, io::Write, path::Path};

use parsenmap::models::scan::{FileType, Scan};

#[derive(Debug)]
pub struct Service {
    ports: Vec<String>,
    hosts: Vec<String>,
}

impl Service {
    fn new() -> Self {
        Self {
            ports: Vec::new(),
            hosts: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl From<std::io::Error> for Error {
    fn from(message: std::io::Error) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<&str> for Error {
    fn from(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl From<parsenmap::error::ParsenmapError> for Error {
    fn from(e: parsenmap::error::ParsenmapError) -> Self {
        Self { message: e.err }
    }
}

#[derive(Debug)]
pub struct Data {
    pub scan: Scan,
    pub structed_service: HashMap<String, Service>,
}

impl Data {
    ///
    /// Generates organized structure of hosts by service name
    ///
    pub fn build(xml_path: &str) -> Result<Self, Error> {
        let mut structed_service: HashMap<String, Service> = HashMap::new();
        match parsenmap::parse(&xml_path) {
            Ok(scan) => {
                let hosts_count = scan
                    .hosts
                    .iter()
                    .map(|h| h.addrs.len())
                    .reduce(|a, b| a + b)
                    .unwrap();
                if hosts_count > 0 {
                    for host in &scan.hosts {
                        let mut ipv4 = String::new();
                        for addr in &host.addrs {
                            if addr.kind == "ipv4" {
                                ipv4 = addr.addr.clone();
                            }
                        }
                        for port in &host.ports {
                            if structed_service.contains_key(&port.service.name) {
                                let service = structed_service.get_mut(&port.service.name).unwrap();
                                if !service.ports.contains(&port.port) {
                                    service.ports.push(port.port.clone());
                                }
                                if !service.hosts.contains(&ipv4) {
                                    service.hosts.push(ipv4.clone());
                                }
                            } else {
                                let mut new_service = Service::new();
                                new_service.ports.push(port.port.clone());
                                new_service.hosts.push(ipv4.clone());
                                structed_service.insert(port.service.name.clone(), new_service);
                            }
                        }
                    }
                    return Ok(Data {
                        structed_service,
                        scan,
                    });
                } else {
                    return Err(Error::from("There are no hosts in your xml"));
                }
            }
            Err(e) => return Err(Error::from(e)),
        };
    }

    ///
    /// Generates all files
    ///
    pub fn generate(&self, save_path: &str) -> Result<(), Error> {
        let path = Path::new(save_path);
        if path.exists() {
            let mut hosts_map_file = File::create(format!("{}/hosts-map.txt", save_path))?;
            let mut file_buf_str: String = String::new();
            for service_name in self.structed_service.keys() {
                let value = self.structed_service.get(service_name).unwrap();
                let mut hosts_file = File::create(format!("{}/{}.txt", save_path, service_name))?;
                let hosts_buf = value.hosts.join("\n");
                hosts_file.write(&hosts_buf.as_bytes())?;
                let ports_buf = value.ports.join(", ");
                file_buf_str += &format!(
                    "File: {}.txt\nProtocol: {}\nPorts: {}\n\n",
                    service_name, service_name, &ports_buf
                );
            }
            hosts_map_file.write(&file_buf_str.as_bytes())?;
            self.scan
                .write_to_file(FileType::CSV, &format!("{}/all-ports.csv", save_path));
            Ok(())
        } else {
            return Err(Error::from("Path is not exist"));
        }
    }
}
