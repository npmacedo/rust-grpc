use tonic::{transport::Server, Request, Response, Status};

use payments::payments_server::{Payments, PaymentsServer};
use payments::{PaymentResponse, PaymentRequest};

pub mod payments {
    tonic::include_proto!("payments");
}

#[derive(Debug,Default)]
pub struct PaymentsService{}

#[tonic::async_trait]
impl Payments for PaymentsService{
    async fn send_payment(
        &self, 
        request: Request<PaymentRequest>
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Got a request: {request:?}");

        let req = request.into_inner();

        let reply = PaymentResponse {
            successful: true,
            message: format!("Sent {} to {}.", req.amount, req.to_addr).into()
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let addr = "[::1]:50051".parse()?;

    let service = PaymentsService::default();

    Server::builder()
        .add_service(PaymentsServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
