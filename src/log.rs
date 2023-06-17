#[macro_export]
macro_rules! log {
    ( $( $x:expr ),* ) => {
        {
            use chrono::prelude::*;
            println!("{} - {}", Local::now().to_rfc3339_opts(SecondsFormat::Millis, true), format!($($x,)*))
        }
    };
}
