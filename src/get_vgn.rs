use serde::Deserialize;
use reqwest;


#[derive(Deserialize, Debug)]
pub(crate) struct VgnResponse {
    pub Haltestellenname: String,
    pub Abfahrten: Vec<TramAbfahrt>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TramAbfahrt {
    pub Linienname: String,
    pub Richtungstext: String,
    pub AbfahrtszeitSoll: String,
    pub AbfahrtszeitIst: String,
}

pub(crate) async fn vgn() -> Result<VgnResponse, reqwest::Error> {
    let station_id = 335;
    let http_response = match reqwest::get(format!("https://start.vag.de/dm/api/v1/abfahrten/VGN/{station_id}")).await {
        Ok(value) => value,
        Err(e) => return Err(e)
    };
    let response = match http_response.json::<VgnResponse>().await {
        Ok(value) => value,
        Err(e) => return Err(e)
    };
    Ok(response)
}