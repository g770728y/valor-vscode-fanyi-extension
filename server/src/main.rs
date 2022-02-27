use actix_web::{error, get, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};

const BAIDU_FANYI_BASE_URL: &str = "http://api.fanyi.baidu.com/api/trans/vip/translate";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv::dotenv();
    let port = dotenv::var("port").unwrap().parse().unwrap();
    println!("端口:{}", port);
    HttpServer::new(|| App::new().service(baidu_translate))
        .bind(("0.0.0.0", port))?
        .run()
        .await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct TranslateQuery {
    q: String,
}

#[get("/translate")]
async fn baidu_translate(web::Query(s): web::Query<TranslateQuery>) -> Result<impl Responder> {
    let q = s.q;
    let from = "zh";
    let to = "en";
    let appid = dotenv::var("baidu_appid").unwrap();
    let salt = dotenv::var("baidu_salt").unwrap();
    let secret = dotenv::var("baidu_secret").unwrap();
    let sign = md5::compute(format!("{}{}{}{}", appid, q, salt, secret).into_bytes());
    let url = format!(
        "{}?q={}&from={}&to={}&appid={}&salt={}&sign={}",
        BAIDU_FANYI_BASE_URL,
        urlencoding::encode(q.as_str()),
        from,
        to,
        appid,
        salt,
        format!("{:?}", sign)
    );
    let resp = reqwest::get(url)
        .await
        .map_err(|_| error::ErrorBadRequest("出错了..."))?
        .text()
        .await
        .map_err(|_| error::ErrorBadRequest("出错了..."))?;
    let translate_response: TranslateResponse = serde_json::from_str(resp.as_str())?;
    Ok(web::Json(translate_response.trans_result))
}

#[derive(Deserialize, Serialize, Debug)]
struct TranslateResult {
    src: String,
    dst: String,
}

#[derive(Deserialize, Debug)]
struct TranslateResponse {
    #[allow(dead_code)]
    from: String,
    #[allow(dead_code)]
    to: String,
    trans_result: Vec<TranslateResult>,
}
