use std::path::PathBuf;
use async_nats::ConnectOptions;
use clap;

use crate::{upload, server};

pub fn mk_cmd() -> clap::Command {
    clap::Command::new("http-nats-obj")
        .arg(clap::Arg::new("creds").short('c').long("creds").required(true).value_parser(clap::value_parser!(PathBuf)))
        .arg(clap::Arg::new("nats_addr").short('n').long("nats").required(true))
        .arg(clap::Arg::new("bucket").short('b').long("bucket").required(true))
        .subcommand_required(true)
        .subcommand(clap::Command::new("upload")
            .arg(clap::Arg::new("dir").short('d').long("dir").required(true).value_parser(clap::value_parser!(PathBuf)))
            .arg(clap::Arg::new("force").short('f').long("force").action(clap::ArgAction::SetTrue)))
        .subcommand(clap::Command::new("serve")
            .arg(clap::Arg::new("hostname").short('H').long("hostname").default_value("0.0.0.0"))
            .arg(clap::Arg::new("port").short('p').long("port").value_parser(clap::value_parser!(u16)).default_value("8000")))
}

pub async fn handle(matches: clap::ArgMatches) {
    let creds_path = matches.get_one::<std::path::PathBuf>("creds").unwrap().clone();
    let nats_addr = matches.get_one::<String>("nats_addr").unwrap().clone();
    let bucket = matches.get_one::<String>("bucket").unwrap().clone();
    match matches.subcommand() {
        Some(("upload", matches)) => handle_upload(matches.clone(), creds_path, nats_addr, bucket).await,
        Some(("serve", matches)) => handle_serve(matches.clone(), creds_path, nats_addr, bucket).await,
        _ => unreachable!("subcommand required")
    }
}

async fn handle_upload(matches: clap::ArgMatches, creds_path: PathBuf, nats_addr: String, bucket: String) {
    let dir = matches.get_one::<std::path::PathBuf>("dir").unwrap().clone();
    let force = matches.get_flag("force");
    let conn_opts = ConnectOptions::with_credentials_file(creds_path).await.expect("Bad creds file");
    dir.as_os_str().to_str().expect("Bad dir");
    upload::upload(
        dir.as_os_str().to_str().expect("Bad dir"),
        nats_addr.as_str(),
        conn_opts, 
        bucket.as_str(), force).await.expect("upload failed");
}

async fn handle_serve(matches: clap::ArgMatches, creds_path: PathBuf, nats_addr: String, bucket: String) {
    let hostname = matches.get_one::<String>("hostname").unwrap().clone();
    let port = matches.get_one::<u16>("port").unwrap().clone();
    let conn_opts = ConnectOptions::with_credentials_file(creds_path).await.expect("Bad creds file");

    server::server(hostname.as_str(), port, nats_addr.as_str(), conn_opts, bucket.as_str()).await.expect("server failed");
}