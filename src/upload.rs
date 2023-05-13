use async_recursion::async_recursion;
use futures::StreamExt;
use std::path::PathBuf;

use async_nats::{jetstream::object_store::ObjectStore, ConnectOptions};

async fn upload_file(main_path: PathBuf, path: PathBuf, obj: &ObjectStore) -> Result<(), ()> {
    let mut file = tokio::fs::File::open(&path).await.unwrap();
    //subtract main_path from path
    let path_local = path.strip_prefix(main_path).unwrap();

    obj.put(path_local.to_str().unwrap(), &mut file)
        .await
        .expect(format!("unable to upload file: {}", path.to_str().unwrap()).as_str());
    Ok(())
}

#[async_recursion]
async fn upload_dir(main_path: PathBuf, dir: PathBuf, obj: &ObjectStore) -> Result<(), ()> {
    let mut entries = tokio::fs::read_dir(dir).await.unwrap();
    let mut awaits = Vec::new();
    while let Some(entry) = entries.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_dir() {
            let path_cl = path.clone();
            upload_dir(main_path.clone(), path_cl, obj).await?;

            // upload_dir(&path, obj).await?;
        } else {
            let path_cl = path.clone();
            awaits.push(upload_file(main_path.clone(), path_cl, obj));
        }
    }
    // return Ok(awaits);
    futures::future::join_all(awaits).await;
    Ok(())
}

pub async fn upload(
    path: &str,
    nats_addr: &str,
    nats_conn_op: ConnectOptions,
    obj_bucket_name: &str,
    force: bool,
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

    let mut items = obj.list().await.unwrap();
    while let Some(Ok(object)) = items.next().await {
        println!("object {:?}", object);
        if force {
            obj.delete(&object.name).await.unwrap();
        }
        else {
            println!("bucket is not empty, re run with --force");
        }
        println!("object {:?}", object.name);
    }

    let path = PathBuf::from(path);
    if path.is_dir() {
        upload_dir(path.clone(), path.clone(), &obj)
            .await
            .expect("Could not upload directory");
        Ok(())
    } else {
        let error_message = format!("{} is not a directory", path.to_str().unwrap());
        let error = Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            error_message,
        ));
        Err(error)
    }
}
