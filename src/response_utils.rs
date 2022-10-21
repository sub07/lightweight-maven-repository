use std::io::Read;

use rouille::{Response, ResponseBody};

#[inline]
pub fn text<S: Into<String>>(text: S) -> Response {
    Response::text(text)
}

#[inline]
pub fn bytes<B: Into<Vec<u8>>>(bytes: B) -> Response {
    Response::from_data("application/octet-stream", bytes)
}

#[inline]
pub fn with_code(code: u16) -> Response {
    Response {
        status_code: code,
        headers: Vec::new(),
        data: ResponseBody::empty(),
        upgrade: None,
    }
}

pub fn with_content_size(size: usize) -> Response {
    struct DumbReader;
    impl Read for DumbReader {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> { Ok(0) }
    }

    let data = ResponseBody::from_reader_and_size(DumbReader, size);
    Response {
        status_code: 200,
        headers: Vec::new(),
        data,
        upgrade: None,
    }
}

#[inline]
pub fn bad_request() -> Response {
    with_code(400)
}

pub fn not_found() -> Response {
    with_code(404)
}

pub fn internal_error() -> Response {
    with_code(500)
}

#[inline]
pub fn ok() -> Response {
    with_code(200)
}
