use serde::{Deserialize, Serialize};

pub type Session = Vec<Peer>;

#[derive(Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "type")]
    offer_type: String,
    sdp: String,
}
#[derive(Serialize, Deserialize)]
pub struct Peer {
    peer_id: String,
    offer: Offer,
}
