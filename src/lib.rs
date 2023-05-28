use std::fs::{read, File};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::os::unix::prelude::FileExt;
use std::path::Path;

use tonic::{transport::Server, Request, Response, Status};

use tectonic::data_node_server::{DataNode, DataNodeServer};
use tectonic::{
    AppendChunkRequest, AppendChunkResponse, DeleteChunkRequest, DeleteChunkResponse,
    GetChunkRequest, GetChunkResponse, PutChunkRequest, PutChunkResponse,
};

pub mod tectonic {
    tonic::include_proto!("tectonic"); // The string specified here must match the proto package name
}

#[derive(Debug)]
pub struct DataNodeImpl {
    pub root_dir: &'static Path,
}

#[tonic::async_trait]
impl DataNode for DataNodeImpl {
    async fn put_chunk(
        &self,
        request: Request<PutChunkRequest>,
    ) -> Result<Response<PutChunkResponse>, Status> {
        println!("put_chunk: {:?}", request);

        let r = request.into_inner();
        let mut f = File::create(self.root_dir.join(r.chunk_id))?;
        f.write_all(&r.data)?;
        Ok(Response::new(PutChunkResponse::default()))
    }

    async fn get_chunk(
        &self,
        request: Request<GetChunkRequest>,
    ) -> Result<Response<GetChunkResponse>, Status> {
        println!("get_chunk: {:?}", request);

        let r = request.into_inner();
        let f = File::open(self.root_dir.join(r.chunk_id))?;
        let mut reader = BufReader::new(f);
        reader.seek(SeekFrom::Start(r.offset))?;

        let mut buf = vec![];

        if r.length > 0 {
            buf.resize(r.length as usize, 0);
            reader.read_exact(&mut buf)?;
        } else {
            reader.read_to_end(&mut buf)?;
        }

        Ok(Response::new(GetChunkResponse { data: buf }))
    }

    async fn delete_chunk(
        &self,
        request: Request<DeleteChunkRequest>,
    ) -> Result<Response<DeleteChunkResponse>, Status> {
        println!("delete_chunk: {:?}", request);

        let r = request.into_inner();

        std::fs::remove_file(self.root_dir.join(r.chunk_id))?;

        Ok(Response::new(DeleteChunkResponse::default()))
    }

    async fn append_chunk(
        &self,
        request: Request<AppendChunkRequest>,
    ) -> Result<Response<AppendChunkResponse>, Status> {
        println!("append_chunk: {:?}", request);

        let r = request.into_inner();
        let mut f = File::open(self.root_dir.join(r.chunk_id))?;

        f.seek(SeekFrom::End(0))?;
        f.write_all(&r.data)?;

        Ok(Response::new(AppendChunkResponse::default()))
    }
}
