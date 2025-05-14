use std::io;

use axum::Router;
use socketioxide::SocketIo;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

// use crate::websocket_apis::{listen_noti::noti_websocket, newfeeds::newfeeds_websocket};

pub async fn axum_services() -> io::Result<()> {
    let (layer, _io) = SocketIo::new_layer();

    // io.ns("/notification", noti_websocket);
    // io.ns("/newfeeds", newfeeds_websocket);

    let app = Router::new().layer(
        ServiceBuilder::new()
            .layer(CorsLayer::permissive())
            .layer(layer),
    );

    let listener = TcpListener::bind(dotenvy::var("AXUM_ADDR").unwrap()).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
