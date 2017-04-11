//! Error struct and methods

use std::result;
use std::error;
use std::io;
use std::fmt;

#[cfg(feature = "security")]
use openssl::ssl::{self, Error as SslError};

/// A type for results generated by this crate's functions where the `Err` type
/// is hard-wired to `enums::Error`.
///
/// This typedef is generally used to avoid writing out `enums::Error` directly and
/// is otherwise a direct mapping to `std::result::Result`.
pub type Result<T> = result::Result<T, Error>;

/// The various errors this library can produce.
#[derive(Debug)]
pub enum Error {
    /// Input/Output error while communicating with Kafka
    Io(io::Error),

    /// An error as reported by a remote Kafka server
    Kafka(KafkaCode),

    /// An error when transmitting a request for a particular topic and partition.
    /// Contains the topic and partition of the request that failed, and the
    /// error code as reported by the Kafka server, respectively.
    TopicPartitionError(String, i32, KafkaCode),

    /// An error as reported by OpenSsl
    #[cfg(feature = "security")]
    Ssl(SslError),

    /// Failure to correctly parse the server response due to the
    /// server speaking a newer protocol version (than the one this
    /// library supports)
    UnsupportedProtocol,

    /// Failure to correctly parse the server response by this library
    /// due to an unsupported compression format of the data
    UnsupportedCompression,

    /// Failure to encode/decode a snappy compressed response from Kafka
    #[cfg(feature = "snappy")]
    InvalidSnappy(::snap::Error),

    /// Failure to decode a response due to an insufficient number of bytes available
    UnexpectedEOF,

    /// Failure to decode or encode a response or request respectively
    CodecError,

    /// Failure to decode a string into a valid utf8 byte sequence
    StringDecodeError,

    /// Unable to reach any host
    NoHostReachable,

    /// Unable to set up `Consumer` due to missing topic assignments
    NoTopicsAssigned,

    /// An invalid user-provided duration
    InvalidDuration,
}

/// Various errors reported by a remote Kafka server.
/// See also [Kafka Errors](http://kafka.apache.org/protocol.html)
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum KafkaCode {
    /// An unexpected server error
    Unknown = -1,
    /// The requested offset is outside the range of offsets
    /// maintained by the server for the given topic/partition
    OffsetOutOfRange = 1,
    /// This indicates that a message contents does not match its CRC
    CorruptMessage = 2,
    /// This request is for a topic or partition that does not exist
    /// on this broker.
    UnknownTopicOrPartition = 3,
    /// The message has a negative size
    InvalidMessageSize = 4,
    /// This error is thrown if we are in the middle of a leadership
    /// election and there is currently no leader for this partition
    /// and hence it is unavailable for writes.
    LeaderNotAvailable = 5,
    /// This error is thrown if the client attempts to send messages
    /// to a replica that is not the leader for some partition. It
    /// indicates that the clients metadata is out of date.
    NotLeaderForPartition = 6,
    /// This error is thrown if the request exceeds the user-specified
    /// time limit in the request.
    RequestTimedOut = 7,
    /// This is not a client facing error and is used mostly by tools
    /// when a broker is not alive.
    BrokerNotAvailable = 8,
    /// If replica is expected on a broker, but is not (this can be
    /// safely ignored).
    ReplicaNotAvailable = 9,
    /// The server has a configurable maximum message size to avoid
    /// unbounded memory allocation. This error is thrown if the
    /// client attempt to produce a message larger than this maximum.
    MessageSizeTooLarge = 10,
    /// Internal error code for broker-to-broker communication.
    StaleControllerEpoch = 11,
    /// If you specify a string larger than configured maximum for
    /// offset metadata
    OffsetMetadataTooLarge = 12,
    /// The server disconnected before a response was received.
    NetworkException = 13,
    /// The broker returns this error code for an offset fetch request
    /// if it is still loading offsets (after a leader change for that
    /// offsets topic partition), or in response to group membership
    /// requests (such as heartbeats) when group metadata is being
    /// loaded by the coordinator.
    GroupLoadInProgress = 14,
    /// The broker returns this error code for group coordinator
    /// requests, offset commits, and most group management requests
    /// if the offsets topic has not yet been created, or if the group
    /// coordinator is not active.
    GroupCoordinatorNotAvailable = 15,
    /// The broker returns this error code if it receives an offset
    /// fetch or commit request for a group that it is not a
    /// coordinator for.
    NotCoordinatorForGroup = 16,
    /// For a request which attempts to access an invalid topic
    /// (e.g. one which has an illegal name), or if an attempt is made
    /// to write to an internal topic (such as the consumer offsets
    /// topic).
    InvalidTopic = 17,
    /// If a message batch in a produce request exceeds the maximum
    /// configured segment size.
    RecordListTooLarge = 18,
    /// Returned from a produce request when the number of in-sync
    /// replicas is lower than the configured minimum and requiredAcks is
    /// -1.
    NotEnoughReplicas = 19,
    /// Returned from a produce request when the message was written
    /// to the log, but with fewer in-sync replicas than required.
    NotEnoughReplicasAfterAppend = 20,
    /// Returned from a produce request if the requested requiredAcks is
    /// invalid (anything other than -1, 1, or 0).
    InvalidRequiredAcks = 21,
    /// Returned from group membership requests (such as heartbeats) when
    /// the generation id provided in the request is not the current
    /// generation.
    IllegalGeneration = 22,
    /// Returned in join group when the member provides a protocol type or
    /// set of protocols which is not compatible with the current group.
    InconsistentGroupProtocol = 23,
    /// Returned in join group when the groupId is empty or null.
    InvalidGroupId = 24,
    /// Returned from group requests (offset commits/fetches, heartbeats,
    /// etc) when the memberId is not in the current generation.
    UnknownMemberId = 25,
    /// Return in join group when the requested session timeout is outside
    /// of the allowed range on the broker
    InvalidSessionTimeout = 26,
    /// Returned in heartbeat requests when the coordinator has begun
    /// rebalancing the group. This indicates to the client that it
    /// should rejoin the group.
    RebalanceInProgress = 27,
    /// This error indicates that an offset commit was rejected because of
    /// oversize metadata.
    InvalidCommitOffsetSize = 28,
    /// Returned by the broker when the client is not authorized to access
    /// the requested topic.
    TopicAuthorizationFailed = 29,
    /// Returned by the broker when the client is not authorized to access
    /// a particular groupId.
    GroupAuthorizationFailed = 30,
    /// Returned by the broker when the client is not authorized to use an
    /// inter-broker or administrative API.
    ClusterAuthorizationFailed = 31,
    /// The timestamp of the message is out of acceptable range.
    InvalidTimestamp = 32,
    /// The broker does not support the requested SASL mechanism.
    UnsupportedSaslMechanism = 33,
    /// Request is not valid given the current SASL state.
    IllegalSaslState = 34,
    /// The version of API is not supported.
    UnsupportedVersion = 35,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        if err.kind() == io::ErrorKind::UnexpectedEof {
            Error::UnexpectedEOF
        } else {
            Error::Io(err)
        }
    }
}

#[cfg(feature = "security")]
impl From<SslError> for Error {
    fn from(err: SslError) -> Error {
        Error::Ssl(err)
    }
}

#[cfg(feature = "security")]
impl<S> From<ssl::HandshakeError<S>> for Error {
    fn from(err: ssl::HandshakeError<S>) -> Error {
        match err {
            ssl::HandshakeError::Failure(e) => From::from(e),
            ssl::HandshakeError::Interrupted(s) => from_sslerror_ref(s.error()),
        }
    }
}

#[cfg(feature = "snappy")]
impl From<::snap::Error> for Error {
    fn from(err: ::snap::Error) -> Error {
        Error::InvalidSnappy(err)
    }
}

impl Clone for Error {
    fn clone(&self) -> Error {
        match self {
            &Error::Io(ref err) => Error::Io(clone_ioe(err)),
            &Error::Kafka(x) => Error::Kafka(x),
            &Error::TopicPartitionError(ref topic, partition, error_code) => {
                Error::TopicPartitionError(topic.clone(), partition, error_code)
            }
            #[cfg(feature = "security")]
            &Error::Ssl(ref x) => from_sslerror_ref(x),
            &Error::UnsupportedProtocol => Error::UnsupportedProtocol,
            &Error::UnsupportedCompression => Error::UnsupportedCompression,
            #[cfg(feature = "snappy")]
            &Error::InvalidSnappy(ref err) => from_snap_error_ref(err),
            &Error::UnexpectedEOF => Error::UnexpectedEOF,
            &Error::CodecError => Error::CodecError,
            &Error::StringDecodeError => Error::StringDecodeError,
            &Error::NoHostReachable => Error::NoHostReachable,
            &Error::NoTopicsAssigned => Error::NoTopicsAssigned,
            &Error::InvalidDuration => Error::InvalidDuration,
        }
    }
}

#[cfg(feature = "security")]
fn from_sslerror_ref(err: &ssl::Error) -> Error {
    match err {
        &SslError::ZeroReturn => Error::Ssl(SslError::ZeroReturn),
        &SslError::WantRead(ref e) => Error::Ssl(SslError::WantRead(clone_ioe(e))),
        &SslError::WantWrite(ref e) => Error::Ssl(SslError::WantWrite(clone_ioe(e))),
        &SslError::WantX509Lookup => Error::Ssl(SslError::WantX509Lookup),
        &SslError::Stream(ref e) => Error::Ssl(SslError::Stream(clone_ioe(e))),
        &SslError::Ssl(ref es) => Error::Ssl(SslError::Ssl(es.clone())),
    }
}

#[cfg(feature = "snappy")]
fn from_snap_error_ref(err: &::snap::Error) -> Error {
    match err {
        &::snap::Error::TooBig { given, max } => {
            Error::InvalidSnappy(::snap::Error::TooBig {
                                     given: given,
                                     max: max,
                                 })
        }
        &::snap::Error::BufferTooSmall { given, min } => {
            Error::InvalidSnappy(::snap::Error::BufferTooSmall {
                                     given: given,
                                     min: min,
                                 })
        }
        &::snap::Error::Empty => Error::InvalidSnappy(::snap::Error::Empty),
        &::snap::Error::Header => Error::InvalidSnappy(::snap::Error::Header),
        &::snap::Error::HeaderMismatch {
             expected_len,
             got_len,
         } => {
            Error::InvalidSnappy(::snap::Error::HeaderMismatch {
                                     expected_len: expected_len,
                                     got_len: got_len,
                                 })
        }
        &::snap::Error::Literal {
             len,
             src_len,
             dst_len,
         } => {
            Error::InvalidSnappy(::snap::Error::Literal {
                                     len: len,
                                     src_len: src_len,
                                     dst_len: dst_len,
                                 })
        }
        &::snap::Error::CopyRead { len, src_len } => {
            Error::InvalidSnappy(::snap::Error::CopyRead {
                                     len: len,
                                     src_len: src_len,
                                 })
        }
        &::snap::Error::CopyWrite { len, dst_len } => {
            Error::InvalidSnappy(::snap::Error::CopyWrite {
                                     len: len,
                                     dst_len: dst_len,
                                 })
        }
        &::snap::Error::Offset { offset, dst_pos } => {
            Error::InvalidSnappy(::snap::Error::Offset {
                                     offset: offset,
                                     dst_pos: dst_pos,
                                 })
        }
        &::snap::Error::StreamHeader { byte } => {
            Error::InvalidSnappy(::snap::Error::StreamHeader { byte: byte })
        }
        &::snap::Error::StreamHeaderMismatch { ref bytes } => {
            Error::InvalidSnappy(::snap::Error::StreamHeaderMismatch { bytes: bytes.clone() })
        }
        &::snap::Error::UnsupportedChunkType { byte } => {
            Error::InvalidSnappy(::snap::Error::UnsupportedChunkType { byte: byte })
        }
        &::snap::Error::UnsupportedChunkLength { len, header } => {
            Error::InvalidSnappy(::snap::Error::UnsupportedChunkLength {
                                     len: len,
                                     header: header,
                                 })
        }
        &::snap::Error::Checksum { expected, got } => {
            Error::InvalidSnappy(::snap::Error::Checksum {
                                     expected: expected,
                                     got: got,
                                 })
        }
    }
}

/// Attempt to clone `io::Error`.
fn clone_ioe(e: &io::Error) -> io::Error {
    match e.raw_os_error() {
        Some(code) => io::Error::from_raw_os_error(code),
        None => io::Error::new(e.kind(), format!("Io error: {}", e)),
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => error::Error::description(err),
            Error::Kafka(_) => "Kafka Error",
            Error::TopicPartitionError(_, _, _) => "Error in request for topic and partition",
            #[cfg(feature = "security")]
            Error::Ssl(ref err) => error::Error::description(err),
            Error::UnsupportedProtocol => "Unsupported protocol version",
            Error::UnsupportedCompression => "Unsupported compression format",
            #[cfg(feature = "snappy")]
            Error::InvalidSnappy(ref err) => error::Error::description(err),
            Error::UnexpectedEOF => "Unexpected EOF",
            Error::CodecError => "Encoding/Decoding error",
            Error::StringDecodeError => "String decoding error",
            Error::NoHostReachable => "No host reachable",
            Error::NoTopicsAssigned => "No topic assigned",
            Error::InvalidDuration => "Invalid duration",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => err.cause(),
            #[cfg(feature = "security")]
            Error::Ssl(ref err) => err.cause(),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Kafka(ref c) => write!(f, "Kafka Error ({:?})", c),
            Error::TopicPartitionError(ref topic, ref partition, ref error_code) => {
                write!(f, "Topic Partition Error ({:?}, {:?}, {:?})", topic, partition, error_code)
            }
            #[cfg(feature = "security")]
            Error::Ssl(ref err) => err.fmt(f),
            Error::UnsupportedProtocol => write!(f, "Unsupported protocol version"),
            Error::UnsupportedCompression => write!(f, "Unsupported compression format"),
            #[cfg(feature = "snappy")]
            Error::InvalidSnappy(ref err) => write!(f, "Snappy error, {}", err),
            Error::UnexpectedEOF => write!(f, "Unexpected EOF"),
            Error::CodecError => write!(f, "Encoding/Decoding Error"),
            Error::StringDecodeError => write!(f, "String decoding error"),
            Error::NoHostReachable => write!(f, "No host reachable"),
            Error::NoTopicsAssigned => write!(f, "No topic assigned"),
            Error::InvalidDuration => write!(f, "Invalid duration"),
        }
    }
}
