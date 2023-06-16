use std::io::{Error, ErrorKind};
use crate::args::ARGS;
use tokio::process::Command;
use std::process::Stdio;
use std::path::PathBuf;

pub async fn launch() -> anyhow::Result<()> {
    let mut args = vec![
        format!("-backend_port={}", ARGS.ko_port.unwrap()),
        format!("-server_min_port={}", ARGS.ko_min_port),
        format!("-server_max_port={}", ARGS.ko_max_port),
        format!("-backend_db={}", ARGS.ko_db),
        format!("-backend_redis_db_host={}", ARGS.ko_redis.split(":").nth(0).unwrap()),
        format!("-backend_redis_db_port={}", ARGS.ko_redis.split(":").nth(1).unwrap()),
    ];
    match ARGS.ko_players {
        Some(x) => args.push(format!("-backend_tunable_user_connections_max_per_backend={x}")),
        None => {}
    }

    let mut working_dir = PathBuf::from(ARGS.ko_path.clone());
    working_dir.pop();

    if cfg!(windows) {
        Command::new(ARGS.ko_path.clone())
            .args(args)
            .current_dir(working_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output().await?;
    } else if cfg!(unix) {
        args.insert(0, ARGS.ko_path.clone());
        Command::new("wine")
            .args(args)
            .current_dir(working_dir)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output().await?;
    }
    else {
        println!("Your device cant run a ko server, if this is a error please report it at https://github.com/AMTitan/koauth");
        return Err(Error::new(ErrorKind::Other, "").into());
    }

    return Err(Error::new(ErrorKind::Other, "").into());
}
