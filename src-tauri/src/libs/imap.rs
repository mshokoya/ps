use std::{ net::TcpStream, sync::{Arc, Mutex}, time::Duration};

use anyhow::Result;
use async_std::{
  channel::unbounded,
  task::{sleep, spawn, JoinHandle}
};
use imap::{self, Session};
use kanal::{AsyncReceiver, AsyncSender};
use native_tls::TlsStream;
use imap_proto::types::Address;


struct IMAPEnvelope {
  pub date: Option<String>,
  pub subject: Option<String>,
  pub from: Option<Vec<IMAPAddress>>,
  pub sender: Option<Vec<IMAPAddress>>,
  pub to: Option<Vec<IMAPAddress>>,
  pub message_id: Option<String>,
  pub body: Option<String>,
}

pub struct IMAPAddress {
  pub name: Option<String>,
  pub adl: Option<String>,
  pub mailbox: Option<String>,
  pub host: Option<String>,
}

// pub struct Address<'a> {
//   pub name: Option<&'a [u8]>,
//   pub adl: Option<&'a [u8]>,
//   pub mailbox: Option<&'a [u8]>,
//   pub host: Option<&'a [u8]>,
// }

pub struct IMAP {
  pub imap: Option<Arc<Mutex<Session<TlsStream<TcpStream>>>>>,
  pub poll: Option<JoinHandle<()>>,
  pub sender: AsyncSender<IMAPEnvelope>,
  pub receiver: AsyncReceiver<IMAPEnvelope>,
  pub watching: u8
}

impl IMAP {
  pub fn new() -> Self {
    let (sender, receiver) = unbounded::<IMAPEnvelope>(0);
    
    Self {
      imap: None,
      sender,
      receiver,
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

    

    let imap_sender = self.sender.clone();
    let imap_clone = imap_session.clone();
    let poll = spawn(async move {
      let imap_clone = imap_clone.lock().unwrap();
      loop {
        sleep(Duration::from_secs(7)).await;
        // https://blog.logrocket.com/email-crates-for-rust-lettre-and-imap/
        let messages = imap_clone.fetch("1:10", "RFC822").unwrap();
        
        for message in messages.iter() {
          let envelope = message.envelope();
          imap_sender.send(IMAPEnvelope {
            date: envelope.and_then(|e| extract_frm_envelope(e.date)),
            subject: envelope.and_then(|e| extract_frm_envelope(e.subject)),
            from: envelope.and_then(|e| extract_addrs_envelope(e.from)),
            sender: envelope.and_then(|e| extract_addrs_envelope(e.sender)),
            to: envelope.and_then(|e| extract_addrs_envelope(e.to)),
            message_id: envelope.and_then(|e| extract_frm_envelope(e.message_id)),
            body: envelope.and_then(|e| extract_frm_envelope(message.body())),
          }).await;
        }
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

fn extract_frm_envelope(env: Option<&[u8]>) -> Option<String> {
  env.and_then(|d| std::str::from_utf8(d).ok().and_then(|s| Some(s.to_string())))
}

fn extract_addrs_envelope(env: Option<Vec<Address>>) -> Option<Vec<IMAPAddress>> {
  env.and_then(|d| {
    Some(
      d.iter().map(|add| {
        // Cow<&[u8]>
        IMAPAddress { 
          name: add.name.unwrap(),
          adl: add.adl.to_owned(),
          mailbox: add.mailbox.to_owned(),
          host: add.host.to_owned(),
        }
      }).collect::<Vec<IMAPAddress>>()
    )
  })
}

// pub struct Address {
//   pub name: Option<String>,
//   pub adl: Option<String>,
//   pub mailbox: Option<String>,
//   pub host: Option<String>,
// }