use actix::Handler;
use actix::{Actor, ActorContext, Addr, AsyncContext, Message, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
#[derive(Message)]
#[rtype(result = "()")]
pub struct NotificationMessage(pub String);

// WebSocket connection actor
pub struct WebSocketSession {
    // Last ping received from client
    last_heartbeat: Instant,
    // Client ID if authenticated
    client_id: Option<String>,
}

impl WebSocketSession {
    pub fn new() -> Self {
        Self {
            last_heartbeat: Instant::now(),
            client_id: None,
        }
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.last_heartbeat) > CLIENT_TIMEOUT {
                log::info!("WebSocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}
impl Handler<NotificationMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: NotificationMessage, ctx: &mut Self::Context) {
        // Send the message text to the WebSocket client
        ctx.text(msg.0);
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        log::info!("WebSocket connection established");
        self.heartbeat(ctx);
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        log::info!("WebSocket connection terminated");

        // Remove from clients list if authenticated
        if let Some(client_id) = &self.client_id {
            remove_client(client_id);
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.last_heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.last_heartbeat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let result = serde_json::from_str::<serde_json::Value>(&text);
                if let Ok(json) = result {
                    if let Some(event) = json.get("event").and_then(|e| e.as_str()) {
                        match event {
                            "subscribe" => {
                                if let Some(client_id) =
                                    json.get("client_id").and_then(|c| c.as_str())
                                {
                                    self.client_id = Some(client_id.to_string());
                                    register_client(client_id.to_string(), ctx.address());

                                    let response = serde_json::json!({
                                        "status": "ok",
                                        "event": "subscribe"
                                    });
                                    ctx.text(response.to_string());
                                    log::info!("Client {} subscribed to notifications", client_id);
                                }
                            }
                            "unsubscribe" => {
                                if let Some(client_id) = &self.client_id {
                                    remove_client(client_id);
                                    self.client_id = None;

                                    let response = serde_json::json!({
                                        "status": "ok",
                                        "event": "unsubscribe"
                                    });
                                    ctx.text(response.to_string());
                                    log::info!("Client unsubscribed from notifications");
                                }
                            }
                            _ => {
                                log::warn!("Unknown event: {}", event);
                            }
                        }
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {
                log::warn!("Binary messages are not supported");
            }
            Ok(ws::Message::Close(reason)) => {
                log::info!("WebSocket closed: {:?}", reason);
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                log::warn!("Continuation messages are not supported");
            }
            Ok(ws::Message::Nop) => {}
            Err(e) => {
                log::error!("WebSocket protocol error: {}", e);
                ctx.stop();
            }
        }
    }
}

// Global clients registry
lazy_static::lazy_static! {
    static ref CLIENTS: Arc<Mutex<HashMap<String, Addr<WebSocketSession>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// Register a client
fn register_client(client_id: String, addr: Addr<WebSocketSession>) {
    let mut clients = CLIENTS.lock().unwrap();
    clients.insert(client_id, addr);
}

// Remove a client
fn remove_client(client_id: &str) {
    let mut clients = CLIENTS.lock().unwrap();
    clients.remove(client_id);
}

// Send a notification to a specific client
pub fn send_notification_to_client(client_id: &str, notification: serde_json::Value) {
    let clients = CLIENTS.lock().unwrap();
    if let Some(addr) = clients.get(client_id) {
        addr.do_send(NotificationMessage(notification.to_string()));
    }
}

// Send a notification to all clients
pub fn broadcast_notification(notification: serde_json::Value) {
    let clients = CLIENTS.lock().unwrap();
    for (_, addr) in clients.iter() {
        addr.do_send(NotificationMessage(notification.to_string()));
    }
}

// Handler for WebSocket connections
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    log::info!("WebSocket connection request");
    ws::start(WebSocketSession::new(), &req, stream)
}

// Helper function to send a notice
pub fn send_notice(group_id: &str, client_id: &str) {
    let notice = serde_json::json!({
        "type": "notice",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "group_id": group_id,
        "client_id": client_id
    });

    broadcast_notification(notice);
}
