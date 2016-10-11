use std::io::{ stdout, Write };
extern crate rand;

#[allow(unused_must_use)]
fn write_header() {
    let stdout = stdout();
    let handle = stdout.lock();

    // ChunkId
    handle.write(b"RIFF");

    // ChunkSize = 36 + subchunk size 2
    handle.write(&[ 0x68, 0xac, 0x00, 0x00 ]);

    // Format
    // Subchunk1ID
    handle.write(b"WAVEfmt ");

    // Subchunk1size
    handle.write(&[16, 0, 0, 0 ]);

    // AudioFormat
    handle.write(&[ 1, 0 ]);

    // Numchannels
    handle.write(&[ 1, 0 ]);

    // Samplerate
    handle.write(&[ 0x44, 0xac, 0x00, 0x00 ]);

    // Byterate samplerate + num of channels * bits per sample /8
    handle.write(&[ 0x44, 0xac, 0x00, 0x00 ]);

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
