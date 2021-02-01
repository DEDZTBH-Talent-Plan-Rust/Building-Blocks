use bson::Document;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::{remove_file, File};
use std::io::{BufRead, BufReader, BufWriter, Read};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Move {
    dir: Direction,
    dist: i32,
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
    fn to_file(&self, filepath: &Path, format: SerializeFormat) -> Result<(), anyhow::Error> {
        let f = File::create(filepath)?;
        let writer = BufWriter::new(f);
        match format {
            SerializeFormat::JSON => serde_json::to_writer(writer, self)?,
            SerializeFormat::RON => ron::ser::to_writer(writer, self)?,
        };
        Ok(())
    }

    fn from_file(filepath: &Path, format: SerializeFormat) -> Result<Move, anyhow::Error> {
        let f = File::open(filepath)?;
        let reader = BufReader::new(f);
        let deserialized: Move = match format {
            SerializeFormat::JSON => serde_json::from_reader(reader)?,
            SerializeFormat::RON => ron::de::from_reader(reader)?,
        };
        Ok(deserialized)
    }

    fn to_vec(&self, format: SerializeFormat) -> Result<Vec<u8>, anyhow::Error> {
        Ok(match format {
            SerializeFormat::JSON => serde_json::to_vec(self)?,
            SerializeFormat::RON => {
                let mut vec: Vec<u8> = vec![];
                ron::ser::to_writer(&mut vec, self)?;
                vec
            }
        })
    }

    fn vec_to_file(vec: &Vec<Move>, filepath: &Path) -> Result<(), anyhow::Error> {
        let f = File::create(filepath)?;
        let mut writer = BufWriter::new(f);
        for mv in vec {
            bson::ser::to_document(mv)?.to_writer(&mut writer)?;
        }
        Ok(())
    }

    fn vec_from_reader<R: Read>(mut reader: BufReader<R>) -> Result<Vec<Move>, anyhow::Error> {
        let mut vec: Vec<Move> = vec![];
        while reader.fill_buf()?.len() > 0 {
            let doc = Document::from_reader(&mut reader)?;
            let mv: Move = bson::from_document(doc)?;
            vec.push(mv)
        }
        Ok(vec)
    }

    fn vec_from_file(filepath: &Path) -> Result<Vec<Move>, anyhow::Error> {
        let f = File::open(filepath)?;
        let reader = BufReader::new(f);
        Move::vec_from_reader(reader)
    }

    fn vec_to_vec(moves: &Vec<Move>) -> Result<Vec<u8>, anyhow::Error> {
        let mut vec: Vec<u8> = vec![];
        for mv in moves {
            bson::to_document(mv)?.to_writer(&mut vec)?;
        }
        Ok(vec)
    }

    fn vec_from_vec(vec: &Vec<u8>) -> Result<Vec<Move>, anyhow::Error> {
        let reader = BufReader::new(vec.as_slice());
        Move::vec_from_reader(reader)
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
    remove_file(filepath).unwrap();

    // serialize and deserialize to ron
    let filepath = Path::new("movea.ron");
    a.to_file(filepath, SerializeFormat::RON).unwrap();
    let b = Move::from_file(filepath, SerializeFormat::RON).unwrap();
    println!("{:?}", b);
    assert_eq!(a, b);
    remove_file(filepath).unwrap();

    // serialize to Vec<u8> with ron
    let vec = a.to_vec(SerializeFormat::RON).unwrap();
    println!("{}", String::from_utf8(vec).unwrap());

    // try ser/de 1000 moves with bson
    let mut moves: Vec<Move> = vec![];
    let mut rng = thread_rng();
    for _ in 1..1001 {
        moves.push(Move {
            dir: match rng.gen_range(0..4) {
                0 => Direction::Left,
                1 => Direction::Right,
                2 => Direction::Up,
                _ => Direction::Down,
            },
            dist: random(),
        })
    }

    let filepath = Path::new("movea.bson");
    Move::vec_to_file(&moves, filepath).unwrap();
    let moves2 = Move::vec_from_file(filepath).unwrap();
    assert_eq!(moves, moves2);
    remove_file(filepath).unwrap();

    let vec = Move::vec_to_vec(&moves).unwrap();
    println!("1000 Move took {} bytes", vec.len());
    let moves2 = Move::vec_from_vec(&vec).unwrap();
    assert_eq!(moves, moves2);
}
