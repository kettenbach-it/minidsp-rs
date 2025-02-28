///! Main entrypoint
/// Launches the application by instantiating all components
use std::{
    collections::HashSet,
    net::IpAddr,
    path::{Path, PathBuf},
    str::FromStr,
};

use anyhow::{Context, Result};
use clap::{ArgAction, Parser};
use confy::load_path;
use minidsp::utils::OwnedJoinHandle;
use once_cell::sync::OnceCell;
use tokio::sync::RwLock;

use crate::{config::Config, device_manager::DeviceManager, discovery::Registry};

pub mod config;
pub mod device_manager;
pub mod discovery;
pub mod http;
pub mod tcp;

static APP: OnceCell<RwLock<App>> = OnceCell::new();

#[derive(Clone, Parser, Debug, Default)]
#[clap(version=env!("CARGO_PKG_VERSION"), author=env!("CARGO_PKG_AUTHORS"))]
pub struct Opts {
    /// Read config file from path
    #[clap(short, long)]
    config: Option<String>,

    /// Verbosity level. -v display decoded commands and responses -vv display decoded commands including readfloats -vvv display hex data frames
    #[clap(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// Log commands and responses to a file
    #[clap(long, env = "MINIDSP_LOG")]
    log: Option<PathBuf>,

    /// Bind address for the TCP server component
    #[clap(default_value = "0.0.0.0:5333")]
    bind_address: String,

    /// If set, advertises the TCP component so it's discoverable from minidsp apps, using the given device name
    #[clap(long)]
    advertise: Option<String>,

    /// IP to use when advertising, required if --advertise is set
    #[clap(long)]
    ip: Option<String>,
}

pub struct App {
    opts: Opts,
    config: Config,
    #[allow(dead_code)]
    device_manager: Option<DeviceManager>,
    #[allow(dead_code)]
    handles: Vec<OwnedJoinHandle<Result<(), anyhow::Error>>>,
}

impl App {
    pub fn new(opts: Opts, config: Config) -> RwLock<Self> {
        RwLock::new(Self {
            device_manager: None,
            handles: Vec::new(),
            opts,
            config,
        })
    }

    pub fn start(&mut self) {
        let registry = Registry::new();

        // If we're advertising a device, make sure to avoid discovering ourselves
        let our_ips: HashSet<IpAddr> = self
            .config
            .tcp_servers
            .iter()
            .filter_map(|s| {
                s.advertise
                    .as_ref()
                    .and_then(|a| IpAddr::from_str(&a.ip).ok())
            })
            .collect();

        let device_mgr = DeviceManager::new(registry, our_ips, self.config.ignore_advertisements);

        let http_server = self.config.http_server.clone();
        self.handles.push(
            tokio::spawn(async move {
                http::main(http_server).await?;
                Ok(())
            })
            .into(),
        );

        for server in &self.config.tcp_servers {
            let server = server.clone();
            self.handles.push(
                tokio::spawn(async move {
                    tcp::main(server).await?;
                    Ok(())
                })
                .into(),
            );
        }

        for static_device in &self.config.static_devices {
            device_mgr.register_static(&static_device.url);
        }

        self.device_manager.replace(device_mgr);
    }

    fn load_config(path: Option<impl AsRef<Path>>) -> Result<Config, confy::ConfyError> {
        match path {
            None => Ok(Config::default()),
            Some(path) => load_path(path),
        }
    }
}

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opts: Opts = Opts::parse();
    let config: Config =
        App::load_config(opts.config.as_ref()).context("cannot load configuration file")?;

    let app = App::new(opts, config);
    APP.set(app).ok().unwrap();

    {
        let mut app_mut = APP.get().unwrap().try_write().unwrap();
        app_mut.start();
    }

    std::future::pending().await
}
