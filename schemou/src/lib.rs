use bitcode::{Decode, Encode};

#[derive(Encode, Decode, Debug)]
pub struct RegisterReq {
    pub username: Box<str>,
    pub pubkey: Box<str>,
}

#[derive(Encode, Decode, Debug)]
pub struct RegisterRes {
    pub commit_id: Box<str>,
}
