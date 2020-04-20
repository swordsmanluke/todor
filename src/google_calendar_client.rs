extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret};
use calendar3::CalendarHub;

use self::hyper::Client;
use std::io::Read;
use self::oauth2::{ConsoleApplicationSecret, FlowType, TokenStorage, Token};
use std::fs::File;
use serde_json as json;
use std::path::{PathBuf, Path};

use std::fs;
use std::io;
use std::fmt;
use serde::de::StdError;
use std::error::Error;

pub fn create_gcal_client(auth_file: String) -> Result<CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, JsonTokenStorage, Client>>, Box<dyn Error>> {
    // Read in the auth file and configure ourselves
    let secret = read_secret_file(auth_file.as_str())?;
    let filename = format!("{}", Path::new(&auth_file.clone()).file_stem().unwrap().to_str().unwrap());

    let auth = Authenticator::new(  &secret, DefaultAuthenticatorDelegate,
                                    hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                    JsonTokenStorage {
                                        program_name: filename,
                                        db_dir: "config".to_string(),
                                    }, Some(FlowType::InstalledRedirect(54324)));

    let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new()));

    return Ok(CalendarHub::new(client, auth))
}

fn read_secret_file(auth_file: &str) -> std::result::Result<ApplicationSecret, Box<dyn Error>> {
    // Get an ApplicationSecret instance from a secret file. It contains the `client_id` and
    // `client_secret`, among other things.
    let mut file = File::open(auth_file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let secret = json::from_str::<ConsoleApplicationSecret>(contents.as_str())?.installed.unwrap();
    Ok(secret)
}

// All the below was yanked mostly verbatim from
// https://github.com/Byron/google-apis-rs/blob/master/gen/calendar3-cli
// because GCal's official Rust lib documentation is garbage.

pub struct JsonTokenStorage {
    pub program_name: String,
    pub db_dir: String,
}

impl JsonTokenStorage {
    fn path(&self, scope_hash: u64) -> PathBuf {
        Path::new(&self.db_dir).join(&format!("{}-token-{}.json", self.program_name, scope_hash))
    }
}

impl TokenStorage for JsonTokenStorage {
    type Error = TokenStorageError;

    // NOTE: logging might be interesting, currently we swallow all errors
    fn set(&mut self,
           scope_hash: u64,
           _: &Vec<&str>,
           token: Option<Token>)
           -> std::result::Result<(), TokenStorageError> {
        match token {
            None => {
                match fs::remove_file(self.path(scope_hash)) {
                    Err(err) => match err.kind() {
                        io::ErrorKind::NotFound => Ok(()),
                        _ => Err(TokenStorageError::Io(err)),
                    },
                    Ok(_) => Ok(()),
                }
            }
            Some(token) => {
                match fs::OpenOptions::new().create(true).write(true).truncate(true).open(&self.path(scope_hash)) {
                    Ok(mut f) => {
                        match json::to_writer_pretty(&mut f, &token) {
                            Ok(_) => Ok(()),
                            Err(serde_err) => Err(TokenStorageError::Json(serde_err)),
                        }
                    }
                    Err(io_err) => Err(TokenStorageError::Io(io_err)),
                }
            }
        }
    }

    fn get(&self, scope_hash: u64, _: &Vec<&str>) -> std::result::Result<Option<Token>, TokenStorageError> {
        match fs::File::open(&self.path(scope_hash)) {
            Ok(f) => {
                match json::de::from_reader(f) {
                    Ok(token) => Ok(Some(token)),
                    Err(err) => Err(TokenStorageError::Json(err)),
                }
            }
            Err(io_err) => {
                match io_err.kind() {
                    io::ErrorKind::NotFound => Ok(None),
                    _ => Err(TokenStorageError::Io(io_err)),
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum TokenStorageError {
    Json(json::Error),
    Io(io::Error),
}

impl fmt::Display for TokenStorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match *self {
            TokenStorageError::Json(ref err) => writeln!(f, "Could not serialize secrets: {}", err),
            TokenStorageError::Io(ref err) => writeln!(f, "Failed to write secret token: {}", err),
        }
    }
}

impl StdError for TokenStorageError {
    fn description(&self) -> &str {
        "Failure when getting or setting the token storage"
    }
}