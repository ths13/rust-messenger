extern crate firebase;
extern crate hyper;
extern crate hyper_openssl;
extern crate rustc_serialize;
extern crate url;

use self::firebase::{ReqErr, ParseError,};
use std::str;
use std::io;

pub fn handleParseError(err: ParseError) -> ServerError {
    match err {
        ParseError::UrlHasNoPath    => ServerError::UrlHasNoPath,
        ParseError::UrlIsNotHTTPS   => ServerError::UrlIsNotHTTPS,
        ParseError::Parser(err)     => ServerError::Parser(err),
    }
}

pub fn handleReqErr(err: ReqErr) -> ServerError {
    match err {
        ReqErr::ReqNotJSON                  => ServerError::ReqNotJSON,
        ReqErr::RespNotUTF8(err)            => ServerError::RespNotUTF8(err),
        ReqErr::NetworkErr(err)             => ServerError::NetworkErr(err),
        ReqErr::SslErr(err)                 => ServerError::SslErr(err),
        ReqErr::FirebaseIoErr(err)          => ServerError::FirebaseIoErr(err),
        ReqErr::FirebaseIoJsonParseErr(err) => ServerError::FirebaseIoJsonParseErr(err),
        ReqErr::OtherErr(err)               => ServerError::OtherErr(err),
    }
}

pub enum ServerError {
    ReqNotJSON,
    RespNotUTF8(str::Utf8Error),
    NetworkErr(hyper::error::Error),
    SslErr(hyper_openssl::openssl::error::ErrorStack),
    FirebaseIoErr(String),
    FirebaseIoJsonParseErr(rustc_serialize::json::DecoderError),
    OtherErr(io::Error),

    UrlHasNoPath,
    UrlIsNotHTTPS,
    Parser(url::ParseError),

    InvalidUserId,
    InvalidThreadId,
}