use tonic::transport::Channel;
use hello::greeter_client::GreeterClient;
use hello::HelloRequest;

pub mod hello {
    tonic::include_proto!("helloworld"); // This must match the `package` in your .proto
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    // Create a request
    let request = tonic::Request::new(HelloRequest {
        name: "itz_klasic".into(),
    });

    // Send the request
    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response.into_inner().message);

    Ok(())
}
