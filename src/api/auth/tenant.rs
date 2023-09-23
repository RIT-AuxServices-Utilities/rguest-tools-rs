use std::fmt::Display;
use serde::Deserialize;
use crate::{api::API_CLIENT, extract::{Json, IntoBody}};
use super::{role::Role, pick};
use anyhow::{Result, Error};

#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Debug, Deserialize)]
struct RolesRequest {
    #[serde(rename = "contextRoles")]
    roles: Vec<Role>
}

#[derive(Debug, Deserialize)]
struct BusinessContext {
    #[serde(rename = "businessContextId")]
    id: String,
    #[serde(rename = "storeName")]
    name: String
}

impl Display for BusinessContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        Ok(())
    }
}

impl Tenant {

    async fn get_business_context(&self) -> Result<String> {
        let req = API_CLIENT.lock().await
           .get(format!("https://buy.rguest.com/api/buy/kiosk/tenants/{}/storeInfos", self.id));

        let res = req.send().await?;
        
        let Json(ctxs): Json<Vec<BusinessContext>> = res.into_body().await?;

        let bctx = pick(&ctxs, "business contexts").await;

        println!("Using business context: {bctx}");

        Ok(bctx.id.clone())
    }

    async fn get_role(&self) -> Result<Role> {
        
        let req = API_CLIENT.lock().await
            .get(format!("https://buy.rguest.com/user-service/user/tenants/{}/users/details", self.id));
        
        let res = req.send().await?;

        let Json(roles): Json<RolesRequest> = res.into_body().await?;
        let roles = roles.roles;

        if roles.len() < 1 {
            return Err(Error::msg("No roles were found on tenant"))
        }
        
        let role = pick(&roles, "roles").await;

        println!("Using role: {role}");
       
        Ok(role.clone())
    
    }

    pub async fn get_context_id(&self) -> Result<String> {

        let role = self.get_role().await?;

        if role.id == "Default" {
            Ok(self.get_business_context().await?)
        } else {
            Ok(role.id)
        }

    }
    
}
