extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
use calendar3::Channel;
use calendar3::{Result, Error};
use std::default::Default;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, MemoryStorage};
use calendar3::CalendarHub;

use crate::schedule_trait::*;
use self::hyper::Client;
use std::panic::resume_unwind;
use std::io::Read;
use self::oauth2::ConsoleApplicationSecret;
use std::fs::File;
use serde_json as json;

struct GoogleScheduler {
    pub secret: ApplicationSecret,
    pub hub: CalendarHub<Client, Authenticator<DefaultAuthenticatorDelegate, MemoryStorage, Client>>
}

impl GoogleScheduler {
    pub fn new(auth_file: &str) -> GoogleScheduler {
        // Read in the auth file and configure ourselves

        // Get an ApplicationSecret instance by some means. It contains the `client_id` and
        // `client_secret`, among other things.
        let mut file = File::open(auth_file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let secret = json::from_str::<ConsoleApplicationSecret>(contents.as_str()).unwrap().installed.unwrap();

        // Instantiate the authenticator. It will choose a suitable authentication flow for you,
        // unless you replace  `None` with the desired Flow.
        // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
        // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
        // retrieve them from storage.
        let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                      hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                                      <MemoryStorage as Default>::default(), None);


        let mut hub = CalendarHub::new(
            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
            auth);

        GoogleScheduler {
            secret,
            hub
        }
    }

    pub fn get_schedule() -> Vec<ScheduledItem> {
        Vec::new()
    }

    pub fn read_something(&mut self) -> String {
        // As the method needs a request, you would usually fill it with the desired information
        // into the respective structure. Some of the parts shown here might not be applicable !
        // Values shown here are possibly random and not representative !
        let mut req = Channel::default();

        // You can configure optional parameters by calling the respective setters at will, and
        // execute the final call using `doit()`.
        // Values shown here are possibly random and not representative !
        println!("About to make request!");
        let result = self.hub.
            calendar_list().
            list().
            max_results(10).
            doit();
        println!("request complete!");

        match result {
            Err(e) => match e {
                // The Error enum provides details about what exactly happened.
                // You can also just use its `Debug`, `Display` or `Error` traits
                Error::HttpError(_)
                |Error::MissingAPIKey
                |Error::MissingToken(_)
                |Error::Cancelled
                |Error::UploadSizeLimitExceeded(_, _)
                |Error::Failure(_)
                |Error::BadRequest(_)
                |Error::FieldClash(_)
                |Error::JsonDecodeError(_, _) => println!("{}", e),
            },
            Ok(res) => println!("Success: {:?}", res),
        }

        String::new()
        // result.unwrap().0.read_to_string(&mut s);
        // s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn making_requests_works() {
        let mut cal = GoogleScheduler::new("config/work_cal.json");

        assert_eq!(cal.read_something(), "1000");
    }
}








