use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: SocketAddr = format!("0.0.0.0:{port}")
        .parse()
        .expect("PORT must produce a valid socket address");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("listener should bind");

    println!("{} listening on http://{addr}", identity::package_name());

    axum::serve(listener, identity::app())
        .await
        .expect("server should run");
}
