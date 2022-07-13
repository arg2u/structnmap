use std::{collections::HashMap, fs::File, io::Write};

#[derive(Debug)]
pub struct Data {
    pub raws: Vec<TableRaw>,
    pub protocol_map: HashMap<String, ProtocolValue>,
}

impl Data {
    pub fn build(file: String) -> Self {
        let raws = file.split("\n").collect::<Vec<&str>>();
        let mut raws_vec: Vec<TableRaw> = Vec::new();
        for raw in raws {
            if let Some(raw_vec) = TableRaw::new(&raw.to_string()) {
                raws_vec.push(raw_vec);
            }
        }
        let mut protocol_map: HashMap<String, ProtocolValue> = HashMap::new();
        for raw in &raws_vec {
            if let Some(p_value) = protocol_map.get_mut(&raw.protocol) {
                if !p_value.ports.contains(&raw.port) {
                    p_value.ports.push(raw.port.clone());
                }
                if !p_value.hosts.contains(&raw.host) {
                    p_value.hosts.push(raw.host.clone());
                }
            } else {
                let mut p_value = ProtocolValue::new();
                p_value.ports.push(raw.port.clone());
                p_value.hosts.push(raw.host.clone());
                protocol_map.insert(raw.protocol.clone(), p_value);
            }
        }
        Data {
            raws: raws_vec,
            protocol_map,
        }
    }

    pub fn generate(&self, save_path: &str) {
        let mut hosts_map_file = File::create(format!("{}/hosts_map.txt", save_path)).unwrap();
        let mut file_buf_str: String = String::new();
        for key in self.protocol_map.keys() {
            let value = self.protocol_map.get(key).unwrap();
            let mut hosts_file = File::create(format!("{}/{}.txt", save_path, key)).unwrap();
            let hosts_buf = value.hosts.join("\n");
            hosts_file.write(&hosts_buf.as_bytes()).unwrap();
            let ports_buf = value.ports.join(", ");
            file_buf_str += &format!(
                "File: {}.txt\nProtocol: {}\nPorts: {}\n\n",
                key, key, &ports_buf
            );
        }
        hosts_map_file.write(&file_buf_str.as_bytes()).unwrap();
    }
}

#[derive(Debug)]
pub struct TableRaw {
    port: String,
    protocol: String,
    host: String,
}

impl TableRaw {
    fn new(raw: &str) -> Option<Self> {
        let raw_vec: Vec<String> = raw.split("\t").map(|s| s.to_lowercase()).collect();
        if raw_vec.len() > 3
            && !raw_vec[1].is_empty()
            && !raw_vec[2].is_empty()
            && raw_vec[2] != "unknown"
        {
            Some(TableRaw {
                port: raw_vec[1].clone(),
                protocol: raw_vec[2].clone(),
                host: raw_vec[0].clone(),
            })
        } else {
            None
        }
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolValue {
    ports: Vec<String>,
    hosts: Vec<String>,
}

impl ProtocolValue {
    fn new() -> Self {
        Self {
            ports: Vec::new(),
            hosts: Vec::new(),
        }
    }
}
