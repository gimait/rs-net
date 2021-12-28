use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:30051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: String::from("Rust"),
    });

    let response = client.say_hello(request).await?.into_inner();

    println!("RESPONSE={:?}", response);

    Ok(())
}
