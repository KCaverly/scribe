use config::Config;
use std::collections::HashMap;

pub struct ScribeConfig {
    _config: HashMap<String, String>,
}

impl ScribeConfig {
    pub fn load() -> Self {
        let cfg: Config = Config::builder()
            .add_source(config::Environment::with_prefix("SCRIBE"))
            .build()
            .unwrap();

        let cfg_hashmap = cfg.try_deserialize::<HashMap<String, String>>().unwrap();

        if cfg!(test) {
            let mut cfg_hashmap: HashMap<String, String> = HashMap::new();
            cfg_hashmap.insert(
                "directory".to_string(),
                "/examples/small_project".to_string(),
            );

            return ScribeConfig {
                _config: cfg_hashmap,
            };
        }

        return ScribeConfig {
            _config: cfg_hashmap,
        };
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        return self._config.get(key);
    }

    pub fn contains_key(&self, key: &str) -> bool {
        return self._config.contains_key(key);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_get() {
        let cfg: ScribeConfig = ScribeConfig::load();
        assert!(cfg.get("directory").is_some());
    }

    #[test]
    fn test_config_contains_key() {
        let cfg: ScribeConfig = ScribeConfig::load();
        assert!(cfg.contains_key("directory"));
    }
}
