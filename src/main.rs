use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use tera::Tera;

mod services;

use services::{bot_detector, ip_lookup};

/// Renders the login page or denied page based on bot detection.
async fn login_page(
    req: HttpRequest,
    tera: web::Data<Tera>,
) -> impl actix_web::Responder {
    // Extract user agent and IP address from the request
    let user_agent = req
        .headers()
        .get("User-Agent")
        .unwrap_or_default()
        .to_str()
        .unwrap_or("");
    let ip = req
        .connection_info()
        .realip_remote_addr()
        .unwrap_or("127.0.0.1");

    // Get the user's country using GeoLite2-Country
    let country = match ip_lookup::get_country(ip) {
        Ok(country) => country,
        Err(_) => "Unknown".to_string(),
    };

    // Call the Python API for bot detection
    if let Ok(is_bot) = bot_detector::detect_bot(ip, user_agent).await {
        if is_bot {
            // Render denied.html if the request is flagged as a bot
            let mut ctx = tera::Context::new();
            ctx.insert("country", &country);
            let rendered = tera.render("denied.html", &ctx).unwrap();
            return HttpResponse::Ok().body(rendered);
        }
    }

    // Render login.html for legitimate users
    let mut ctx = tera::Context::new();
    ctx.insert("logo", "logo1.png");
    ctx.insert("background", "background.jpg");
    ctx.insert("country", &country);
    let rendered = tera.render("login.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

/// The main entry point for the Actix-Web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the Tera template engine
    let tera = Tera::new("src/templates/**/*").expect("Error initializing Tera templates");

    // Start the Actix-Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone())) // Share Tera instance
            .route("/", web::get().to(login_page))  // Login route
    })
    .bind("127.0.0.1:8080")? // Bind to localhost:8080
    .run()
    .await
}
