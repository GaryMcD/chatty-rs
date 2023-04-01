use actix_web::{
    web::Data,
    App, HttpResponse, HttpServer, Responder,
    get, post} ;
use actix_cors::Cors;
use serde::{Serialize, Serializer};
use shared::Message;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub(crate) struct ChatHistory {
    pub chat_history: Arc<RwLock<Vec<Message>>>
}

impl Serialize for ChatHistory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let chat_history = self.chat_history.read().unwrap();
        chat_history.serialize(serializer)
    }
}

#[get("/")]
async fn get_chat(chat: Data<ChatHistory>) -> impl Responder {
    let chat_history = chat.chat_history.read().unwrap();
    HttpResponse::Ok().json(&*chat_history)
}

#[post("/")]
async fn add_message(request_body: String, chat: Data<ChatHistory>,) -> impl Responder {
    println!("{:?}", request_body);
    // Deserialize the JSON payload into a Message struct
    match serde_json::from_str::<Message>(&request_body) {
        Ok(new_message) => {
            let mut chat_history = chat.chat_history.write().unwrap();
            chat_history.push(new_message);
            HttpResponse::Created().finish()
        }
        Err(e) => {
            eprintln!("Failed to deserialize JSON: {:?}", e);
            HttpResponse::BadRequest().body("Invalid JSON payload")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat = ChatHistory {
        chat_history: Arc::new(RwLock::new(vec![]))
    };

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://127.0.0.1:8080");
        App::new()
            .app_data(Data::new(chat.clone()))
            .wrap(cors)
            .service(get_chat)
            .service(add_message)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}