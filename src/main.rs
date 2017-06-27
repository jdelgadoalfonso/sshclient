extern crate futures;
extern crate thrussh;

use futures::Future;
use thrussh::*;
use std::rc::Rc;


struct Client { }

impl client::Handler for Client {
    type Error = ();
    type FutureBool = futures::Finished<(Self, bool), Self::Error>;
    type FutureUnit = futures::Finished<Self, Self::Error>;
    type SessionUnit = futures::Finished<(Self, client::Session), Self::Error>;

    fn check_server_key(self, server_public_key: &key::PublicKey) -> Self::FutureBool {
        println!("check_server_key: {:?}", server_public_key);
        futures::finished((self, true))
    }

    fn channel_open_confirmation(self, channel: ChannelId, session: client::Session) -> Self::SessionUnit {
        println!("channel_open_confirmation: {:?}", channel);
        futures::finished((self, session))
    }

    fn data(self, channel: ChannelId, ext: Option<u32>, data: &[u8], session: client::Session) -> Self::SessionUnit {
        println!("data on channel {:?} {:?}: {:?}", ext, channel, std::str::from_utf8(data));
        futures::finished((self, session))
    }
}

impl Client {
    fn run(self, config: Rc<client::Config>, addr: &str) {

        client::connect(addr, config, None, self, |connection| {

            let password = "pepe".to_owned();

            connection.authenticate_password("jose", password).and_then(|session| {

                session.channel_open_session().and_then(|(session, channelid)| {

                    session.data(channelid, None, "Hello, world!").and_then(|(mut session, _)| {
                        session.disconnect(Disconnect::ByApplication, "Ciao", "");
                        futures::finished(session)
                    })
                })
            })
        }).unwrap();
    }
}

fn main() {
    let mut config = thrussh::client::Config::default();
    config.connection_timeout = Some(std::time::Duration::from_secs(600));
    let config = Rc::new(config);
    let sh = Client {};
    sh.run(config, "127.0.0.1:22");
}