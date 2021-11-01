use proc_macro::error;
use std::collections::HashMap;
use std::path::Path;

pub struct Configuration
{
    pub m_defaults: HashMap<String, HashMap<String, String>>,
    pub m_loaded_settings: HashMap<String, HashMap<String, String>>,
    pub m_settings: HashMap<String, HashMap<String, String>>,
}

impl Configuration {
    pub fn new(defaults: &HashMap<String, HashMap<String, String>>) -> Self {
        Self {
            m_defaults: defaults.clone(),
            m_loaded_settings: HashMap::new(),
            m_settings: HashMap::map(),
        }
    }

    pub fn load_data(&mut self, file_name: &String) {
        // TODO: load configuration file
        // TODO: set m_loaded_settings
        // TODO: set m_settings
    }

    pub fn load(&mut self, file_name: &String) {
        info!("initializing configuration...");
        let mut path = Path::new(file_name);
        if !path.exists() {
            error!(format!("configuration file {} does not exist. using default options", file_name))
        } else {
            // TODO: read file
            // TODO: Parse file
            info!("done");
        }
    }

    pub fn save_data(&self) -> String {
        format_xml_data(&self.m_settings, "configuration", "automatically generated configuration file")
    }

    pub fn save(&self, file_path: &String) {
        // TODO: open/create file
        let xml_string = self.save_data();
        // TODO: write file
    }
}
