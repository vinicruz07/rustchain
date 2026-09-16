#![no_std]

/// Variavel publica de hash
/// Um hash SHA-256 é composto por exatamente 32 bytes brutos (256 bits).
/// pub type Hash = [u8; 32];
pub type Hash = [u8; 32];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockHeader {
    pub index: u32,
    pub timestamp: u64,
    pub prev_hash: Hash,
    pub data_hash: Hash,
    pub nonce: u64,
}

impl BlockHeader {
    pub fn new(index: u32, timestamp: u64, prev_hash: Hash, data_hash: Hash) -> Self {
        Self {
            index,
            timestamp,
            prev_hash,
            data_hash,
            nonce: 0,
        }
    }
}
