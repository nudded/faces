use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use slack_api as slack;
use cached::proc_macro::cached;
use serde::{Serialize};
use std::env;
use actix_files::Files;

#[derive(Serialize, Clone)]
struct SlackUser {
    real_name: String,
    profile_url: String,
    phone: String,
    title: String
}

#[cached(time=60)]
async fn users() -> Vec<SlackUser> {
    let token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN env var not set");
    let client =
        slack::default_client().expect("Could not create slack client, exiting");

    let response = slack::users::list(
        &client,
        &token,
        &slack::users::ListRequest {
            presence: Some(true)
        },
    ).await;

    let mut users: Vec<SlackUser> = Vec::new();
    if let Ok(response) = response {
        if let Some(members) = response.members {
            for member in &members {
                if !member.is_bot.unwrap_or(true) &&
                    member.id.as_ref().unwrap_or(&"".to_string()) != "USLACKBOT" &&
                    !member.is_restricted.unwrap_or(true) &&
                    !member.deleted.unwrap_or(true)
                {
                    let profile = member.profile.as_ref().unwrap();
                    let user = SlackUser {
                        real_name: profile.real_name.as_ref().unwrap().to_owned(),
                        profile_url: profile.image_192.as_ref().unwrap().to_owned(),
                        phone: profile.phone.as_ref().unwrap().to_owned(),
                        title: profile.title.as_ref().unwrap().to_owned()
                    };
                    users.push(user);
                }

            }
        }
    }
    users
}

async fn list_users() -> impl Responder {
    web::Json(users().await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/users.json", web::get().to(list_users))
            .service(
                Files::new("/", "./frontend/dist/"))
        })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
