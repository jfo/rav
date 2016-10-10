use std::io::{ stdout, Write };
extern crate rand;

#[allow(unused_must_use)]
fn main() {

    // ChunkId
    stdout().write(b"RIFF");

    // ChunkSize = 36 + subchunk size 2
    stdout().write(&[ 0x68, 0xac, 0x00, 0x00 ]);

    // Format
    stdout().write(b"WAVE");

    // Subchunk1ID
    stdout().write(b"fmt ");

    // Subchunk1size
    stdout().write(&[16, 0, 0, 0 ]);

    // AudioFormat
    stdout().write(&[ 1, 0 ]);

    // Numchannels
    stdout().write(&[ 1, 0 ]);

    // Samplerate
    stdout().write(&[ 0x44, 0xac, 0x00, 0x00 ]);

    // Byterate samplerate + num of channels * bits per sample /8
    stdout().write(&[ 0x44, 0xac, 0x00, 0x00 ]);

    // blockalign
    stdout().write(&[ 1, 0 ]);

    // bitspersample
    stdout().write(&[ 8, 0 ]);

    // subchunk2 id
    stdout().write(b"data");

    // subchunk2size == numsamples * numchannels * bitspersample / 8
    stdout().write(&[ 0x44, 0xac, 0x00, 0x00 ]);

    for x in 0..44100 {
        stdout().write(&[ rand::random::<u8>() ]);
    }
}
