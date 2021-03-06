#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate rprompt;
extern crate stderrlog;
extern crate ws;
extern crate wurl;

mod app;
mod messages;

use messages::{parse_message, Kind};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use rprompt::read_reply;
use ws::CloseCode;
use app::build_app;
use wurl::util::options::{Options, Show};
use wurl::network::ws::connect;

fn main() {
    let mut app = build_app();
    let matches = app.clone().get_matches();
    let mut opts = Options::default();

    opts.echo = matches.is_present("echo");
    opts.print_headers = matches.is_present("include");
    opts.silent = matches.is_present("silent");

    if let Ok(show_control_frames) = value_t!(matches, "show_control_frames", Show) {
        opts.show_control_frames = show_control_frames;
    }

    if let Ok(url) = value_t!(matches, "url", String) {
        opts.url = url;
    }

    if let Ok(headers) = values_t!(matches, "headers", String) {
        opts.headers = headers;
    }

    if opts.url.is_empty() {
        app.print_help().expect("Failed to print help message");
        exit(1);
    }

    stderrlog::new()
        .module(module_path!())
        .quiet(opts.silent)
        .verbosity(matches.occurrences_of("verbose") as usize)
        .init()
        .expect("Failed to instantiate logger");

    info!("Parsed options as: {:?}", opts);

    let sender = connect(opts).unwrap_or_else(|error| {
        eprintln!("Failed to connect to WebSocket server: {}", error);
        exit(1);
    });

    loop {
        match read_reply() {
            Ok(input) => {
                let message = parse_message(input);
                trace!("Message: {:?}", message);

                if let Err(error) = message {
                    eprintln!("Error: {:?}", error.description());
                    continue;
                }

                let message = message.unwrap();

                match message.kind {
                    Kind::Message => sender
                        .send(message.message.expect("Message did not contain a message"))
                        .expect("Failed to send WebSocket message"),
                    Kind::Ping => sender
                        .ping(Vec::new())
                        .expect("Failed to send ping message"),
                    Kind::Pong => sender
                        .pong(Vec::new())
                        .expect("Failed to send ping message"),
                    Kind::Close => sender
                        .close(CloseCode::from(
                            message
                                .code
                                .expect("Close control frame did not containt a cause code"),
                        ))
                        .expect("Failed to send ping message"),
                }
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::UnexpectedEof => {
                    trace!("Encounteded EOF in stdin, sleeping");
                    sleep(Duration::from_secs(1));
                }
                _ => {
                    warn!("Error: {:?}", error);
                    eprintln!("error: {}", error);
                    exit(2);
                }
            },
        }
    }
}
