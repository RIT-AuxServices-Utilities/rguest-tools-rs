use serde::Deserialize;
use anyhow::Result;

use crate::{Context, extract::{IntoBody, Json}};

#[derive(Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    #[serde(rename = "workflowType")]
    pub workflow_type: String
}

impl Location {

    pub async fn get_from_context(ctx: &Context) -> Result<Vec<Self>> {
        let req = ctx.client
            .get(format!(
                "https://buy.rguest.com/api/buy/kiosk/tenants/{}/businessContexts/{}/displayProfiles",
                &ctx.tenant.id,
                &ctx.context_id
            ));

        let res = req.send().await?;

        let Json(locations): Json<Vec<Location>> = res.into_body().await?;

        Ok(locations)
    }

}

