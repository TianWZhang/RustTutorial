use serde::{Serialize, Deserialize};
use crate::error::BlobError;

pub fn read_u64<R: std::io::Read>(r: &mut R) -> Result<u64, BlobError> {
    let mut buf = [0u8; 8];
    r.read_exact(&mut buf)?;
    Ok(bincode::deserialize(&buf)?)
}

pub fn write_u64<W: std::io::Write>(w: &mut W, dat: u64) -> Result<(), BlobError> {
    let ec = bincode::serialize(&dat)?;
    Ok(w.write_all(&ec)?)
}

pub struct Blob {
    k: Vec<u8>,
    v: Vec<u8>
}

impl Blob {
    pub fn from<K: Serialize, V: Serialize>(k: &K, v: &V) -> Result<Self, bincode::Error> {
        Ok(Self {
            k: bincode::serialize(k)?,
            v: bincode::serialize(v)?
        })
    }

    pub fn out<W: std::io::Write>(&self, w: &mut W) -> Result<(), BlobError> {
        write_u64(w, self.k.len() as u64)?;
        write_u64(w, self.v.len() as u64)?;
        w.write_all(&self.k)?;
        w.write_all(&self.v)?;
        Ok(())
    }

    pub fn read<R: std::io::Read>(r: &mut R) -> Result<Self, BlobError> {
        let klen = read_u64(r)? as usize;
        let vlen = read_u64(r)? as usize;
        let mut k = vec![0u8; klen];
        let mut v = vec![0u8; vlen];
        r.read_exact(&mut k)?;
        r.read_exact(&mut v)?;
        Ok(Blob{k, v})
    }

    pub fn get_v<'a, V: Deserialize<'a>>(&'a self) -> Result<V, BlobError> {
        Ok(bincode::deserialize(&self.v)?)
    }

    pub fn len(&self) -> u64 {
        (16 + self.k.len() + self.v.len()) as u64
    }

    pub fn k_hash(&self, seed: u64) -> u64 {
        hmap::hash(seed, &self.k)
    }

    pub fn key_match(&self, rhs: &Self) -> bool {
        self.k == rhs.k
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    pub struct Point<T> {
        x: T,
        y: T
    }

    #[test]
    fn test_read_write_string() {
        let tfile = "test_data/t_read_write_string";
        std::fs::remove_file(tfile).ok();
        let k: i32 = 7;
        let v = "hello world";
        let blob = Blob::from(&k, &v).unwrap();
        {
            let mut fout = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(tfile)
                .unwrap();
            blob.out(&mut fout);
        }
        let mut fin = std::fs::File::open(tfile).unwrap();
        let b2 = Blob::read(&mut fin).unwrap();
        let v2: String = b2.get_v().unwrap();
        assert_eq!(&v2, v);

        let p: Point<i32> = b2.get_v().unwrap();
        assert_eq!(p, Point{x: 11, y: 0});
    }
}