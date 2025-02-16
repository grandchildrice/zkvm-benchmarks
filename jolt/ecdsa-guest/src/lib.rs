#![cfg_attr(feature = "guest", no_std)]
#![no_main]

use k256::{
    ecdsa::{signature::Verifier, Signature, VerifyingKey},
    elliptic_curve::sec1::EncodedPoint,
    Secp256k1,
};

const MESSAGE: &[u8] = include_bytes!("../../../helper/ecdsa_signature/message.txt");
const KEY: &[u8] = include_bytes!("../../../helper/ecdsa_signature/verifying_key.txt");
const SIGNATURE: &[u8] = include_bytes!("../../../helper/ecdsa_signature/signature.txt");

#[jolt::provable(stack_size = 10000, memory_size = 100000)]
pub fn ecdsa_verify() -> bool {
    let message = hex::decode(MESSAGE).expect("Failed to decode hex of 'message'");

    let encoded_point = EncodedPoint::<Secp256k1>::from_bytes(
        &hex::decode(KEY).expect("Failed to decode hex of 'verifying_key'"),
    )
    .expect("Invalid encoded verifying_key bytes");
    let verifying_key: VerifyingKey = VerifyingKey::from_encoded_point(&encoded_point)
        .expect("Could not create VerifyingKey from encoded point");

    let bytes = hex::decode(SIGNATURE).expect("Failed to decode hex of 'signature'");
    let signature = Signature::from_slice(&bytes).expect("Invalid signature bytes");

    verifying_key.verify(&message, &signature).is_ok()
}