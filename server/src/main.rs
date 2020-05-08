use std::{error::Error, thread};

use actix_web::{middleware, App, HttpServer};

use common::*;
use server::{config, AppData};

#[actix_rt::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let (data, rx) = AppData::new_with_rx()?;
    let server = HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(middleware::Logger::default())
            .configure(config)
    })
    .bind(server_address().as_ref())?
    .run();
    let server_cloned = server.clone();
    thread::spawn(move || {
        let _ = rx.recv();
        futures::executor::block_on(server_cloned.stop(true));
    });
    server.await?;
    Ok(())
}
