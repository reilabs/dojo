mod oracle;

use axum::{
    extract,
    routing::post,
    Router,
    Json,
};
use num::{BigInt, Num};
use stark_vrf::{generate_public_key, BaseField, ScalarValue, StarkVRF};
use serde::{Serialize, Deserialize};
use tracing::debug;
use tower_http::trace::TraceLayer;
use oracle::*;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
struct JsonResult {
    result: StarkVrfProof
}

async fn stark_vrf(extract::Json(payload): extract::Json<StarkVrfRequest>) -> Json<JsonResult> {
    debug!("received payload {payload:?}");
    let secret_key = ScalarValue!("190");
    let public_key = generate_public_key(secret_key);
    
    println!("public key {public_key}");

    let seed: Vec<_> = payload.felt252_seed.iter().map(|x| {
        let dec_string = BigInt::from_str_radix(&x[2..], 16).unwrap().to_string();
        println!("seed string {dec_string}");
        BaseField::from_str(&dec_string).unwrap()
    }).collect();

    let ecvrf = StarkVRF::new(public_key).unwrap();
    let proof = ecvrf.prove(&secret_key, seed.as_slice()).unwrap();
    let sqrt_ratio_hint = ecvrf.hash_to_sqrt_ratio_hint(seed.as_slice());

    println!("proof gamma: {}", proof.0);
    println!("proof c: {}", proof.1);
    println!("proof s: {}", proof.2);
    println!("proof verify hint: {}", sqrt_ratio_hint);

    fn format<T: std::fmt::Display>(v: T) -> String {
        let int = BigInt::from_str(&format!("{v}")).unwrap();
        int.to_str_radix(16)
    }

    let result = StarkVrfProof {
        felt252_gamma_x: format(proof.0.x),
        felt252_gamma_y: format(proof.0.y),
        felt252_c: format(proof.1),
        felt252_s: format(proof.2),
        felt252_sqrt_ratio: format(sqrt_ratio_hint),
    };

    println!("result {result:?}");

    //let n = (payload.n as f64).sqrt() as u64;
    Json(JsonResult { result })
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/stark_vrf", post(stark_vrf))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000, port already in use by another process. Change the port or terminate the other process.");
    debug!("Server started on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
