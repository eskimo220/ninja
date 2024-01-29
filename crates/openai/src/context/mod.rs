pub mod args;
pub mod arkose;
pub mod init;
mod preauth;

use self::preauth::PreauthCookieProvider;
use crate::{
    arkose::funcaptcha::solver::ArkoseSolver, auth::AuthClient, client::ClientRoundRobinBalancer,
};
use reqwest::Client;
use std::sync::OnceLock;

pub const WORKER_DIR: &str = ".ninja";
// Program context
static CTX: OnceLock<Context> = OnceLock::new();

#[macro_export]
macro_rules! with_context {
    ($method:ident) => {{
        crate::context::init::instance().$method()
    }};
    ($method:ident, $($arg:expr),*) => {{
        crate::context::init::instance().$method($($arg),*)
    }};
    () => {
        crate::context::init::instance()
    };
}

pub fn init(args: args::Args) {
    init::init(args);
}

pub struct CfTurnstile {
    pub site_key: String,
    pub secret_key: String,
}

pub struct Context {
    /// Requesting client
    api_client: ClientRoundRobinBalancer,
    /// Requesting oauth client
    auth_client: ClientRoundRobinBalancer,
    /// Requesting arkose client
    arkose_client: ClientRoundRobinBalancer,
    /// arkoselabs solver
    arkose_solver: Option<ArkoseSolver>,
    /// HAR file upload authenticate key
    arkose_har_upload_key: Option<String>,
    /// Enable files proxy
    enable_file_proxy: bool,
    /// Login auth key
    auth_key: Option<String>,
    /// visitor_email_whitelist
    visitor_email_whitelist: Option<Vec<String>>,
    /// Cloudflare Turnstile
    cf_turnstile: Option<CfTurnstile>,
    /// Arkose endpoint
    arkose_endpoint: Option<String>,
    /// Enable Arkose GPT-3.5 experiment
    arkose_gpt3_experiment: bool,
    /// Enable Arkose GPT-3.5 experiment solver
    arkose_gpt3_experiment_solver: bool,
    /// PreAuth cookie cache
    preauth_provider: Option<PreauthCookieProvider>,
}

impl Context {
    /// Get the reqwest client
    pub fn api_client(&self) -> Client {
        self.api_client.next().into()
    }

    /// Get the reqwest auth client
    pub fn auth_client(&self) -> AuthClient {
        self.auth_client.next().into()
    }

    /// Get the reqwest arkose client
    pub fn arkose_client(&self) -> Client {
        self.arkose_client.next().into()
    }

    /// Get the arkoselabs har file upload authenticate key
    pub fn arkose_har_upload_key(&self) -> Option<&str> {
        self.arkose_har_upload_key.as_deref()
    }

    /// Get the arkoselabs solver
    pub fn arkose_solver(&self) -> Option<&ArkoseSolver> {
        self.arkose_solver.as_ref()
    }

    /// Cloudflare Turnstile config
    pub fn cf_turnstile(&self) -> Option<&CfTurnstile> {
        self.cf_turnstile.as_ref()
    }

    /// Arkoselabs endpoint
    pub fn arkose_endpoint(&self) -> Option<&str> {
        self.arkose_endpoint.as_deref()
    }

    /// Login auth key
    pub fn auth_key(&self) -> Option<&str> {
        self.auth_key.as_deref()
    }

    /// Push a preauth cookie
    #[cfg(feature = "preauth")]
    pub fn push_preauth_cookie(&self, value: &str, max_age: Option<u32>) {
        self.preauth_provider
            .as_ref()
            .map(|p| p.push(value, max_age));
    }

    /// Pop a preauth cookie
    #[cfg(feature = "preauth")]
    pub fn pop_preauth_cookie(&self) -> Option<String> {
        self.preauth_provider.as_ref().map(|p| p.get()).flatten()
    }

    /// Get the arkose gpt3 experiment
    pub fn arkose_gpt3_experiment(&self) -> bool {
        self.arkose_gpt3_experiment
    }

    /// Enable file proxy
    pub fn enable_file_proxy(&self) -> bool {
        self.enable_file_proxy
    }

    /// Get the visitor email whitelist
    pub fn visitor_email_whitelist(&self) -> Option<&[String]> {
        self.visitor_email_whitelist.as_deref()
    }

    /// Get the arkose gpt3 experiment solver
    pub fn arkose_gpt3_experiment_solver(&self) -> bool {
        self.arkose_gpt3_experiment_solver
    }
}
