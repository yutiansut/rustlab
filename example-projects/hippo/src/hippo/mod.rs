
pub mod config;
pub mod sources;

use self::config::Config;

use std::sync::mpsc;

#[allow(dead_code)]
pub struct Hippo {
    //inputs
    //outputs
    //tasks
    //config

}

pub fn start(config: &Config) {
    
    //build sources
    let (tx, rx): (mpsc::SyncSender<Vec<u8>>, mpsc::Receiver<Vec<u8>>)= mpsc::sync_channel(1024);
    //let arx = Arc::new(Mutex::new(rx));

    let source = config.source.build(tx);

    let sink = config.sink.build(rx);

    sink.start()
}



