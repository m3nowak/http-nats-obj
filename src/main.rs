use warp::{Filter, path::FullPath, hyper::StatusCode};

async fn file_lookup(path: &str) -> String{
    //format!("Hello, {}!", path)
    String::from(path)
}

async fn dyn_reply(word: FullPath) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let expanded = word.as_str();
    if expanded == "/hello" {
        Ok(Box::new("world"))
    } else {
        Ok(Box::new(StatusCode::NOT_FOUND))
    }
}

#[tokio::main]
async fn main() {
    let any_redirect2 = warp::path::full().and_then(dyn_reply);

    warp::serve(any_redirect2)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// use async_nats;
// use futures::StreamExt;

// #[tokio::main]
// async fn main() -> Result<(), async_nats::Error> {
//     let client = async_nats::ConnectOptions::with_credentials_file("obj.creds".into()).await?
//         .connect("localhost").await?;
//     let mut subscriber = client.subscribe("messages".into()).await?.take(10);

//     for _ in 0..10 {
//         client.publish("messages".into(), "data".into()).await?;
//     }

//     while let Some(message) = subscriber.next().await {
//       println!("Received message {:?}", message);
//     }

//     Ok(())
// }