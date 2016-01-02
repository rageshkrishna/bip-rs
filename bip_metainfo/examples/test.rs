extern crate bip_metainfo;

use std::fs::{File};
use std::io::{Write};

use bip_metainfo::{MetainfoBuilder};

// Note: Remember to run this in release mode since building this file is VERY cpu bound.

fn main() {
    let builder = MetainfoBuilder::with_tracker("http://9.rarbg.com:2710/announce").unwrap()
        .set_comment("Torrent File Generated By bip_metainfo");
    // Warning, if you are target a very large file, you may not want to use all of
    // your computer's logical cpus as it can cause slowdowns for other applicatins.
    let contents = builder.build_from_file("C:/Users/GG/Desktop/test.iso", 8).unwrap();
    
    let mut output_file = File::create("C:/Users/GG/Desktop/test.torrent").unwrap();
    output_file.write_all(&contents).unwrap();
}