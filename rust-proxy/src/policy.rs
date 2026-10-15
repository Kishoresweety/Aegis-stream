//! mmap'd policy file with the validation the charter's risk register requires.
use memmap2::Mmap;
use std::{fs::File, io, path::Path};

pub const MAGIC: u32 = 0x4145_4753;
pub const HEADER_LEN: usize = 64;

pub struct Policy {
    map: Mmap,
}

fn u32_at(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

pub fn validate(b: &[u8]) -> Result<(), String> {
    if b.len() < HEADER_LEN {
        return Err(format!("file too small: {}", b.len()));
    }
    if u32_at(b, 0) != MAGIC {
        return Err("bad magic".into());
    }
    if u32_at(b, 4) != 1 {
        return Err(format!("unsupported version {}", u32_at(b, 4)));
    }
    let (k, d) = (u32_at(b, 8) as usize, u32_at(b, 12) as usize);
    let p = u32_at(b, 24) as usize; // reserved[0..4] sits at header offset 24
    if p % 64 != 0 {
        return Err("proj_bytes not 64-aligned".into());
    }
    let expected = k
        .checked_mul(d)
        .and_then(|x| x.checked_mul(4))
        .and_then(|x| x.checked_add(HEADER_LEN + p))
        .ok_or("size overflow")?;
    if b.len() != expected {
        return Err(format!("length {} != expected {}", b.len(), expected));
    }
    Ok(())
}

impl Policy {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        let map = unsafe { Mmap::map(&File::open(path)?)? };
        validate(&map).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self { map })
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.map
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// Minimal valid policy: K=2, D=4, P=64.
    pub fn dummy_policy_bytes() -> Vec<u8> {
        let (k, d, p) = (2u32, 4u32, 64u32);
        let mut v = Vec::new();
        for x in [MAGIC, 1, k, d] {
            v.extend(x.to_le_bytes());
        }
        v.extend(0.85f32.to_le_bytes());
        v.extend(0xFu32.to_le_bytes());
        v.extend(p.to_le_bytes());
        v.extend(std::iter::repeat(0u8).take(36));
        v.extend(std::iter::repeat(0u8).take(p as usize));
        v.extend(std::iter::repeat(0u8).take((k * d * 4) as usize));
        v
    }

    #[test]
    fn dummy_is_valid() {
        assert!(validate(&dummy_policy_bytes()).is_ok());
    }
    #[test]
    fn truncated_rejected() {
        let mut v = dummy_policy_bytes();
        v.pop();
        assert!(validate(&v).is_err());
    }
}
