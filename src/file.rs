use crate::colours::{self, LsColours};

fn config_path() -> String {
    format!(
        "{}/lscoltui.toml",
        dirs::config_dir().unwrap().to_str().unwrap()
    )
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SaveFile {
    pub most_recent: String,
    pub schemes: Vec<(String, colours::LsColours)>,
}

impl SaveFile {
    pub fn load() -> Self {
        if std::path::Path::new(&config_path()).exists() {
            toml::from_str(&std::fs::read_to_string(&config_path()).unwrap()).unwrap()
        } else {
            SaveFile {
                most_recent: String::new(),
                schemes: vec![],
            }
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        std::fs::write(config_path(), toml::to_string_pretty(self).unwrap())
    }
}

pub fn env_command(colours: &LsColours) -> String {
    format!("export LS_COLORS=\'{}\'", colours.parse())
}
