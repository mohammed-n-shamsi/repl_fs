use tonic::{transport::Server, Request, Response, Status};

use repl_fs::lan_fs::{ReplFS, ReplFSServer};
use repl_fs::{Dir, File};

pub mod repl_fs {
    tonic::include_proto!("repl_fs"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct GenFs {}

#[tonic::async_trait]
impl ReplFS for GenFs {
    type ListDirsStream = ReceiverStream<Result<Dir, Status>>;
    async fn list_dirs(&self) -> Result<Response<Self::ListDirsStream>, Status> {
        unimplemented!()
    }

    type ListFilesStream = ReceiverStream<Result<File, Status>>;
    async fn list_files(
        &self,
        request: Request<Dir>,
    ) -> Result<Response<Self::ListFilesStream>, Status> {
        unimplemented!()
    }
}
