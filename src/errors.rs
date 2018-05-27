// error.rs - noise based wire protocol errors
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

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CommandError {
    GetConsensusDecodeError,
    ConsensusDecodeError,
    PostDescriptorDecodeError,
    PostDescriptorStatusDecodeError,
    VoteDecodeError,
    VoteStatusDecodeError,
    RetreiveMessageDecodeError,
    MessageDecodeError,
    InvalidMessageType,
    InvalidStateError,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CommandError::*;
        match *self {
            GetConsensusDecodeError => write!(f, "Failed to decode a Get Consensus command."),
            ConsensusDecodeError => write!(f, "Failed to decode a Consensus command."),
            PostDescriptorDecodeError => write!(f, "Failed to decode a PostDescriptor command."),
            PostDescriptorStatusDecodeError => write!(f, "Failed to decode a PostDescriptor command."),
            VoteDecodeError => write!(f, "Failed to decode a Vote command."),
            VoteStatusDecodeError => write!(f, "Failed to decode a VoteStatus command."),
            RetreiveMessageDecodeError => write!(f, "Failed to decode a RetreiveMessage command."),
            MessageDecodeError => write!(f, "Failed to decode a Message command."),
            InvalidMessageType => write!(f, "Failed to decode a Message command with invalid type."),
            InvalidStateError => write!(f, "Encountered invalid state transition."),
        }
    }
}


impl Error for CommandError {
    fn description(&self) -> &str {
        "I'm a modem error."
    }

    fn cause(&self) -> Option<&Error> {
        use self::CommandError::*;
        match *self {
            GetConsensusDecodeError => None,
            ConsensusDecodeError => None,
            PostDescriptorDecodeError => None,
            PostDescriptorStatusDecodeError => None,
            VoteDecodeError => None,
            VoteStatusDecodeError => None,
            RetreiveMessageDecodeError => None,
            MessageDecodeError => None,
            InvalidMessageType => None,
            InvalidStateError => None,
        }
    }
}


#[derive(Debug)]
pub enum HandshakeError {
    ServerFailedToDecodeRemoteStatic,
    ClientFailedToDecodeRemoteStatic,
    ClientFailedToGetRemoteStatic,
    ClientHandshakeInvalidAuthError,
    InvalidNoiseSpecError,
    NoPeerKeyError,
    MessageFactoryCreateError,
    InvalidStateError,
    ClientHandshakeNoise1Error,
    ClientHandshakeNoise2Error,
    ClientHandshakeNoise3Error,
    ClientHandshakeSend1Error,
    ClientHandshakeSend2Error,
    ClientHandshakeReceiveError,
    ClientAuthenticationError,
    ServerHandshakeReceive1Error,
    ServerHandshakeReceive2Error,
    ServerHandshakeSendError,
    ServerHandshakeNoise1Error,
    ServerHandshakeNoise2Error,
    ServerHandshakeNoise3Error,
    ServerPrologueMismatchError,
    ServerAuthenticationError,
    DataTransferFail,
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::HandshakeError::*;
        match *self {
            ServerFailedToDecodeRemoteStatic => write!(f, "Server failed to decode the remote peer static key."),
            ClientFailedToDecodeRemoteStatic => write!(f, "Client failed to decode the remote peer static key."),
            ClientFailedToGetRemoteStatic => write!(f, "Client failed to get the remote peer static key."),
            ClientHandshakeInvalidAuthError => write!(f, "Invalid auth message error."),
            InvalidNoiseSpecError => write!(f, "Invalid noise protocol string."),
            NoPeerKeyError => write!(f, "No peer key was supplied, error."),
            MessageFactoryCreateError => write!(f, "Failure creating session."),
            InvalidStateError => write!(f, "Invalid session state error."),
            ClientHandshakeNoise1Error => write!(f, "Error preparing client handshake payload."),
            ClientHandshakeNoise2Error => write!(f, "Error preparing client handshake payload."),
            ClientHandshakeNoise3Error => write!(f, "Error preparing client handshake payload."),
            ClientHandshakeSend1Error => write!(f, "Error sending client handshake payload."),
            ClientHandshakeSend2Error => write!(f, "Error sending client handshake payload."),
            ClientHandshakeReceiveError => write!(f, "Error receiving client handshake payload."),
            ClientAuthenticationError => write!(f, "Error authenticating peer."),
            ServerHandshakeNoise1Error => write!(f, "Error preparing server handshake payload."),
            ServerHandshakeNoise2Error => write!(f, "Error preparing server handshake payload."),
            ServerHandshakeNoise3Error => write!(f, "Error preparing server handshake payload."),
            ServerHandshakeSendError => write!(f, "Error sending server handshake payload."),
            ServerHandshakeReceive1Error => write!(f, "Error receiving server handshake payload."),
            ServerHandshakeReceive2Error => write!(f, "Error receiving server handshake payload."),
            ServerPrologueMismatchError => write!(f, "Error server received wrong prologue from client."),
            ServerAuthenticationError => write!(f, "Error server failed to authenticate client."),
            DataTransferFail => write!(f, "Error failed to switch to data transfer mode."),
        }
    }
}


impl Error for HandshakeError {
    fn description(&self) -> &str {
        "I'm a modem error."
    }

    fn cause(&self) -> Option<&Error> {
        use self::HandshakeError::*;
        match *self {
            ServerFailedToDecodeRemoteStatic => None,
            ClientFailedToDecodeRemoteStatic => None,
            ClientFailedToGetRemoteStatic => None,
            ClientHandshakeInvalidAuthError => None,
            InvalidNoiseSpecError => None,
            NoPeerKeyError => None,
            MessageFactoryCreateError => None,
            InvalidStateError => None,
            ClientHandshakeNoise1Error => None,
            ClientHandshakeNoise2Error => None,
            ClientHandshakeNoise3Error => None,
            ClientHandshakeSend1Error => None,
            ClientHandshakeSend2Error => None,
            ClientHandshakeReceiveError => None,
            ClientAuthenticationError => None,
            ServerHandshakeNoise1Error => None,
            ServerHandshakeNoise2Error => None,
            ServerHandshakeNoise3Error => None,
            ServerHandshakeSendError => None,
            ServerHandshakeReceive1Error => None,
            ServerHandshakeReceive2Error => None,
            ServerPrologueMismatchError => None,
            ServerAuthenticationError => None,
            DataTransferFail => None,
        }
    }
}

#[derive(Debug)]
pub enum SendMessageError {
    InvalidMessageSize,
    EncryptFail,
}

impl fmt::Display for SendMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::SendMessageError::*;
        match *self {
            InvalidMessageSize => write!(f, "Invalid message size."),
            EncryptFail => write!(f, "Failure to encrypt."),
        }
    }
}

impl Error for SendMessageError {
    fn description(&self) -> &str {
        "I'm a modem error."
    }

    fn cause(&self) -> Option<&Error> {
        use self::SendMessageError::*;
        match *self {
            InvalidMessageSize => None,
            EncryptFail => None,
        }
    }
}

#[derive(Debug)]
pub enum ReceiveMessageError {
    InvalidMessageSize,
    DecryptFail,
}

impl fmt::Display for ReceiveMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ReceiveMessageError::*;
        match *self {
            InvalidMessageSize => write!(f, "Invalid message size."),
            DecryptFail => write!(f, "Failure to encrypt."),
        }
    }
}

impl Error for ReceiveMessageError {
    fn description(&self) -> &str {
        "I'm a modem error."
    }

    fn cause(&self) -> Option<&Error> {
        use self::ReceiveMessageError::*;
        match *self {
            InvalidMessageSize => None,
            DecryptFail => None,
        }
    }
}
