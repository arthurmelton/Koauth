use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream};
use std::str;
use crate::args::ARGS;
use crate::db::{get_password, set_password};
use crate::responses::*;
use crate::{HEADER_MAX_LENGTH, PAYLOAD_MAX_LENGTH};
use serde_json::Value;
use std::io::{Error, ErrorKind};

const AUTH_REQUEST_START: &[u8] = "POST /api/auth HTTP/1.1".as_bytes();

pub async fn handle_request(mut inbound: TcpStream) -> anyhow::Result<()> {
    let mut outbound = TcpStream::connect(format!("127.0.0.1:{}", ARGS.ko_port.unwrap())).await?;

    let mut input = [0; AUTH_REQUEST_START.len()];

    let _ = inbound.peek(&mut input).await?;

    if AUTH_REQUEST_START == input {
        let mut headers = [0; HEADER_MAX_LENGTH];
        let length = inbound.try_read(&mut headers)?;
        let headers_string = str::from_utf8(&headers)?;

        let headers_end = headers_string.find("\r\n\r\n");

        match headers_end {
            Some(headers_end) => {
                let mut content_length = None;
                let headers_only = &headers_string[AUTH_REQUEST_START.len()+2..headers_end].split("\r\n").map(|x| x.split(":").map(|x| x.trim().to_lowercase()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>();

                for i in headers_only {
                    if i.len() != 2 {
                        write(&mut inbound, true, BADLY_FORMATED_HTML.to_string()).await?;
                        return Ok(());
                    }
                    if i.first().unwrap() == "content-length" {
                        content_length = Some(i.last().unwrap().parse::<usize>()?);
                    }
                }

                match content_length {
                    Some(content_length) => {
                        if content_length <= PAYLOAD_MAX_LENGTH {
                            let body_string = if headers_end+content_length > HEADER_MAX_LENGTH {
                                let mut body = Vec::with_capacity(PAYLOAD_MAX_LENGTH-(HEADER_MAX_LENGTH-headers_end));
                                let _ = inbound.try_read(&mut body)?;
                                let body_string = str::from_utf8(&body)?;
                                format!("{}{}", headers_string[headers_end+4..].to_string(), body_string)
                            }
                            else {
                                headers_string[headers_end+4..].to_string()
                            };
                            match serde_json::from_str::<Value>(&body_string.trim_end_matches('\x00')) {
                                Ok(mut json) => {
                                    match json["credentials"]["username"].as_str() {
                                        Some(username) => {
                                            let username = username.to_string();
                                            match json["credentials"]["secret"].as_u64() {
                                                Some(password) => {
                                                    match get_password(username.clone()).await {
                                                        Some(auth_password) => {
                                                            if password != auth_password {
                                                                write(&mut inbound, true, INCORRECT_PASSWORD.to_string()).await?;
                                                                return Ok(());
                                                            }
                                                        },
                                                        None => {
                                                            set_password(username, password).await?
                                                        }
                                                    }
                                                    json["credentials"].as_object_mut().unwrap().remove("secret");
                                                    let body = json.to_string();

                                                    write(&mut outbound, false, format!("POST /api/auth HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)).await?;
                                                },
                                                None => write(&mut inbound, true, NO_PASSWORD_SPECIFIED.to_string()).await?
                                            }
                                        },
                                        None => write(&mut inbound, true, NO_USERNAME_SPECIFIED.to_string()).await?
                                    }
                                },
                                Err(_) => write(&mut inbound, true, BADLY_FORMATED_JSON.to_string()).await?
                            }
                        }
                        else {
                            write(&mut inbound, true, PAYLOAD_TOO_LARGE.to_string()).await?;
                        }
                    },
                    None => write(&mut inbound, true, UNKOWN_LENGTH.to_string()).await?
                }
            }
            None => {
                if length == HEADER_MAX_LENGTH {
                    write(&mut inbound, true, HEADERS_TOO_LARGE.to_string()).await?;
                }
                else {
                    write(&mut &mut inbound, true, BADLY_FORMATED_HTML.to_string()).await?;
                }

            }
        }
    }

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}

pub async fn write(output: &mut TcpStream, exit: bool, write: String) -> anyhow::Result<()> {
    output.writable().await?;
    output.try_write(&mut write.as_bytes())?;

    if exit {
        return Err(Error::new(ErrorKind::Other, "").into());
    }

    Ok(())
}
