use serde::Deserialize;
use anyhow::Result;

use crate::{Context, extract::{Json, IntoBody}};

#[derive(Deserialize)]
pub struct Menu {
    pub id: String,
    pub name: String
}

impl Menu {

    pub async fn get_from_context(ctx: &Context) -> Result<Vec<Self>> {

        let req = ctx.client
            .get(format!(
                "https://buy.rguest.com/api/buy/kiosk/tenants/{}/businessContexts/{}/menus",
                &ctx.tenant.id,
                &ctx.context_id
            ));

        let res = req.send().await?;

        let Json(menus): Json<Vec<Menu>> = res.into_body().await?;

        Ok(menus)
    }

}


