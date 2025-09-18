use temp_backend::create_app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = create_app().await?;

    println!("Server starting on http://localhost:3000");
    println!("API documentation available at http://localhost:3000/api-docs/openapi.json");
    println!("Swagger UI available at http://localhost:3000/swagger-ui");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
