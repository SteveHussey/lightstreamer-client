/// Simple class representing a Proxy configuration.
///
/// An instance of this class can be used through `ConnectionOptions.setProxy()` to instruct
/// a `LightstreamerClient` to connect to the Lightstreamer Server passing through a proxy.
///
/// # Parameters
///
/// * `proxy_type`: the proxy type
/// * `host`: the proxy host
/// * `port`: the proxy port
/// * `user`: the user name to be used to validate against the proxy. Optional.
/// * `password`: the password to be used to validate against the proxy. Optional.
#[derive(Debug)]
pub struct Proxy {
    proxy_type: ProxyType,
    host: String,
    port: u16,
    user: Option<String>,
    password: Option<String>,
}

impl Proxy {
    /// Creates a new instance of `Proxy`.
    ///
    /// # Parameters
    ///
    /// * `proxy_type`: the proxy type
    /// * `host`: the proxy host
    /// * `port`: the proxy port
    /// * `user`: the user name to be used to validate against the proxy. Optional.
    /// * `password`: the password to be used to validate against the proxy. Optional.
    pub fn new(
        proxy_type: ProxyType,
        host: String,
        port: u16,
        user: Option<String>,
        password: Option<String>,
    ) -> Self {
        Proxy {
            proxy_type,
            host,
            port,
            user,
            password,
        }
    }

    /// Returns the proxy type.
    pub fn get_proxy_type(&self) -> &ProxyType {
        &self.proxy_type
    }

    /// Returns the proxy host.
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Returns the proxy port.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Returns the proxy user name.
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    /// Returns the proxy password.
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }
}

/// Represents the type of proxy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyType {
    /// HTTP proxy.
    Http,
    /// SOCKS4 proxy.
    Socks4,
    /// SOCKS5 proxy.
    Socks5,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_auth() {
        let proxy = Proxy::new(
            ProxyType::Http,
            "proxy.example.com".to_string(),
            8080,
            Some("user".to_string()),
            Some("pass".to_string()),
        );
        assert_eq!(proxy.get_proxy_type(), &ProxyType::Http);
        assert_eq!(proxy.get_host(), "proxy.example.com");
        assert_eq!(proxy.get_port(), 8080);
        assert_eq!(proxy.get_user().unwrap(), "user");
        assert_eq!(proxy.get_password().unwrap(), "pass");
    }

    #[test]
    fn test_new_without_auth() {
        let proxy = Proxy::new(
            ProxyType::Socks5,
            "socks.example.com".to_string(),
            1080,
            None,
            None,
        );
        assert_eq!(proxy.get_proxy_type(), &ProxyType::Socks5);
        assert_eq!(proxy.get_host(), "socks.example.com");
        assert_eq!(proxy.get_port(), 1080);
        assert!(proxy.get_user().is_none());
        assert!(proxy.get_password().is_none());
    }

    #[test]
    fn test_proxy_port_range_valid() {
        let proxy = Proxy::new(ProxyType::Http, "host".to_string(), 1, None, None);
        assert_eq!(proxy.get_port(), 1);

        let proxy = Proxy::new(ProxyType::Http, "host".to_string(), 65535, None, None);
        assert_eq!(proxy.get_port(), 65535);
    }
}
