use services::{payment_service_client::PaymentServiceClient, PaymentRequest};

pub mod services {
    tonic::include_proto!("services");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?;

    // 1. Create and send the request
    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(),
        amount: 100.0,
    });

    let response = client.process_payment(request).await?;
    
    println!("RESPONSE={:?}", response.into_inner());

    Ok(())
}