use axum::{
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

// Response model
#[derive(Serialize, ToSchema)]
struct HelloResponse {
    message: String,
    status: String,
}

#[derive(Serialize, ToSchema)]
struct MvengResponse {
    greeting: String,
    wisdom: String,
    from: String,
}

/// Simple hello world endpoint
#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello world response", body = HelloResponse)
    ),
    tag = "greetings"
)]
async fn hello_world() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    })
}

/// Mveng greeting endpoint with African wisdom
#[utoipa::path(
    get,
    path = "/mveng",
    responses(
        (status = 200, description = "Mveng greeting with African wisdom", body = MvengResponse)
    ),
    tag = "greetings"
)]
async fn mveng_greeting() -> Json<MvengResponse> {
    Json(MvengResponse {
        greeting: "🌍 Akwaaba! Welcome, friend!".to_string(),
        wisdom: "Nkukuma nkobe ye, a si nkukuma nda - The storyteller is like a tree, rooted in wisdom".to_string(),
        from: "Mveng, your African storyteller".to_string(),
    })
}

// OpenAPI documentation
#[derive(OpenApi)]
#[openapi(
    paths(hello_world, mveng_greeting),
    components(schemas(HelloResponse, MvengResponse)),
    tags(
        (name = "greetings", description = "Simple greeting endpoints")
    ),
    info(
        title = "Mveng - Hello World API",
        version = "1.0.0",
        description = "🌍 Simple Hello World API for Mveng - African AI Assistant"
    )
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Build our application with routes
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/mveng", get(mveng_greeting))
        // Add Swagger UI
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // Run the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("🌍 Mveng server starting...");
    println!("📖 Swagger UI: http://localhost:3000/swagger-ui");
    println!("🎯 Hello endpoint: http://localhost:3000/hello");
    println!("🌟 Mveng endpoint: http://localhost:3000/mveng");
    
    axum::serve(listener, app).await.unwrap();
}