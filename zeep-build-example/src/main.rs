use crate::zeep::hello::{HelloEndpointService, SayHelloInputEnvelope};

pub mod zeep;

#[tokio::main]
async fn main() {
    let _example = HelloEndpointService::new(None)
        .say_hello(SayHelloInputEnvelope::default())
        .await;
}
