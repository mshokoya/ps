use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Proxy {
    _id: String,
    proxy: String,
    protocol: String,
    host: String,
    port: String,
}

pub struct ProxyArg {
    _id: Option<String>,
    proxy: Option<String>,
    protocol: Option<String>,
    host: Option<String>,
    port: Option<String>,
}

impl ProxyArg {
    pub fn is_valid(&mut self) -> bool {
        if self.proxy.is_none() {
            return false;
        }
        true
    }
    pub fn fmt_insert(&mut self) -> &ProxyArg {
        if self.protocol.is_none() {
            self.protocol = None;
        }

        if self.host.is_none() {
            self.host = None;
        }

        if self.port.is_none() {
            self.port = None;
        }

        self
    }
}
