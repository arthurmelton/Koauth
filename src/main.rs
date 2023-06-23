use std::process::exit;
use tokio::net::TcpListener;

mod args;
mod db;
mod launch;
mod log;
mod proxy;
mod responses;

use args::ARGS;

const PAYLOAD_MAX_LENGTH: usize = 16384;
const HEADER_MAX_LENGTH: usize = 16384;

#[tokio::main]
async fn main() {
    db::create_passwords()
        .await
        .expect("Failed to add the table 'passwords' in your database");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", ARGS.port))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Cant open port {}, make sure nothing else is using this",
                ARGS.port
            )
        });

    tokio::spawn(async {
        let _ = launch::launch().await;
        println!("Make sure that your configuration and right and should be working. The server has exited");
        exit(1);
    });

    #[cfg(feature = "stats")]
    tokio::spawn(async {
        kostats_web::host(
            ARGS.api_port.unwrap(),
            ARGS.ko_db.clone(),
            ARGS.ko_redis.clone(),
        )
        .await;
    });

    println!("                                   +              ");
    println!("                                 .#%.             ");
    println!("                         .==    :#%%.             ");
    println!("                       -*%%#   =%%%%:             ");
    println!("                   .-*%%%%%%..*%%#%%-             ");
    println!("                 -*%%%%%#%%%+#%%++%%=             ");
    println!("              :+%%%%%*+-##%%%%#=-+%%+             ");
    println!("           .=#%%%%#+---+%#*%%#-=-*%%#+====---:    ");
    println!("           *%%%#+-----=%%%-#+-*#-+***####%%#=     ");
    println!("         .=%%%#-------%%%%=-=#%#-===--=*%%=       ");
    println!("       -*%%%%%+------#%%%%######%%*-=*%%#.        ");
    println!("    :*%%%%%%%%------+%%%#+=-------=#%%%%#.        ");
    println!("    -%%%#*=#%#-----=%%*=---------=+==*%%%%:       ");
    println!("     %%%---*%+-----#%+----=======-----+%%%#       ");
    println!("     +%%=--*%-----*%%*++*%+==*%*=+##*++#%%%-      ");
    println!("     .%%*--+#----+%%%+--*#+=+*%#++##=-*%%%%=      ");
    println!("      *%%--=+---=%%%%#++++===------==++%%%%-      ");
    println!("      :%%+------*%%%%+----------------+%%%*       ");
    println!("       #%#-------*%%%%*--------------*%%%#.       ");
    println!("       =%%=--=#*+-=#%%%%*=--------=+#%%%*         ");
    println!("        %%*-=#%%%%#%%%%%%%%#****#%%%%#+.          ");
    println!("        *%%+#%%%+=*%%%+ :=+##%%%#*+-.             ");
    println!("        -%%%%%+.    ::                            ");
    println!("         %%%+.                                    ");
    println!("         =+.                                      ");
    println!();
    println!("    Version: koauth {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("    Port Forwarding:");
    #[cfg(feature = "stats")]
    if ARGS.api_port.unwrap() == 0 {
        println!("        TCP - {}", ARGS.port);
    }
    else {
        println!("        TCP - {}, {}", ARGS.port, ARGS.api_port.unwrap());
    }
    #[cfg(not(feature = "stats"))]
    println!("        TCP - {}", ARGS.port);
    println!("        UDP - {}-{}", ARGS.ko_min_port, ARGS.ko_max_port);
    println!("        **NOT** TCP - {}", ARGS.ko_port.unwrap());
    println!();

    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async { proxy::handle_request(socket).await });
        }
    }
}
