use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct RegisterReq {
    pub username: Box<str>,
    pub pubkey: Box<[u8]>,
}

#[derive(Encode, Decode, Debug)]
pub struct RegisterRes {
    pub commit_id: Box<[u8]>,
}
