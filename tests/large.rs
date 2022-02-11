use rand::prelude::*;
use std::{
    io::{Cursor, Seek, SeekFrom, Write},
    path::PathBuf,
};

use cfb::CompoundFile;

#[test]
fn large_file() {
    let cursor = Cursor::new(Vec::new());
    let mut comp = CompoundFile::create(cursor).expect("create");

    let mut rng = rand::thread_rng();
    comp.create_stream("/index").unwrap();

    for i in 0..1000 {
        let mut path = PathBuf::new();
        for _ in 0..10 {
            let r: u64 = rng.gen();
            path.push(format!("{}", r));
        }
        comp.create_storage_all(&path).unwrap();
        path.push("file");

        println!("{}: {:?}", i, &path);
        let length: usize = rng.gen_range(0..1_000_000);
        let data = vec![0x11; length];
        comp.create_stream(&path).unwrap().write_all(&data).unwrap();

        {
            let mut stream = comp.open_stream(&path).unwrap();
            stream.seek(SeekFrom::End(0)).unwrap();
            stream.write_all(b"additional test data").unwrap();
        }

        let mut index = comp.open_stream("/index").unwrap();
        index.seek(SeekFrom::End(0)).unwrap();
        index.write_all(&path.to_string_lossy().as_bytes()).unwrap(); 
    }

    let cursor = comp.into_inner();
    let _comp = CompoundFile::open(cursor).expect("open");
}
