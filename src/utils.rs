use axum::{
    http::{Request, Response, StatusCode, header},
    body::Body,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, errors::ErrorKind};
use serde::Deserialize;
use std::env;
// Your struct for claims; adjust according to your token's claims
#[derive(Debug, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

pub async fn validate_token<B>(
    req: Request<B>,
) -> Result<Request<B>, Response<Body>> {
    // Extract the authorization header
    let auth_header = req.headers().get(header::AUTHORIZATION);

    match auth_header {
        Some(header_value) => {
            let token = header_value.to_str().unwrap();

            // You'll need to fetch or configure your Keycloak's public key here
            // This is just an example public key
            let public_key = env::var("KC_PUBLIC_KEY").unwrap();
            let issuer = env::var("KC_ISSUER").expect("KC_ISSUER must be set");
            let audience = env::var("KC_AUDIENCE").expect("KC_AUDIENCE must be set");
            // Create a validation object specifying the required algorithm
            let mut validation = Validation::new(Algorithm::RS256);
            validation.set_issuer(&[&issuer]);
            validation.set_audience(&[&audience]);
            // Attempt to decode the token
            match decode::<Claims>(
                token,
                &DecodingKey::from_rsa_pem(public_key.as_bytes()).unwrap(),
                &validation,
            ) {
                Ok(token_data) => {
                    println!("Token is valid! {:?}", token_data.claims);
                    Ok(req)
                },
                Err(err) => match *err.kind() {
                    ErrorKind::InvalidToken => Err(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("Invalid token")).unwrap()),
                    ErrorKind::ExpiredSignature => Err(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("Token expired")).unwrap()),
                    _ => Err(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("Invalid request")).unwrap()),
                },
            }
        },
        None => Err(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from("No authorization header found")).unwrap()),
    }
}
