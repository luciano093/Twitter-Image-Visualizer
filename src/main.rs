use std::net::SocketAddr;
use std::{fs::File, sync::RwLock};
use std::io::Read;

use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use serde::Deserialize;

mod error;
mod twitter_api;

use twitter_api::{fetch_rest_id, get_media, fetch_guest_token};

async fn index() -> impl Responder {
    let mut site = File::open("./static/index.html").unwrap();
    let mut site_html = String::new();
    site.read_to_string(&mut site_html).unwrap();

    HttpResponse::Ok().body(site_html)
}

#[get("/{user}")]
async fn show_user(_path: web::Path<String>) -> impl Responder {
    let mut file = File::open("./static/index.html").unwrap();

    let mut html = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut html).unwrap();

    HttpResponse::Ok().body(html)
}

#[get("/request.js")]
async fn request() -> impl Responder {
    let mut file = File::open("./static/request.js").unwrap();

    let mut js = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut js).unwrap();
    
    HttpResponse::Ok().content_type("application/javascript").body(js)
}

#[get("/sidebar.js")]
async fn sidebar() -> impl Responder {
    let mut file = File::open("./static/sidebar.js").unwrap();

    let mut js = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut js).unwrap();
    
    HttpResponse::Ok().content_type("application/javascript").body(js)
}

#[get("/styles.css")]
async fn styles() -> impl Responder {
    let mut file = File::open("./static/styles.css").unwrap();

    let mut css = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut css).unwrap();
    
    HttpResponse::Ok().body(css)
}

#[derive(Debug, Deserialize)]
struct Query {
    count: usize,
    cursor: Option<String>
}

#[get("/media/{user}")]
async fn get_user_media(
    path: web::Path<String>,
    web::Query(query): web::Query<Query>,
    guest_token: web::Data<RwLock<String>>,
    client: web::Data<reqwest::Client>
) -> impl Responder {
    let user = path.to_string();

    let rest_id = fetch_rest_id(&client, &guest_token.read().unwrap(), &user).await;
    
    // possible fix to possible deadlock?
    let rest_id = match rest_id {
        Ok(rest_id) => rest_id,
        Err(error::Error::TwitterApi(err)) => {
            match err {
                twitter_api::Error::NoUserFound => return "{\"error\":{\"code\":0,\"message\":\"No user was found\"}}".to_string(),
                twitter_api::Error::UserUnavailable => return "{\"error\":{\"code\":1,\"message\":\"User is unavailable\"}}".to_string(),
                twitter_api::Error::BadToken => {
                    *guest_token.write().unwrap() = fetch_guest_token(&client).await.unwrap();

                    fetch_rest_id(&client, &guest_token.read().unwrap(), &user).await.unwrap()
                }
            }
        }
        Err(err) => panic!("{}", err),
    };

    let tweets = get_media(&client, &guest_token.read().unwrap(), &rest_id, query.count, query.cursor).await.unwrap();

    let mut links = Vec::new();

    for tweet in tweets.as_array().unwrap() {
        if let Some(media_array) = tweet.pointer("/content/itemContent/tweet_results/result/legacy/extended_entities/media") {
            for media in media_array.as_array().unwrap() {
                links.push(media["media_url_https"].to_string());
            }
        }
    }

    let mut json = "{\"media\":[".to_string();

    for link in links {
        json += &format!("{},", link);
    }

    // remove trailing comma
    if let Some(',') = json.chars().last() {
        json.pop().unwrap();
    }

    let curson_bottom = tweets[tweets.as_array().unwrap().len() - 1]["content"]["value"].to_string();

    json += &format!("],\"cursor\":{}}}", curson_bottom);

    json
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = reqwest::ClientBuilder::new()
        .pool_max_idle_per_host(4)
        .build()
        .unwrap();
    let client = web::Data::new(client);
    let guest_token = web::Data::new(RwLock::new(fetch_guest_token(&client).await.unwrap()));

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();

    HttpServer::new(move || {
        App::new()
        .service(web::scope("/i/api")
            .app_data(web::Data::clone(&guest_token))
            .app_data(web::Data::clone(&client))
            .service(get_user_media)
        )
        .route("/", web::get().to(index))
        .route("/home", web::get().to(index))
        .service(styles)
        .service(request)
        .service(sidebar)
        .service(show_user)
    })
    .bind(addr)?
    .run()
    .await?;

    Ok(())
}