use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ARGS: Args = {
        let mut args = Args::parse();
        if args.ko_port.is_none() {
            args.ko_port = Some(args.port - 1);
        }
        if args.auth_db.is_none() {
            args.auth_db = Some(args.ko_db.clone());
        }
        if !args.ko_redis.contains(':') {
            args.ko_redis = format!("{}:6379", args.ko_redis);
        }
        args
    };
}

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The port that people will connect to
    #[arg(short, long, default_value_t = 23600)]
    pub port: u16,

    /// The path to the ko server exe
    #[arg(long, default_value_t = String::from("KnockoutCityServer.exe"))]
    pub ko_path: String,

    /// The port that the real game runs on [default: port-1]
    #[arg(long)]
    pub ko_port: Option<u16>,

    /// The minium port that ko will use for udp
    #[arg(long, default_value_t = 23600)]
    pub ko_min_port: u16,

    /// The maximum port that ko will use for udp
    #[arg(long, default_value_t = 23699)]
    pub ko_max_port: u16,

    /// The maximum amount of people able to join the server [default: infinity]
    #[arg(long)]
    pub ko_players: Option<u32>,

    /// The database that ko will use (has to be posgres)
    #[arg(long, default_value_t = String::from("postgresql://127.0.0.1:5432/knockout"))]
    pub ko_db: String,

    /// The redis server that ko will use
    #[arg(long, default_value_t = String::from("127.0.0.1:6379"))]
    pub ko_redis: String,

    /// The pasgres database to store the passwords of users (defaults to ko_db)
    #[arg(short, long)]
    pub auth_db: Option<String>,
}
