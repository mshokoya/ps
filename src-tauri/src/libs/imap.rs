use std::{ net::TcpStream, sync::{Arc, Mutex}, time::Duration};

use anyhow::Result;
use async_std::task::{sleep, spawn, JoinHandle};
use imap::{self, Session};
use native_tls::TlsStream;

pub struct IMAP {
  pub imap: Option<Arc<Mutex<Session<TlsStream<TcpStream>>>>>,
  pub poll: Option<JoinHandle<()>>,
  pub watching: u8
}

impl IMAP {
  pub fn new() -> Self {
    Self {
      imap: None,
      poll: None,
      watching: 0
    }
  }

  
  pub fn connect(&mut self) -> Result<()> {
    let domain = "imap.example.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = Arc::new(Mutex::new(client
        .login("me@example.com", "password")
        .map_err(|e| e.0).unwrap()));

    let guard = imap_session.lock().unwrap();
    guard.select("INBOX").unwrap();
    drop(imap_session);

    let imap_clone = imap_session.clone();
    let poll = spawn(async move {
      let imap_clone = imap_clone.lock().unwrap();
      loop {
        sleep(Duration::from_secs(7)).await;
        // https://blog.logrocket.com/email-crates-for-rust-lettre-and-imap/
        let messages = imap_clone.fetch("*", "RFC822").unwrap();

      }
    });

    self.imap = Some(imap_session);
    self.poll = Some(poll);

    Ok(())
  }

  pub fn logout(&mut self) -> Result<()> {
    self.imap.as_mut().unwrap().logout()?;
    self.imap = None;
    Ok(())
  }

}