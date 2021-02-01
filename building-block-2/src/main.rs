use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Move {
    dir: Direction,
    dist: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

enum SerializeFormat {
    JSON,
    RON,
}

impl Move {
    fn to_file(&self, filepath: &Path, format: SerializeFormat) -> io::Result<()> {
        let f = File::create(filepath)?;
        let writer = BufWriter::new(f);
        match format {
            SerializeFormat::JSON => serde_json::to_writer(writer, self).unwrap(),
            SerializeFormat::RON => ron::ser::to_writer(writer, self).unwrap(),
        };

        Ok(())
    }

    fn from_file(filepath: &Path, format: SerializeFormat) -> io::Result<Move> {
        let f = File::open(filepath)?;
        let reader = BufReader::new(f);
        let deserialized: Move = match format {
            SerializeFormat::JSON => serde_json::from_reader(reader).unwrap(),
            SerializeFormat::RON => ron::de::from_reader(reader).unwrap(),
        };
        Ok(deserialized)
    }

    fn to_vec(&self, format: SerializeFormat) -> Vec<u8> {
        match format {
            SerializeFormat::JSON => serde_json::to_vec(self).unwrap(),
            SerializeFormat::RON => {
                let mut vec: Vec<u8> = vec![];
                ron::ser::to_writer(&mut vec, self).unwrap();
                vec
            }
        }
    }
}

fn main() {
    let a = Move {
        dir: Direction::Left,
        dist: 42,
    };
    println!("{:?}", a);

    // serialize and deserialize to json
    let filepath = Path::new("movea.json");
    a.to_file(filepath, SerializeFormat::JSON).unwrap();
    let b = Move::from_file(filepath, SerializeFormat::JSON).unwrap();
    println!("{:?}", b);
    assert_eq!(a, b);

    // serialize and deserialize to ron
    let filepath = Path::new("movea.ron");
    a.to_file(filepath, SerializeFormat::RON).unwrap();
    let b = Move::from_file(filepath, SerializeFormat::RON).unwrap();
    println!("{:?}", b);
    assert_eq!(a, b);

    // serialize to Vec<u8> with ron
    let vec = a.to_vec(SerializeFormat::RON);
    println!("{}", String::from_utf8(vec).unwrap())
}
