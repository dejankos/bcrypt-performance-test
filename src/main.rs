use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};

const DEFAULT_COST: u32 = 4;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct HashRequest {
    hashing_algorithm: String,
    plain_text: String,
}

#[derive(Serialize, Deserialize)]
struct HashResponse {
    hash: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MatchHashRequest {
    plain_text: String,
    hash: String,
}

#[derive(Serialize, Deserialize)]
struct MatchHashResponse {
    matches: bool
}

async fn hash_str(h_req: Json<HashRequest>) -> HttpResponse {
    let cost = hash_cost(&h_req.hashing_algorithm);

    match hash(&h_req.plain_text, cost) {
        Ok(hash) => HttpResponse::Ok().json(HashResponse { hash }),
        _ => HttpResponse::InternalServerError().finish()
    }
}


async fn verify_str(h_req: Json<MatchHashRequest>) -> HttpResponse {
    match verify(&h_req.plain_text, &h_req.hash) {
        Ok(matches) => HttpResponse::Ok().json(MatchHashResponse { matches }),
        _ => HttpResponse::InternalServerError().finish()
    }
}

fn hash_cost(hashing_algorithm: &String) -> u32 {
    hashing_algorithm
        .rsplit("-")
        .next()
        .map(|c| c.parse::<u32>().ok().unwrap_or(DEFAULT_COST))
        .unwrap_or(DEFAULT_COST)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(web::JsonConfig::default().limit(1024))
            .service(web::resource("/api/v1/hash").route(web::post().to(hash_str)))
            .service(web::resource("/api/v1/match").route(web::post().to(verify_str)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use actix_web::{App, test, web};
    use actix_web::dev::Service;
    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn should_hash_string() {
        let mut app = test::init_service(
            App::new().service(web::resource("/api/v1/hash").route(web::post().to(hash_str))),
        )
            .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/hash")
            .set_json(&HashRequest {
                hashing_algorithm: "bcrypt-4".to_string(),
                plain_text: "abcd".to_string(),
            })
            .to_request();

        let start = Instant::now();
        let resp = app.call(req).await.unwrap();
        let duration = start.elapsed();
        println!("duration '/api/v1/hash' {:?}", duration);
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn should_verify_string() {
        let mut app = test::init_service(
            App::new().service(web::resource("/api/v1/match").route(web::post().to(verify_str))),
        )
            .await;

        let req = test::TestRequest::post()
            .uri("/api/v1/match")
            .set_json(&MatchHashRequest {
                plain_text: "abba".to_string(),
                hash: "$2b$04$Lk1f9qGCdObNRJXASf2SnO//2jv3e6mf9E8/3IPtW1YtpZ6ffdMgm".to_string(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"{"matches":true}"##);
    }
}

