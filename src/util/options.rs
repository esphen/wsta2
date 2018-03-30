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

    /// Supresses all output except incoming frames
    pub quiet: bool,

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
}

impl Default for Options {
    fn default() -> Options {
        Options {
            url: String::new(),
            login_url: String::new(),
            follow_redirect: false,
            echo: false,
            verbosity: 0,
            quiet: false,
            print_headers: false,
            headers: Vec::new(),
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
            quiet: false,
            print_headers: false,
            headers: Vec::new(),
        }
    }
}
