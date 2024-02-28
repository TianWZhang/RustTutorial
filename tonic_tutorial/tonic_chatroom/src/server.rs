use std::pin::Pin;
use futures::prelude::*;
use tonic::transport::Server;
use crate::pb::*;
use tracing::{info, warn};
use self::chat_server::{Chat, ChatServer};
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;

const MAX_MESSAGES: usize = 1024;

pub struct ChatService {
    tx: broadcast::Sender<ChatMessage>
}

pub type ChatResult<T> = Result<tonic::Response<T>, tonic::Status>;

#[tonic::async_trait]
impl Chat for ChatService {
    async fn login(&self, request: tonic::Request<LoginRequest>) -> ChatResult<Token> {
        let info = request.into_inner();
        info!("login: {info:?}");
        let token = info.into_token();
        Ok(tonic::Response::new(token))
    }

    async fn send_message(&self, request: tonic::Request<NewChatMessage>) -> ChatResult<SendMessageResponse> {
        let sender = get_username(request.extensions())?;
        let info = request.into_inner();
        info!("send_message: {info:?}");
        let msg = info.into_chat_message(sender);
        // broadcast msg to everyone who is interested
        self.tx.send(msg).unwrap();
        Ok(tonic::Response::new(SendMessageResponse {  }))
    }

    type GetMessagesStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, tonic::Status>> + Send>>;

    async fn get_messages(&self, request: tonic::Request<GetMessagesRequest>) -> ChatResult<Self::GetMessagesStream> {
        let info = request.into_inner();
        info!("get_messages: {info:?}");
        let mut rx = self.tx.subscribe();
        let (sender, receiver) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            // TODO: filter out uninteresting messages
            while let Ok(msg) = rx.recv().await {
                if let Err(_) = sender.send(Ok(msg)) {
                    warn!("Failed to send. Sender might be closed");
                    return;
                }
            }
        });

        let stream = UnboundedReceiverStream::new(receiver);
        Ok(tonic::Response::new(Box::pin(stream)))
    }
}

impl Default for ChatService {
    fn default() -> Self {
        let (tx, _rx) = broadcast::channel(MAX_MESSAGES);
        Self { tx }
    }
}

pub async fn start() {
    let service = ChatServer::with_interceptor(ChatService::default(), check_auth);
    let addr = "0.0.0.0:8080".parse().unwrap();
    info!("listening on http://{}", addr);
    Server::builder()
        .add_service(service)
        .serve(addr)
        .await
        .unwrap();
}

fn check_auth(mut req: tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status> {
    let token = match req.metadata().get("authorization") {
        Some(v) => {
            let data = v.to_str().map_err(|_| tonic::Status::unauthenticated("Invalid token format"))?;
            Token::new(data.strip_prefix("Bearer ").unwrap())
        }
        None => Token::default(),
    };
    req.extensions_mut().insert(token);
    Ok(req)
}

fn get_username(ext: &tonic::Extensions) -> Result<String, tonic::Status> {
    let token = ext.get::<Token>().ok_or(tonic::Status::unauthenticated("No token"))?;
    if token.is_valid() {
        Ok(token.into_username())
    } else {
        Err(tonic::Status::unauthenticated("Invalid token"))
    }
}