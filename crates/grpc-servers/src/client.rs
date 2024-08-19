use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use log::info;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    info!("run gRPC client to :50051");

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
