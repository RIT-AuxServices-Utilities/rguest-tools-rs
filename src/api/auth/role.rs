use std::fmt::Display;

use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Role {
    #[serde(rename = "contextId")]
    pub id: String,
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)?;
        Ok(())
    }
}
