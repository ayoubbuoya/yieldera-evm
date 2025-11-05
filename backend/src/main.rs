use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the http server
    println!("Starting Http Server at http://127.0.0.1:8090");
    println!("Starting SWAGGER Server at http://127.0.0.1:8090/swagger-ui/");

    let app_state = web::Data::new(()); // Placeholder for shared application state

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        let (app, app_api) = App::new()
            .wrap(cors)
            .into_utoipa_app()
            .app_data(web::Data::clone(&app_state))
            .split_for_parts();

        app.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", app_api))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
