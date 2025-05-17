use std::io;

use futures_util::StreamExt;
use socketioxide::extract::SocketRef;
use surrealdb::{Notification, RecordId};
use tokio::sync::mpsc;

use crate::structures::{static_vars::DB, websocket_structures::notifications::UserNotification};

use super::utils::jwt_token::extract_token;

pub async fn noti_websocket(socket: SocketRef) {
    match extract_token(&socket.req_parts().headers) {
        Ok(userinfo) => {
            let (tx, mut rx) = mpsc::channel::<UserNotification>(3);

            socket
                .emit(
                    "notification",
                    &fetch_noti_history(&userinfo.id.key().to_string()).await,
                )
                .ok();

            tokio::spawn(live_noti_listener(userinfo.id, tx));

            while let Some(msg) = rx.recv().await {
                socket.emit("notification_history", &msg).ok();
            }
        }
        Err(e) => {
            socket.emit("error", &e).ok();
            socket.disconnect().ok();
        }
    }
}

pub async fn fetch_noti_history(userid: &str) -> Vec<UserNotification> {
    let query = "SELECT * FROM tb_notifications WHERE noti_for_user = $user_id ORDER BY date DESC;";

    let mut result = DB
        .query(query)
        .bind(("user_id", RecordId::from_table_key("tb_users", userid)))
        .await
        .unwrap();

    result.take(0).unwrap()
}

pub async fn live_noti_listener(
    userid: RecordId,
    tx: mpsc::Sender<UserNotification>,
) -> io::Result<()> {
    let query = "LIVE SELECT * FROM tb_notifications WHERE noti_for_user = $user_id";

    let mut result = DB
        .query(query)
        .bind(("user_id", userid.clone()))
        .await
        .unwrap();

    let mut stream = result.stream::<Notification<UserNotification>>(0).unwrap();

    while let Some(result) = stream.next().await {
        let result = result.unwrap();
        tx.send(result.data).await.unwrap();
    }

    Ok(())
}
