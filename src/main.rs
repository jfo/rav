extern crate rand;
extern crate byteorder;

use std::io::{ stdout, Write };
use byteorder::{LittleEndian, WriteBytesExt};


#[allow(unused_must_use)]
fn write_header() {
    let stdout = stdout();
    let mut handle = stdout.lock();

    // ChunkId
    handle.write(b"RIFF");

    // ChunkSize = 36 + subchunk size 2
    handle.write(&[ 0x68, 0xac, 0x00, 0x00 ]);

    // Format
    // Subchunk1ID
    handle.write(b"WAVEfmt ");

    // Subchunk1size
    handle.write_u32::<LittleEndian>(16);

    // AudioFormat
    handle.write(&[ 1, 0 ]);

    // Numchannels
    handle.write(&[ 1, 0 ]);

    // Samplerate
    handle.write_u32::<LittleEndian>(44100);

    // Byterate samplerate + num of channels * bits per sample /8
    handle.write_u32::<LittleEndian>(44100);

    // blockalign
    handle.write(&[ 1, 0 ]);

    // bitspersample
    handle.write(&[ 8, 0 ]);

    // subchunk2 id
    handle.write(b"data");

    // subchunk2size == numsamples * numchannels * bitspersample / 8
    handle.write(&[ 0x44, 0xac, 0x00, 0x00 ]);
}

fn main() {
    write_header();
    // for x in 0..44100 {
    //     stdout().write(&[ rand::random::<u8>() ]);
    // }
}
