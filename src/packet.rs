use std::io::Read;
use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Packet {
    pub seq_num: u32,
    pub sub_channel: u8,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn read(data: &mut dyn Read) -> Result<Option<Packet>> {
        // read length
        let len = match read_var_uint(data)? {
            Some(len) => {
                if len < 5 {
                    bail!("invalid packet length: {len}");
                }
                len - 5
            }
            None => return Ok(None),
        };
        // read seq_num + sub_channel
        let mut buffer = [0; 5];
        data.read_exact(&mut buffer)?;
        // read payload
        let mut payload = Vec::with_capacity(len as usize);
        unsafe {
            payload.set_len(len as usize);
        }
        data.read_exact(&mut payload)?;

        Ok(Some(Packet {
            seq_num: (buffer[0] as u32) << 24 | (buffer[1] as u32) << 16 | (buffer[2] as u32) << 8 | (buffer[3] as u32),
            sub_channel: buffer[4],
            payload,
        }))
    }
}


fn read_var_uint(data: &mut dyn Read) -> Result<Option<u64>> {
    let mut value: u64 = 0;

    let mut buffer = [0; 1];
    if let Err(_) = data.read_exact(&mut buffer) {
        // failed reading the first byte means we reached end of file
        return Ok(None);
    }

    loop {
        let byte = buffer[0];
        value <<= 7;
        value |= (byte & 0x7f) as u64;
        if byte & 0x80 == 0x80 {
            return Ok(Some(value));
        }
        data.read_exact(&mut buffer)?;
    }
}
