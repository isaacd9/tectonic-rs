use std::path::Path;

use tectonic_rs::{tectonic::data_node_server::DataNodeServer, DataNodeImpl};
use tonic::{transport::Server, Request, Response, Status};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let root_dir_path = Path::new("/tmp/tectonic_data_node");

    std::fs::create_dir_all(root_dir_path)?;

    let data_node = DataNodeImpl {
        root_dir: root_dir_path,
    };

    println!("DataNode listening on: {}", addr);

    Server::builder()
        .add_service(DataNodeServer::new(data_node))
        .serve(addr)
        .await?;

    Ok(())
}
