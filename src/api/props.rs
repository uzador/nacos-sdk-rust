use std::collections::HashMap;

/// Configures settings for Client.
#[derive(Debug, Clone)]
pub struct ClientProps {
    /// server_addr like 127.0.0.1:8848
    pub(crate) server_addr: String,
    pub(crate) namespace: String,
    /// app_name
    pub(crate) app_name: String,
    /// metadata
    pub(crate) labels: HashMap<String, String>,
    /// client_version
    pub(crate) client_version: String,
    /// auth context
    pub(crate) auth_context: HashMap<String, String>,
}

#[allow(clippy::new_without_default)]
impl ClientProps {
    /// Creates a new `ClientConfig`.
    pub fn new() -> Self {
        let env_project_version = env!("CARGO_PKG_VERSION");
        let client_version = format!("Nacos-Rust-Client:{}", env_project_version);

        ClientProps {
            server_addr: String::from(crate::api::constants::DEFAULT_SERVER_ADDR),
            /// public is "", Should define a more meaningful namespace
            namespace: String::from(""),
            app_name: crate::api::constants::UNKNOWN.to_string(),
            labels: HashMap::default(),
            client_version,
            auth_context: HashMap::default(),
        }
    }

    /// Sets the server addr.
    pub fn server_addr(mut self, server_addr: impl Into<String>) -> Self {
        self.server_addr = server_addr.into();
        self
    }

    /// Sets the namespace.
    pub fn namespace(mut self, namespace: impl Into<String>) -> Self {
        self.namespace = namespace.into();
        self
    }

    /// Sets the app_name.
    pub fn app_name(mut self, app_name: impl Into<String>) -> Self {
        let name = app_name.into();
        self.app_name = name.clone();
        self.labels
            .insert(crate::api::constants::KEY_LABEL_APP_NAME.to_string(), name);
        self
    }

    /// Sets the labels.
    pub fn labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels.extend(labels.into_iter());
        self
    }

    /// Add auth username.
    #[cfg(feature = "auth-by-http")]
    pub fn auth_username(mut self, username: impl Into<String>) -> Self {
        self.auth_context
            .insert(crate::api::plugin::USERNAME.into(), username.into());
        self
    }

    /// Add auth password.
    #[cfg(feature = "auth-by-http")]
    pub fn auth_password(mut self, password: impl Into<String>) -> Self {
        self.auth_context
            .insert(crate::api::plugin::PASSWORD.into(), password.into());
        self
    }

    /// Add auth ext params.
    pub fn auth_ext(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.auth_context.insert(key.into(), val.into());
        self
    }
}
