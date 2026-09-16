/// Módulo core responsável pelas estruturas de dados do blockchain
/// Escrito em no_std para rodar bare metal
#![no_std]

/// Dependencias externas
/// * sha2 - Algoritmo de hash SHA-256
use sha2::{Digest, Sha256};

/// Variavel publica do tipo hash
/// Um hash SHA-256 é composto por exatamente 32 bytes brutos (256 bits).
pub type Hash = [u8; 32];

/// Struct de cabecalho de bloco
///
/// # Fields
///
/// * `index` - Numero do bloco na blockchain
/// * `timestamp` - Timestamp quando o bloco foi criado (em segundos desde a epoch)
/// * `prev_hash` - Hash do bloco anterior
/// * `data_hash` - Hash dos dados do bloco
/// * `nonce` - Numero usado para encontrar um hash valido
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockHeader {
    pub index: u32,
    pub timestamp: u64,
    pub prev_hash: Hash,
    pub data_hash: Hash,
    pub nonce: u64,
}

///Funcoes da struct BlockHeader
impl BlockHeader {

    /// Funcao new para instanciar um BlockHeader
    pub fn new(index: u32, timestamp: u64, prev_hash: Hash, data_hash: Hash) -> Self {
        Self {
            index,
            timestamp,
            prev_hash,
            data_hash,
            nonce: 0,
        }
    }

    /// Funcao que serializa o header do bloco e calcula o hash
    /// 1. Inicializa o estado interno do hasher SHA-256
    /// 2. Alimenta o hasher em big-endian para garantir um hash determinístico cross-platform (x86, ARM, ESP32)
    /// 3. Finaliza a computação e converte o resultado no tipo `Hash` ([u8; 32])
    pub fn calc_hash(&self) -> Hash {
        
        // 1. Inicializa o estado interno do hasher SHA-256
        let mut hasher = Sha256::new();

        // 2. Alimenta o hasher em big-endian para garantir um hash determinístico cross-platform (x86, ARM, ESP32)
        hasher.update(self.index.to_be_bytes());
        hasher.update(self.timestamp.to_be_bytes());
        hasher.update(&self.prev_hash);
        hasher.update(&self.data_hash);
        hasher.update(self.nonce.to_be_bytes());

        // 3. Finaliza a computação e converte o resultado no tipo `Hash` ([u8; 32])
        hasher.finalize().into()
    }

    /// Funcao que verifica se o hash do cabecalho e valido
    /// A dificuldade é definida pelo target
    /// Para ser valido, o hash deve ser menor que o target (ter mais zeros à esquerda)
    pub fn valid(&self, target: &Hash) -> bool {
        let hash = self.calc_hash();

        // Verifica se o hash é menor que o target (tem mais zeros à esquerda)
        &hash < target;
    }
}
