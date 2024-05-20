use anyhow::Ok;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    pub clash: ClashConfig,
    pub server: Server,
}

#[derive(Default, Debug, Deserialize)]
pub struct Server {
    pub bind: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct ClashConfig {
    pub extra: String,
    pub subscribe: String,
    pub path: String,
}

impl ClashConfig {
    pub fn patch(
        &self,
        key: &str,
        prepend: bool,
        dst: &mut serde_yml::Value,
        src: &mut serde_yml::Value,
    ) {
        let key = serde_yml::Value::String(key.into());
        let mut empty: Vec<serde_yml::Value> = Vec::new();
        let src: &mut Vec<serde_yml::Value> = match src {
            serde_yml::Value::Mapping(map) => match map.get_mut(&key) {
                Some(serde_yml::Value::Sequence(seq)) => seq,
                _ => &mut empty,
            },
            _ => &mut empty,
        };

        match dst {
            serde_yml::Value::Mapping(map) => match map.get_mut(&key) {
                Some(serde_yml::Value::Sequence(seq)) => {
                    if !prepend {
                        seq.append(src);
                    } else {
                        src.append(seq);
                        *seq = src.clone();
                    }
                }
                _ => {
                    map.insert(key.clone(), serde_yml::Value::Sequence(src.clone()));
                }
            },
            _ => {}
        }
    }

    pub fn patch_rules(&self, dst: &mut serde_yml::Value) -> anyhow::Result<()> {
        let file = std::fs::File::open(&self.extra)?;
        let mut extra: serde_yml::Value = serde_yml::from_reader(file)?;
        self.patch("proxy-groups", false, dst, &mut extra);
        self.patch("rules", true, dst, &mut extra);

        Ok(())
    }
}
