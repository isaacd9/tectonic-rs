use tectonic_rs::tectonic::data_node_client::DataNodeClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataNodeClient::connect("http://[::1]:50052").await?;

    let put_request = tonic::Request::new(tectonic_rs::tectonic::PutChunkRequest {
        chunk_id: "mychunk".into(),
        data: "hello world".into(),
    });

    let response = client.put_chunk(put_request).await?;

    println!("RESPONSE={:?}", response);

    let get_request = tonic::Request::new(tectonic_rs::tectonic::GetChunkRequest {
        chunk_id: "mychunk".into(),
        offset: 0,
        length: 0,
    });

    let response = client.get_chunk(get_request).await?;
    println!("RESPONSE={:?}", response);

    println!("UTF8={:?}", std::str::from_utf8(&response.get_ref().data));

    Ok(())
}
