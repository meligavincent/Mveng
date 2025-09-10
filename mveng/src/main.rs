use axum::{
    extract::Multipart,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use utoipa::ToSchema;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, ToSchema)]
struct UploadResponse {
    saved: String,
}

#[derive(ToSchema)]
struct AudioUpload {
    /// The audio file (multipart/form-data)
    file: String,
}

#[utoipa::path(
    post,
    path = "/upload",
    request_body(
        content = String,
        content_type = "multipart/form-data",
        description = "Upload an audio file"
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse)
    )
)]
async fn upload_audio(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        if let Some(filename) = field.file_name().map(|f| f.to_string()) {
            let data = field.bytes().await.unwrap();
            let filepath = format!("./data/{filename}");
            tokio::fs::create_dir_all("./data").await.unwrap();
            tokio::fs::write(&filepath, &data).await.unwrap();
            return Json(UploadResponse { saved: filepath });
        }
    }
    Json(UploadResponse {
        saved: "No file uploaded".to_string(),
    })
}

#[derive(OpenApi)]
#[openapi(paths(upload_audio), components(schemas(AudioUpload, UploadResponse)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_audio))
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    println!("Docs available at http://{}/docs", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}