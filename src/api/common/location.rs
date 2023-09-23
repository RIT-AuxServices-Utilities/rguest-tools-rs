use serde::Deserialize;
use anyhow::Result;
use crate::{api::{auth::context::Context, API_CLIENT}, extract::{IntoBody, Json}};

#[derive(Debug, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    #[serde(rename = "workflowType")]
    pub workflow_type: String
}

impl Location {

    pub async fn get_from_context(ctx: &Context) -> Result<Vec<Self>> {
        let req = API_CLIENT.lock().await
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

