use anyhow::Result;
use serde::Deserialize;
use time::{Time, format_description};
use crate::{api::{auth::context::Context, API_CLIENT}, extract::{IntoBody, Json}};

#[derive(Debug, Deserialize)]
struct DaypartRequest {
    #[serde(rename = "startTime")]
    start_time: String,
    #[serde(rename = "endTime")]
    end_time: String
}

#[derive(Debug)]
pub struct Daypart {
    pub start: Time,
    pub end: Time
}

impl Daypart {

    pub async fn get_from_context(ctx: &Context) -> Result<Vec<Self>> {
        
        let req = API_CLIENT.lock().await
            .get(format!(
                "https://buy.rguest.com/api/buy/kiosk/tenants/{}/businessContexts/{}/day-parts",
                ctx.tenant.id,
                ctx.context_id
            ));

        let res = req.send().await?;

        let Json(body): Json<Vec<DaypartRequest>> = res.into_body().await?;

        let format = format_description::parse("[hour]:[minute]")?;

        body.into_iter()
            .map(|d| Ok(Daypart {
                start: Time::parse(&d.start_time, &format)?,
                end: Time::parse(&d.end_time, &format)?
            })).collect()
    }

}
