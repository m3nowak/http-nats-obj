use async_nats::{jetstream::object_store::ObjectStore, ConnectOptions};

use mime;

use axum::{
    extract::{Path, State},
    http::header,
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::io::AsyncReadExt;

use std::sync::Arc;

async fn get_file(
    path: String,
    obj_str: Arc<ObjectStore>,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut object = obj_str
        .get(path.as_str())
        .await
        .expect("Error getting object");
    let mut buffer = Vec::new();
    object.read_to_end(&mut buffer).await?;
    Ok(buffer)
}

async fn file_exists(path: String, obj_str: Arc<ObjectStore>) -> bool {
    match obj_str.info(path.as_str()).await {
        Ok(info) => {
            return !info.deleted;
        }
        Err(_) => {
            return false;
        }
    }
}

async fn file_lookup_bare(
    State(state): State<Arc<ObjectStore>>
) -> impl IntoResponse {
    file_lookup("index.html".to_string(), state).await
}

async fn file_lookup_pth(
    Path(path): Path<String>,
    State(state): State<Arc<ObjectStore>>,
) -> impl IntoResponse {
    file_lookup(path, state).await
}
async fn file_lookup(path: String, state: Arc<ObjectStore>) -> impl IntoResponse {
    let mut path_read = path.clone();

    if path_read.ends_with('/') {
        if file_exists(format!("{}index.html", path), state.clone()).await {
            path_read.push_str("index.html");
        } else if file_exists(format!("{}index.htm", path), state.clone()).await {
            path_read.push_str("index.htm");
        }
    }

    let guess = mime_guess::from_path(path_read.clone())
        .first()
        .unwrap_or(mime::APPLICATION_OCTET_STREAM);

    match get_file(path_read.clone(), state).await {
        Ok(buffer) => ([(header::CONTENT_TYPE, guess.to_string())], buffer),
        Err(e) => (
            [(header::CONTENT_TYPE, mime::TEXT.to_string())],
            format!("Error: {}", e).into(),
        ),
    }
}

pub async fn server(
    host: &str,
    port: u16,
    nats_addr: &str,
    nats_conn_op: ConnectOptions,
    obj_bucket_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let nats_client = nats_conn_op
        .connect(nats_addr)
        .await
        .expect("unable to connect to NATS");
    let jetstream = async_nats::jetstream::new(nats_client);

    let obj = jetstream
        .get_object_store(obj_bucket_name)
        .await
        .expect("unable to get object store");
    let ss = Arc::new(obj);
    let app = Router::new()
        .route("/", get(file_lookup_bare))
        .route("/*path", get(file_lookup_pth))
        .with_state(ss);
    axum::Server::bind(&format!("{}:{}", host, port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
