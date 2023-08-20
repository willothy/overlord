use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Result};

use dashmap::DashMap;
use proto::server::{
    server_service_server::{self, ServerServiceServer},
    AttachSessionRequest, AttachSessionResponse, DetachSessionRequest, DetachSessionResponse,
    InputEvent, InputEventResponse, KillSessionRequest, KillSessionResponse, ShutdownRequest,
    ShutdownResponse, StartSessionRequest, StartSessionResponse,
};
use overlord_proto as proto;

use tokio::{
    fs,
    net::UnixListener,
    signal::unix::{signal, SignalKind},
    sync::{
        broadcast,
        mpsc::{self, UnboundedSender},
        oneshot, OnceCell,
    },
};
use tokio_stream::wrappers::{UnboundedReceiverStream, UnixListenerStream};
use tonic::{transport::Server, Request, Response, Status};

async fn runtime_dir() -> Result<PathBuf> {
    let Some(runtime_dir) = dirs::runtime_dir().map(|dir| dir.join("rx")) else {
        bail!("Could not find runtime directory");
    };

    if !runtime_dir.exists() {
        fs::create_dir_all(&runtime_dir).await?;
    }

    Ok(runtime_dir)
}

async fn sock_path() -> Result<PathBuf> {
    Ok(runtime_dir().await?.join("rx.sock"))
}

pub type SessionId = u64;
pub type ProcessId = u64;

pub struct Process {
    executable: String,
    args: Vec<String>,
    pid: ProcessId,
    stdin: UnboundedSender<u8>,
    stdout: UnboundedReceiverStream<u8>,
    stderr: UnboundedReceiverStream<u8>,
}

struct Session {
    name: String,
    id: SessionId,
    processes: DashMap<u64, Process>,
    // attached: Option<Arc<Mutex<RxClient>>>,
}

impl Session {
    pub fn new(name: impl Into<String>, id: SessionId) -> Self {
        Self {
            id,
            name: name.into(),
            processes: DashMap::new(),
        }
    }
}

struct RxServer {
    sessions: DashMap<String, Session>,
    session_ids: DashMap<SessionId, String>,
    exit_tx: broadcast::Sender<()>,
}

impl RxServer {
    pub fn new(exit_tx: broadcast::Sender<()>) -> Self {
        Self {
            sessions: DashMap::new(),
            session_ids: DashMap::new(),
            exit_tx,
        }
    }
}

#[tonic::async_trait]
impl server_service_server::ServerService for RxServer {
    async fn handle_input_event(
        &self,
        request: Request<InputEvent>,
    ) -> Result<Response<InputEventResponse>, Status> {
        todo!()
    }

    async fn start_session(
        &self,
        request: Request<StartSessionRequest>,
    ) -> Result<Response<StartSessionResponse>, Status> {
        todo!()
    }

    async fn attach_session(
        &self,
        request: Request<AttachSessionRequest>,
    ) -> Result<Response<AttachSessionResponse>, Status> {
        todo!()
    }

    async fn detach_session(
        &self,
        request: Request<DetachSessionRequest>,
    ) -> Result<Response<DetachSessionResponse>, Status> {
        todo!()
    }

    async fn kill_session(
        &self,
        request: Request<KillSessionRequest>,
    ) -> Result<Response<KillSessionResponse>, Status> {
        todo!()
    }

    async fn shutdown_server(
        &self,
        request: Request<ShutdownRequest>,
    ) -> Result<Response<ShutdownResponse>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let sock_path = sock_path().await?;

    let socket = UnixListener::bind(&sock_path)?;
    let stream = UnixListenerStream::new(socket);

    let (tx, mut rx) = broadcast::channel(1);

    Server::builder()
        .add_service(ServerServiceServer::new(RxServer::new(tx)))
        .serve_with_incoming_shutdown(stream, {
            let mut sigint = signal(SignalKind::interrupt())?;
            let mut sigquit = signal(SignalKind::quit())?;

            async move {
                tokio::select! {
                    _ = sigint.recv() => (),
                    _ = sigquit.recv() => (),
                    _ = rx.recv() => (),
                }
            }
        })
        .await?;

    fs::remove_file(&sock_path).await?;

    write!(std::io::stdout(), "\r").ok();

    Ok(())
}
