use tokio_io::codec::{Decoder, Encoder};

pub struct FtpCodec;

use bytes::BytesMut;
use cmd::Command;
use error::Error;

impl Decoder for FtpCodec {
    type Item = Command;
    type Error = io::Error;
    fn decode(&mut self, buf:  &mut BytesMut) -> io::Result<Option<Command>> {
        io::Result<Option<Command>> {
            if let Some(index) = find_crlf(buf) {
                let line = buf.split_to(index);
                buf.split_to(2);
                Command::new(line.to_vec()).map(|command| Some(command)).map_err(Error::to_io_error)
            } else {
                Ok(None)
            }
        }
    }
}

