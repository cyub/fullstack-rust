use std::{
    cell::Cell,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use actix_web::{
    error::{InternalError, JsonPayloadError},
    get, middleware, post,
    web::{self, Data},
    App, Error, HttpRequest, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct MessageApp {
    port: u16,
}
const LOG_FORMAT: &'static str = r#""%r" %s %b "%{User-Agent}i" %D"#;
static SERVER_COUNTER: AtomicUsize = AtomicUsize::new(0);
struct AppState {
    server_id: usize,
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);

        let messages = Arc::new(Mutex::new(vec![]));
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(AppState {
                    server_id: SERVER_COUNTER.fetch_add(1, Ordering::SeqCst),
                    request_count: Cell::new(0),
                    messages: messages.clone(),
                }))
                .wrap(middleware::Logger::new(LOG_FORMAT))
                .service(index)
                .service(
                    web::resource("/send")
                        .app_data(
                            web::JsonConfig::default()
                                .limit(4096)
                                .error_handler(post_error),
                        )
                        .route(web::post().to(post)),
                )
                .service(clear)
                .service(lookup)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
        .await
    }
}

#[derive(Serialize)]
struct IndexResponse {
    server_id: usize,
    request_count: usize,
    message: Vec<String>,
}

#[get("/")]
async fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count: request_count,
        message: ms.clone(),
    }))
}

#[derive(Deserialize)]
struct PostInput {
    message: String,
}

#[derive(Serialize)]
struct PostResponse {
    server_id: usize,
    request_count: usize,
    message: String,
}

async fn post(
    msg: web::Json<PostInput>,
    state: web::Data<AppState>,
) -> Result<web::Json<PostResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.push(msg.message.clone());
    Ok(web::Json(PostResponse {
        server_id: state.server_id,
        request_count: request_count,
        message: msg.message.clone(),
    }))
}

#[post("/clear")]
async fn clear(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let mut ms = state.messages.lock().unwrap();
    ms.clear();
    Ok(web::Json(IndexResponse {
        server_id: state.server_id,
        request_count: request_count,
        message: vec![],
    }))
}

#[derive(Serialize)]
struct PostError {
    server_id: usize,
    request_count: usize,
    error: String,
}

fn post_error(err: JsonPayloadError, req: &HttpRequest) -> Error {
    let state = req.app_data::<web::Data<AppState>>().unwrap();
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let post_error = PostError {
        server_id: state.server_id,
        request_count: request_count,
        error: format!("{}", err),
    };
    InternalError::from_response(err, HttpResponse::BadRequest().json(post_error)).into()
}

#[derive(Serialize)]
struct LookupResonse {
    server_id: usize,
    request_count: usize,
    result: Option<String>,
}

#[get("/lookup/{index}")]
async fn lookup(
    state: web::Data<AppState>,
    idx: web::Path<usize>,
) -> Result<web::Json<LookupResonse>> {
    let request_count = state.request_count.get() + 1;
    state.request_count.set(request_count);
    let ms = state.messages.lock().unwrap();
    println!("idx: {}", idx);
    let result = ms.get(idx.into_inner()).cloned();
    Ok(web::Json(LookupResonse {
        server_id: state.server_id,
        request_count: request_count,
        result,
    }))
}
