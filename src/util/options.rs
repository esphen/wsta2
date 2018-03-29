//! The command line options provided to the program
use std::vec::Vec;

#[derive(Clone, Debug)]
pub struct Options {
    /// The verbosity level of the application. Should be a number
    /// between 0 and 4.
    /// * 0:   NO LOGGING
    /// * 1:   ERROR LOGGING
    /// * 2:   DEBUG LOGGING
    /// * 3:   TRACE LOGGING
    /// * >=4: BINARY LOGGING
    pub verbosity: u8,

    /// The WebSocket URL to connect to.
    pub url: String,

    /// Optional: A GET URL to authenticate with before connecting
    /// to the main url.
    pub login_url: String,

    /// When passed, this flag will cause the program to follow
    /// HTTP GET redirection encountered when calling login_url.
    pub follow_redirect: bool,

    /// Echo outgoing frames, as well as the incoming frames. Outgoing
    /// frames will be prefixed with ">".
    pub echo: bool,

    /// Print the headers of any HTTP request when true.
    pub print_headers: bool,

    /// Headers
    pub headers: Vec<String>,

    /// Messages to send after connecting to the server
    pub messages: Vec<String>,

    /// If provided, will specify an interval for wsta to send a ping
    /// frame to the server.
    pub ping_interval: Option<u64>,

    /// The string to send to the server when sending a ping frame.
    pub ping_msg: String,

    /// If provided, will turn the program into a binary mode, reading 255 bytes
    /// at a time and sending frames when the buffer is filled
    pub binary_mode: bool,

    /// Specifies the amount of bytes per frame to send when
    /// sending binary data.
    pub binary_frame_size: String,
}

impl Default for Options {
    fn default() -> Options {
        Options {
            url: String::new(),
            login_url: String::new(),
            follow_redirect: false,
            echo: false,
            verbosity: 0,
            print_headers: false,
            headers: Vec::new(),
            messages: Vec::new(),
            ping_interval: None,
            ping_msg: String::from("ping"),
            binary_mode: false,
            binary_frame_size: String::from("256"),
        }
    }
}

impl Options {
    /// Construct options for iterating over *none* of the symbol kinds.
    pub fn nothing() -> Options {
        Options {
            url: String::new(),
            login_url: String::new(),
            follow_redirect: false,
            echo: false,
            verbosity: 0,
            print_headers: false,
            headers: Vec::new(),
            messages: Vec::new(),
            ping_interval: None,
            ping_msg: String::from("ping"),
            binary_mode: false,
            binary_frame_size: String::from("256"),
        }
    }
}
