use std::process::exit;
use tokio::net::TcpListener;

mod args;
mod db;
mod launch;
mod proxy;
mod responses;

const PAYLOAD_MAX_LENGTH: usize = 16384;
const HEADER_MAX_LENGTH: usize = 16384;

#[tokio::main]
async fn main() {
    db::create_passwords()
        .await
        .expect("Failed to add the table 'passwords' in your database");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", args::ARGS.port))
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Cant open port {}, make sure nothing else is using this",
                args::ARGS.port
            )
        });

    tokio::spawn(async {
        let _ = launch::launch().await;
        println!("Make sure that your configuration and right and should be working. The server has exited");
        exit(1);
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
    println!("        TCP - {}", args::ARGS.port);
    println!(
        "        UDP - {}-{}",
        args::ARGS.ko_min_port,
        args::ARGS.ko_max_port
    );
    println!("        **NOT** TCP - {}", args::ARGS.ko_port.unwrap());

    loop {
        if let Ok((socket, _)) = listener.accept().await {
            tokio::spawn(async { proxy::handle_request(socket).await });
        }
    }
}
