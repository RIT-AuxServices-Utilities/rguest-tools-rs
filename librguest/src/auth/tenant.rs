use std::fmt::Display;
use reqwest::Client;
use serde::Deserialize;
use crate::extract::{Json, IntoBody};

use super::role::Role;
use anyhow::Result;

#[derive(Deserialize, Clone, Default)]
pub struct Tenant {
    #[serde(rename = "tenantId")]
    pub id: String,
    #[serde(rename = "tenantName")]
    pub name: String,
}

impl Display for Tenant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} ({})",self.name, self.id))?;
        Ok(())
    }
}

#[derive(Deserialize)]
struct RolesRequest {
    #[serde(rename = "contextRoles")]
    roles: Vec<Role>
}

#[derive(Deserialize)]
pub struct BusinessContext {
    #[serde(rename = "businessContextId")]
    pub id: String,
    #[serde(rename = "storeName")]
    pub name: String
}

impl Display for BusinessContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        Ok(())
    }
}

impl Tenant {

    pub async fn get_contexts(&self, client: &Client) -> Result<Vec<BusinessContext>> {
        let req = client
           .get(format!("https://buy.rguest.com/api/buy/kiosk/tenants/{}/storeInfos", self.id));

        let res = req.send().await?;
        
        let Json(ctxs): Json<Vec<BusinessContext>> = res.into_body().await?;
        
        Ok(ctxs)
    }

    pub async fn get_roles(&self, client: &Client) -> Result<Vec<Role>> {
        
        let req = client
            .get(format!("https://buy.rguest.com/user-service/user/tenants/{}/users/details", self.id));
        
        let res = req.send().await?;

        let Json(roles): Json<RolesRequest> = res.into_body().await?;

        Ok(roles.roles)
    
    }

}
