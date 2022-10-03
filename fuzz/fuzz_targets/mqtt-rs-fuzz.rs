#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::Cursor;

use mqtt::{Encodable, Decodable};
use mqtt::packet::{VariablePacket, PublishPacket, QoSWithPacketIdentifier};
use mqtt::TopicName;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    if data.len() > 0 {
        let opt = data[0] % 2;
        let mut dec_buf = Cursor::new(&data[1..]);
        match opt {
            0=>{
                let decoded = PublishPacket::decode(&mut dec_buf);
            },
            1=>{
                let auto_decode = VariablePacket::decode(&mut dec_buf);
            },
            _=>()
        }
    }
});
