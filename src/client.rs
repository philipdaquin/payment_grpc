use payments::payment_client::PaymentClient;
use payments::PaymentRequest;


pub mod payments { 
    tonic::include_proto!("payments");

}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentClient::connect("http://[::1]:50051").await?;
    let mut req = tonic::Request::new(
        PaymentRequest { 
            from: "John Apple".to_string(),
            to: "Steve".to_string(),
            amount: 10
        }

    );
    let resp = client.send_payment(req).await?;
    println!("RESPONSE{resp:?}");
    
    Ok(())
}