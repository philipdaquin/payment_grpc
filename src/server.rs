use tonic::{transport::Server, Request, Response, Status};

use payments::payment_server::{Payment, PaymentServer};
use payments::{PaymentResponse, PaymentRequest};

pub mod payments { 
    tonic::include_proto!("payments");
}

#[derive(Debug, Default)]
pub struct PaymentService {}


// This is the payment service server for grpc 
#[tonic::async_trait]
impl Payment for PaymentService {
    async fn send_payment(&self, req: Request<PaymentRequest>) -> Result<Response<PaymentResponse>, Status> {
        println!("Received a request {req:?}");
        let request = req.into_inner();
        let reply = PaymentResponse {
            success: true, 
            message: format!("Sent {{req.amount}} from {{req.from}} to {{req.to}}"),
            errors: false
        };
        Ok(Response::new(reply))
    } 
} 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server_addr = "[::1]:50051".parse()?;
    let payment_service = PaymentService::default();

    Server::builder()
        .add_service(PaymentServer::new(payment_service))
        .serve(server_addr)
        .await?;

    Ok(())
}
