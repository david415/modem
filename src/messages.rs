// messages.rs - noise wire protocol cryptographic messages
// Copyright (C) 2018  David Anthony Stainton.
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

extern crate snow;
extern crate ecdh_wrapper;
extern crate rustc_serialize;

use self::rustc_serialize::hex::ToHex;
use std::time::SystemTime;
use subtle::ConstantTimeEq;
use byteorder::{ByteOrder, BigEndian};
use snow::params::NoiseParams;
use snow::NoiseBuilder;
use ecdh_wrapper::{PrivateKey, PublicKey};

use super::errors::{HandshakeError, SendMessageError, ReceiveMessageError};
use super::commands::{Command};

const NOISE_PARAMS: &'static str = "Noise_XX_25519_ChaChaPoly_BLAKE2b";
const PROLOGUE: [u8;1] = [0u8;1];
const PROLOGUE_SIZE: usize = 1;
const NOISE_MESSAGE_MAX_SIZE: usize = 65535;
const KEY_SIZE: usize = 32;
const MAC_SIZE: usize = 16;
const MAX_ADDITIONAL_DATA_SIZE: usize = 255;
const AUTH_SIZE: usize = 1 + MAX_ADDITIONAL_DATA_SIZE + 4;
const AUTH_MESSAGE_SIZE: usize = 1 + 4 + MAX_ADDITIONAL_DATA_SIZE;
const NOISE_HANDSHAKE_MESSAGE1_SIZE: usize = PROLOGUE_SIZE + KEY_SIZE;
const NOISE_HANDSHAKE_MESSAGE2_SIZE: usize = 101;
const NOISE_HANDSHAKE_MESSAGE3_SIZE: usize = 64;
const NOISE_MESSAGE_HEADER_SIZE: usize = MAC_SIZE + 4;


struct AuthenticateMessage {
    additional_data: Vec<u8>,
    unix_time: u32,
}

impl AuthenticateMessage {
    fn to_vec(&self) -> Result<Vec<u8>, &'static str> {
        if self.additional_data.len() > MAX_ADDITIONAL_DATA_SIZE {
            return Err("additional data exceeds maximum allowed size");
        }
        let mut out = Vec::new();
        out.push(self.additional_data.len() as u8);
        out.extend_from_slice(&self.additional_data);
        let mut _time = [0u8; 4];
        BigEndian::write_u32(&mut _time, self.unix_time);
        out.extend_from_slice(&_time);
        return Ok(out);
    }
}

fn authenticate_message_from_bytes(b: &[u8]) -> Result<AuthenticateMessage, &'static str> {
    if b.len() != AUTH_MESSAGE_SIZE {
        return Err("authenticate message is not the valid size");
    }
    let ad_len = b[0] as usize;
    return Ok(AuthenticateMessage {
        additional_data: b[1..1+ad_len].to_vec(),
        unix_time: BigEndian::read_u32(&b[1+MAX_ADDITIONAL_DATA_SIZE..]),
    });
}

pub struct PeerCredentials {
    pub additional_data: Vec<u8>,
    pub public_key: PublicKey,
}

pub trait PeerAuthenticator {
    fn is_peer_valid(&self, peer_credentials: &PeerCredentials) -> bool;
}

pub struct SessionConfig {
    pub authenticator: Box<PeerAuthenticator>,
    pub authentication_key: PrivateKey,
    pub peer_public_key: Option<PublicKey>,
    pub additional_data: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum SessionState {
    Init,
    Established,
    Invalid,
}

pub struct Session {
    initiator: bool,
    session: snow::Session,
    additional_data: Vec<u8>,
    authenticator: Box<PeerAuthenticator>,
    authentication_key: PrivateKey,
}

impl Session {
    pub fn new(session_config: SessionConfig, is_initiator: bool) -> Result<Session, HandshakeError> {
        let noise_params: NoiseParams = NOISE_PARAMS.parse().unwrap();
        let noise_builder: NoiseBuilder = NoiseBuilder::new(noise_params);
        let session: snow::Session;
        if is_initiator {
            if !session_config.peer_public_key.is_some() {
                return Err(HandshakeError::NoPeerKeyError);
            }
            let _match = noise_builder
                .local_private_key(&session_config.authentication_key.to_vec())
                .remote_public_key(&(session_config.peer_public_key.unwrap()).to_vec())
                .prologue(&PROLOGUE)
                .build_initiator();
            session = match _match {
                Ok(x) => x,
                Err(_) => return Err(HandshakeError::SessionCreateError),
            };
        } else {
            let _match = noise_builder
                .local_private_key(&session_config.authentication_key.to_vec())
                .prologue(&PROLOGUE)
                .build_responder();
            session = match _match {
                Ok(x) => x,
                Err(_) => return Err(HandshakeError::SessionCreateError),
            };
        }
        let _s = Session {
            initiator: is_initiator,
            additional_data: session_config.additional_data,
            authenticator: session_config.authenticator,
            authentication_key: session_config.authentication_key,
            session: session,
        };
        Ok(_s)
    }

    pub fn client_handshake1(&mut self) -> Result<[u8; NOISE_HANDSHAKE_MESSAGE1_SIZE], HandshakeError> {
        let mut msg = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _match = self.session.write_message(&PROLOGUE, &mut msg);
        let mut _len = match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ClientHandshakeNoise1Error),
        };
        assert_eq!(NOISE_HANDSHAKE_MESSAGE1_SIZE, _len);
        let mut msg1 = [0u8; NOISE_HANDSHAKE_MESSAGE1_SIZE];
        msg1.copy_from_slice(&msg[..NOISE_HANDSHAKE_MESSAGE1_SIZE]);
        return Ok(msg1);
    }

    pub fn client_handshake2(&mut self) -> Result<[u8; NOISE_HANDSHAKE_MESSAGE3_SIZE], HandshakeError> {
        let mut msg = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _match = self.session.write_message(&[], &mut msg);
        let _len = match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ClientHandshakeNoise3Error),
        };
        assert_eq!(NOISE_HANDSHAKE_MESSAGE3_SIZE, _len);
        let mut _msg3 = [0u8; NOISE_HANDSHAKE_MESSAGE3_SIZE];
        _msg3.copy_from_slice(&msg[..NOISE_HANDSHAKE_MESSAGE3_SIZE]);
        return Ok(_msg3);
    }

    pub fn client_read_handshake1(&mut self, message: [u8; NOISE_HANDSHAKE_MESSAGE2_SIZE]) -> Result<(), HandshakeError> {
        let mut _raw_auth = [0u8; AUTH_MESSAGE_SIZE];
        let _match = self.session.read_message(&message, &mut _raw_auth);
        let _len = match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ClientHandshakeNoise2Error),
        };
        let auth_msg = authenticate_message_from_bytes(&_raw_auth).unwrap();
        let raw_peer_key = self.session.get_remote_static().unwrap();
        let mut peer_key = PublicKey::default();
        peer_key.from_bytes(raw_peer_key);
        let peer_credentials = PeerCredentials {
            additional_data: auth_msg.additional_data,
            public_key: peer_key,
        };
        if !self.authenticator.is_peer_valid(&peer_credentials) {
            return Err(HandshakeError::ClientAuthenticationError);
        }
        return Ok(());
    }
    
    pub fn server_read_handshake1(&mut self, message: [u8; NOISE_HANDSHAKE_MESSAGE1_SIZE]) -> Result<(), HandshakeError> {
        if message[NOISE_HANDSHAKE_MESSAGE1_SIZE-1..NOISE_HANDSHAKE_MESSAGE1_SIZE].ct_eq(&PROLOGUE).unwrap_u8() == 0 {
            return Err(HandshakeError::ServerPrologueMismatchError);
        }
        let mut _msg1p = [0u8; NOISE_HANDSHAKE_MESSAGE1_SIZE];
        let _match = self.session.read_message(&message, &mut _msg1p);
        let mut _len = match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ServerHandshakeNoise1Error),
        };
        assert_eq!(PROLOGUE_SIZE, _len);
        return Ok(());
    }

    pub fn server_handshake1(&mut self) -> Result<[u8; NOISE_HANDSHAKE_MESSAGE2_SIZE], HandshakeError> {
        let now = SystemTime::now();
        let our_auth = AuthenticateMessage {
            additional_data: self.additional_data.clone(),
            unix_time: now.elapsed().unwrap().as_secs() as u32,
        };
        let raw_auth = our_auth.to_vec().unwrap();
        let mut _msg2 = [0u8; NOISE_HANDSHAKE_MESSAGE2_SIZE];
        let _match = self.session.write_message(&raw_auth, &mut _msg2);
        let mut _len = match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ServerHandshakeNoise2Error),
        };
        assert_eq!(NOISE_HANDSHAKE_MESSAGE2_SIZE, _len);
        return Ok(_msg2);
    }

    pub fn server_read_handshake2(&mut self, message: [u8; NOISE_HANDSHAKE_MESSAGE3_SIZE]) -> Result<(), HandshakeError> {
        let mut raw_auth = [0u8; AUTH_MESSAGE_SIZE];
        let _match = self.session.read_message(&message, &mut raw_auth);
        match _match {
            Ok(x) => x,
            Err(_) => return Err(HandshakeError::ServerHandshakeNoise3Error),
        };
        let peer_auth = authenticate_message_from_bytes(&raw_auth).unwrap();
        let raw_peer_key = self.session.get_remote_static().unwrap();
        let mut peer_key = PublicKey::default();
        peer_key.from_bytes(raw_peer_key);
        let peer_credentials = PeerCredentials {
            additional_data: peer_auth.additional_data,
            public_key: peer_key,
        };
        if !self.authenticator.is_peer_valid(&peer_credentials) {
            return Err(HandshakeError::ServerAuthenticationError);
        }
        return Ok(());
    }

    pub fn data_transfer(mut self) -> Result<Self, HandshakeError> {
        match self.session.into_transport_mode() {
            Err(y) => {
                return Err(HandshakeError::DataTransferFail)
            }
            Ok(x) => {
                self.session = x;
                return Ok(self);
            },
        }
    }
    
    pub fn encrypt_message(&mut self, message: Vec<u8>) -> Result<Vec<u8>, SendMessageError> {
        let ct_len = MAC_SIZE + message.len();
        if ct_len > NOISE_MESSAGE_MAX_SIZE {
            return Err(SendMessageError::InvalidMessageSize);
        }
        let mut ct_hdr = [0u8; 4];
        BigEndian::write_u32(&mut ct_hdr, ct_len as u32);
        let mut ciphertext_header = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _result = self.session.write_message(&ct_hdr, &mut ciphertext_header);
        let _header_len;
        match _result {
            Ok(x) => {
                _header_len = x;
            },
            Err(_) => {
                return Err(SendMessageError::EncryptFail)
            },
        }
        let mut ciphertext = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _result = self.session.write_message(&message, &mut ciphertext);
        let mut _payload_len;
        match _result {
            Ok(x) => {
                _payload_len = x;
            },
            Err(_) => {
                return Err(SendMessageError::EncryptFail)
            },            
        }
        let mut output = Vec::new();
        output.extend_from_slice(&ciphertext_header[.._header_len]);
        output.extend_from_slice(&ciphertext[.._payload_len]);
        return Ok(output);
    }

    pub fn decrypt_message_header(&mut self, message: Vec<u8>) -> Result<u32, ReceiveMessageError> {
        let mut ciphertext_header = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _result = self.session.read_message(&message[..NOISE_MESSAGE_HEADER_SIZE], &mut ciphertext_header);
        match _result {
            Ok(x) => {
                assert_eq!(x, 4);
                return Ok(BigEndian::read_u32(&ciphertext_header[..NOISE_MESSAGE_HEADER_SIZE]));
            },
            Err(y) => {
                return Err(ReceiveMessageError::DecryptFail);
            },
        }
    }

    pub fn decrypt_message(&mut self, message: Vec<u8>) -> Result<Vec<u8>, ReceiveMessageError> {
        let mut ciphertext = [0u8; NOISE_MESSAGE_MAX_SIZE];
        let _result = self.session.read_message(&message, &mut ciphertext);
        match _result {
            Ok(len) => {
                let mut out = vec![];
                out.extend_from_slice(&ciphertext[..len]);                
                return Ok(out);
            },
            Err(y) => return Err(ReceiveMessageError::DecryptFail),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate rustc_serialize;

    use self::rustc_serialize::hex::ToHex;
    use super::*;
    use self::rand::{Rng};
    use self::rand::os::OsRng;

    struct NaiveAuthenticator {}
    impl PeerAuthenticator for NaiveAuthenticator {
        fn is_peer_valid(&self, peer_credentials: &PeerCredentials) -> bool {
            return true;
        }
    }

    #[test]
    fn session_handshake_test() {
        // server
        let mut r = OsRng::new().expect("failure to create an OS RNG");
        let server_keypair = PrivateKey::generate(&mut r).unwrap();
        let authenticator = NaiveAuthenticator{};
        let server_config = SessionConfig {
            authenticator: Box::new(authenticator),
            authentication_key: server_keypair,
            peer_public_key: None,
            additional_data: vec![],
        };
        let mut server_session = Session::new(server_config, false).unwrap();

        // client
        let authenticator = NaiveAuthenticator{};
        let client_keypair = PrivateKey::generate(&mut r).unwrap();
        let client_config = SessionConfig {
            authenticator: Box::new(authenticator),
            authentication_key: client_keypair,
            peer_public_key: Some(server_keypair.public_key()),
            additional_data: vec![],
        };
        let mut client_session = Session::new(client_config, true).unwrap();

        // handshake phase
        let client_mesg1 = client_session.client_handshake1().unwrap();
        server_session.server_read_handshake1(client_mesg1).unwrap();
        let server_msg1 = server_session.server_handshake1().unwrap();
        client_session.client_read_handshake1(server_msg1).unwrap();
        let client_mesg2 = client_session.client_handshake2().unwrap();
        server_session.server_read_handshake2(client_mesg2).unwrap();

        // data transfer phase
        server_session = server_session.data_transfer().unwrap();
        client_session = client_session.data_transfer().unwrap();

        let payload1 = String::from("\"And 'Will to equality' -that itself shall henceforth be the name of virtue; and against everything that has power we will raise our outcry!\"");
        let text_len = payload1.len();
        let message = payload1.into_bytes();
        let ciphertext = server_session.encrypt_message(message.clone()).unwrap();
        let message_len = client_session.decrypt_message_header(ciphertext.clone()).unwrap();
        let plaintext = client_session.decrypt_message(ciphertext[NOISE_MESSAGE_HEADER_SIZE..].to_vec()).unwrap();
        assert_eq!(message, plaintext);

        let payload2 = String::from("You preachers of equality, the tyrant-madness of impotence cries this in you for \"equality\": thus your most secret tyrant appetite disguies itself in words of virtue!");
        let text_len = payload2.len();
        let message = payload2.into_bytes();
        let ciphertext = server_session.encrypt_message(message.clone()).unwrap();
        let message_len = client_session.decrypt_message_header(ciphertext.clone()).unwrap();
        let plaintext = client_session.decrypt_message(ciphertext[NOISE_MESSAGE_HEADER_SIZE..].to_vec()).unwrap();
        assert_eq!(message, plaintext);
    }
}