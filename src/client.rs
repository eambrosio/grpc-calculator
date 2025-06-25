use crate::proto::calculator_client::CalculatorClient;

mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://127.0.0.1:50051";
    let mut client = CalculatorClient::connect(url).await?;

    let req = proto::CalculatorRequest { a: 5, b: 4 };
    let request = tonic::Request::new(req);

    let response = client.add(request).await?;

    println!("Response: {:?}", response.get_ref().result);
    Ok(())
}
