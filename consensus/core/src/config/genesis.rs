use crate::{block::Block, header::Header, subnets::SUBNETWORK_ID_COINBASE, tx::Transaction};
use entropyx_hashes::{Hash, ZERO_HASH};
use entropyx_muhash::EMPTY_MUHASH;

/// The constants uniquely representing the genesis block
#[derive(Clone, Debug)]
pub struct GenesisBlock {
    pub hash: Hash,
    pub version: u16,
    pub hash_merkle_root: Hash,
    pub utxo_commitment: Hash,
    pub timestamp: u64,
    pub bits: u32,
    pub nonce: u64,
    pub daa_score: u64,
    pub coinbase_payload: &'static [u8],
}

impl GenesisBlock {
    pub fn build_genesis_transactions(&self) -> Vec<Transaction> {
        vec![Transaction::new(0, Vec::new(), Vec::new(), 0, SUBNETWORK_ID_COINBASE, 0, self.coinbase_payload.to_vec())]
    }
}

impl From<&GenesisBlock> for Header {
    fn from(genesis: &GenesisBlock) -> Self {
        Header::new_finalized(
            genesis.version,
            Vec::new(),
            genesis.hash_merkle_root,
            ZERO_HASH,
            genesis.utxo_commitment,
            genesis.timestamp,
            genesis.bits,
            genesis.nonce,
            genesis.daa_score,
            0.into(),
            0,
            ZERO_HASH,
        )
    }
}

impl From<&GenesisBlock> for Block {
    fn from(genesis: &GenesisBlock) -> Self {
        Block::new(genesis.into(), genesis.build_genesis_transactions())
    }
}

impl From<(&Header, &'static [u8])> for GenesisBlock {
    fn from((header, payload): (&Header, &'static [u8])) -> Self {
        Self {
            hash: header.hash,
            version: header.version,
            hash_merkle_root: header.hash_merkle_root,
            utxo_commitment: header.utxo_commitment,
            timestamp: header.timestamp,
            bits: header.bits,
            nonce: header.nonce,
            daa_score: header.daa_score,
            coinbase_payload: payload,
        }
    }
}

/// The genesis block of the block-DAG which serves as the public transaction ledger for the main network.
pub const GENESIS: GenesisBlock = GenesisBlock {
    // hash: Hash::from_bytes([
    //     0x58, 0xc2, 0xd4, 0x19, 0x9e, 0x21, 0xf9, 0x10, 0xd1, 0x57, 0x1d, 0x11, 0x49, 0x69, 0xce, 0xce, 0xf4, 0x8f, 0x9, 0xf9, 0x34,
    //     0xd4, 0x2c, 0xcb, 0x6a, 0x28, 0x1a, 0x15, 0x86, 0x8f, 0x29, 0x99,
    // ]),

    // ENX-CHANGE: generate new hash
    hash: Hash::from_bytes([
        0x01, 0x67, 0x57, 0x27, 0x82, 0xd2, 0x47, 0x1c, 0xa2, 0xd9, 0xf1, 0xcc, 0xb1, 0xb0, 0x92, 0xe4, 0x23, 0x22, 0x21, 0x49, 0x66, 0x63, 
        0x1d, 0xfc, 0xde, 0xf7, 0x7c, 0x36, 0x28, 0x0b, 0x85, 0x26,
    ]),
    version: 0,
    // Old hash_merkle_root
    // hash_merkle_root: Hash::from_bytes([
    //     0x8e, 0xc8, 0x98, 0x56, 0x8c, 0x68, 0x1, 0xd1, 0x3d, 0xf4, 0xee, 0x6e, 0x2a, 0x1b, 0x54, 0xb7, 0xe6, 0x23, 0x6f, 0x67, 0x1f,
    //     0x20, 0x95, 0x4f, 0x5, 0x30, 0x64, 0x10, 0x51, 0x8e, 0xeb, 0x32,
    // ]),
    // ENX-CHANGE: Change hash_merkle_root to the new hash
    hash_merkle_root: Hash::from_bytes([
        0x9b, 0xad, 0x5c, 0x70, 0x01, 0xe2, 0x2a, 0x43, 0x72, 0x44, 0x9e, 0x61, 0xc0, 0x77, 0x40, 0x78, 0xef, 0xf4, 0x16, 0xd6, 0x8b, 0xf3, 0xe9, 
        0x9b, 0x81, 0x42, 0x2b, 0xc7, 0xf4, 0x6b, 0xa9, 0x7b,
    ]),
    utxo_commitment: Hash::from_bytes([
        0x71, 0x0f, 0x27, 0xdf, 0x42, 0x3e, 0x63, 0xaa, 0x6c, 0xdb, 0x72, 0xb8, 0x9e, 0xa5, 0xa0, 0x6c, 0xff, 0xa3, 0x99, 0xd6, 0x6f,
        0x16, 0x77, 0x04, 0x45, 0x5b, 0x5a, 0xf5, 0x9d, 0xef, 0x8e, 0x20,
    ]),
    timestamp: 1740396000000,// ENX-CHANGE: utc
    bits: 486722099,
    nonce: 0x3392c,
    daa_score: 0, // ENX-CHANGE 1312860, // Checkpoint DAA score
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xad, 0xd5, 0xc0, 0x07, 0x00, 0x00, 0x00, // Subsidy (333 ENX in first period)
        0x00, 0x00, //script version
        0x01,                                           // Varint
        0x00,                                           // OP-FALSE
        // Bitcoin block hash
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x02, 0x4a, 0x78, 0xc2, 0x71, 0x69, 0x38,
        0x13, 0x68, 0x3c, 0x46, 0x6b, 0xda, 0x32, 0x25,
        0x50, 0x55, 0x46, 0x16, 0x67, 0xdc, 0xd9, 0x42,
        // ENX-CHANGE: Change genesis message
        // "As entropyx increases, energy disperses, and entropyx becomes more concentrated. A new era is born at this moment."
        0x41, 0x73, 0x20, 0x65, 0x6E, 0x74, 0x72, 0x6F, 
        0x70, 0x79, 0x20, 0x69, 0x6E, 0x63, 0x72, 0x65, 
        0x61, 0x73, 0x65, 0x73, 0x2C, 0x20, 0x65, 0x6E, 
        0x65, 0x72, 0x67, 0x79, 0x20, 0x64, 0x69, 0x73, 
        0x70, 0x65, 0x72, 0x73, 0x65, 0x73, 0x2E, 0x20, 
        0x41, 0x73, 0x20, 0x45, 0x4E, 0x58, 0x20, 0x62, 
        0x75, 0x72, 0x6E, 0x73, 0x2C, 0x20, 0x76, 0x61, 
        0x6C, 0x75, 0x65, 0x20, 0x63, 0x6F, 0x6E, 0x63, 
        0x65, 0x6E, 0x74, 0x72, 0x61, 0x74, 0x65, 0x73, 
        0x2E, 0x20, 0x41, 0x20, 0x6E, 0x65, 0x77, 0x20, 
        0x64, 0x69, 0x67, 0x69, 0x74, 0x61, 0x6C, 0x20, 
        0x65, 0x72, 0x61, 0x20, 0x69, 0x73, 0x20, 0x62, 
        0x6F, 0x72, 0x6E, 0x20, 0x61, 0x74, 0x20, 0x74, 
        0x68, 0x69, 0x73, 0x20, 0x6D, 0x6F, 0x6D, 0x65, 
        0x6E, 0x74, 0x2E,                                
    ],
};

pub const TESTNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0xf8, 0x96, 0xa3, 0x03, 0x48, 0x73, 0xbe, 0x17, 0x39, 0xfc, 0x43, 0x59, 0x23, 0x68, 0x99, 0xfd, 0x3d, 0x65, 0xd2, 0xbc, 0x94,
        0xf9, 0x78, 0x0d, 0xf0, 0xd0, 0xda, 0x3e, 0xb1, 0xcc, 0x43, 0x70,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x17, 0x34, 0x14, 0x08, 0xa5, 0x72, 0x45, 0x56, 0x50, 0x4d, 0xf4, 0xd6, 0xcf, 0x51, 0x5c, 0xbf, 0xbb, 0x22, 0x04, 0x30, 0xdc,
        0x45, 0x1c, 0x74, 0x3c, 0x22, 0xd5, 0xe9, 0x11, 0x72, 0x0c, 0x2a,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x17c5f62fbb6,
    bits: 0x1e7fffff,
    nonce: 0x14582,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                         // Varint
        0x00,                                                                         // OP-FALSE
        0x6b, 0x61, 0x73, 0x70, 0x61, 0x2d, 0x74, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, // entropyx-testnet
    ],
};

pub const TESTNET11_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0xcf, 0xa2, 0xa7, 0xeb, 0xfb, 0x8b, 0x4e, 0xa3, 0x45, 0x30, 0x7b, 0xc2, 0x5e, 0xf9, 0x42, 0x1b, 0x23, 0x91, 0xf0, 0x9c, 0x8b,
        0x2c, 0xf2, 0x15, 0xf0, 0x75, 0x60, 0xaf, 0x0d, 0x4d, 0x71, 0x64,
    ]),
    hash_merkle_root: Hash::from_bytes([
        0x3c, 0x35, 0xdb, 0x98, 0x02, 0x7e, 0x84, 0x6e, 0x02, 0xcc, 0x60, 0xb7, 0xa7, 0xfa, 0xb1, 0x6d, 0x6c, 0xf2, 0xa5, 0x42, 0xd8,
        0xe1, 0x60, 0xad, 0x9c, 0xd9, 0x08, 0x5f, 0x51, 0x0c, 0x47, 0xbb,
    ]),
    bits: 504155340, // see `gen_testnet11_genesis`
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                         // Varint
        0x00,                                                                         // OP-FALSE
        0x6b, 0x61, 0x73, 0x70, 0x61, 0x2d, 0x74, 0x65, 0x73, 0x74, 0x6e, 0x65, 0x74, // entropyx-testnet
        11, 4                                                                         // TN11, Relaunch 4
    ],
    ..TESTNET_GENESIS
};

pub const SIMNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        0x41, 0x1f, 0x8c, 0xd2, 0x6f, 0x3d, 0x41, 0xae, 0xa3, 0x9e, 0x78, 0x57, 0x39, 0x27, 0xda, 0x24, 0xd2, 0x39, 0x95, 0x70, 0x5b,
        0x57, 0x9f, 0x30, 0x95, 0x9b, 0x91, 0x27, 0xe9, 0x6b, 0x79, 0xe3,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x19, 0x46, 0xd6, 0x29, 0xf7, 0xe9, 0x22, 0xa7, 0xbc, 0xed, 0x59, 0x19, 0x05, 0x21, 0xc3, 0x77, 0x1f, 0x73, 0xd3, 0x52, 0xdd,
        0xbb, 0xb6, 0x86, 0x56, 0x4a, 0xd7, 0xfd, 0x56, 0x85, 0x7c, 0x1b,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x17c5f62fbb6,
    bits: 0x207fffff,
    nonce: 0x2,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                   // Varint
        0x00,                                                                   // OP-FALSE
        0x6b, 0x61, 0x73, 0x70, 0x61, 0x2d, 0x73, 0x69, 0x6d, 0x6e, 0x65, 0x74, // entropyx-simnet
    ],
};

pub const DEVNET_GENESIS: GenesisBlock = GenesisBlock {
    hash: Hash::from_bytes([
        // Golang devnet genesis hash
        // 0xb3, 0x13, 0x87, 0x0a, 0x32, 0xc7, 0x04, 0xbd, 0xf1, 0x21, 0x4a, 0x3b, 0x27, 0x0c, 0xc4, 0x75, 0xd9, 0x42, 0xc2, 0x09, 0x2d,
        // 0x37, 0x9b, 0xc8, 0x70, 0x0a, 0xb0, 0x43, 0x31, 0x9e, 0xf8,
        // 0x46,
        // New rust devnet genesis hash updated according to the modified bits field (see below)
        0x4c, 0xb4, 0x8d, 0x0b, 0x20, 0x73, 0xb8, 0x02, 0x36, 0x01, 0x45, 0xa1, 0x5a, 0xd1, 0xab, 0xdc, 0x01, 0xd8, 0x9b, 0x5c, 0x2f,
        0xe4, 0x72, 0x26, 0x30, 0xab, 0x9b, 0x5f, 0xe9, 0xdf, 0xc4, 0xf2,
    ]),
    version: 0,
    hash_merkle_root: Hash::from_bytes([
        0x58, 0xab, 0xf2, 0x03, 0x21, 0xd7, 0x07, 0x16, 0x16, 0x2b, 0x6b, 0xf8, 0xd9, 0xf5, 0x89, 0xca, 0x33, 0xae, 0x6e, 0x32, 0xb3,
        0xb1, 0x9a, 0xbb, 0x7f, 0xa6, 0x5d, 0x11, 0x41, 0xa3, 0xf9, 0x4d,
    ]),
    utxo_commitment: EMPTY_MUHASH,
    timestamp: 0x11e9db49828,
    // bits: 525264379, // Golang devnet genesis bits
    bits: 0x1e21bc1c, // Bits with ~testnet-like difficulty for slow devnet start
    nonce: 0x48e5e,
    daa_score: 0,
    #[rustfmt::skip]
    coinbase_payload: &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Blue score
        0x00, 0xE1, 0xF5, 0x05, 0x00, 0x00, 0x00, 0x00, // Subsidy
        0x00, 0x00, // Script version
        0x01,                                                                   // Varint
        0x00,                                                                   // OP-FALSE
        0x6b, 0x61, 0x73, 0x70, 0x61, 0x2d, 0x64, 0x65, 0x76, 0x6e, 0x65, 0x74, // entropyx-devnet
    ],
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::bps::Testnet11Bps, merkle::calc_hash_merkle_root};

    pub fn calculate_genesis_hash() -> Hash {
        // create a temporary Block object
        let block = Block::from(&GENESIS);
        
        // compute genesis hash
        let hash = block.hash();
        
        // print new genesis hash
        println!("New genesis hash: {:?}", hash);
        
        hash
    }
    
    #[test]
    fn print_new_genesis_hash() {
        let hash = calculate_genesis_hash();
        println!("New genesis hash bytes:");
        print!("[");
        for byte in hash.as_bytes() {
            print!("0x{:02x}, ", byte);
        }
        println!("]");
    }

    #[test]
    fn test_genesis_hashes() {
        [GENESIS, TESTNET_GENESIS, TESTNET11_GENESIS, SIMNET_GENESIS, DEVNET_GENESIS].into_iter().for_each(|genesis| {
            let block: Block = (&genesis).into();
            // println!("genesis.bits: {}, genesis.hash: {:#04x?}", genesis.bits, genesis.hash.as_bytes());
            assert_hashes_eq(calc_hash_merkle_root(block.transactions.iter(), false), block.header.hash_merkle_root);
            assert_hashes_eq(block.hash(), genesis.hash);
        });
    }

    #[test]
    fn gen_testnet11_genesis() {
        let bps = Testnet11Bps::bps();
        let mut genesis = TESTNET_GENESIS;
        let target = entropyx_math::Uint256::from_compact_target_bits(genesis.bits);
        let scaled_target = target * bps / 100;
        let scaled_bits = scaled_target.compact_target_bits();
        genesis.bits = scaled_bits;
        if genesis.bits != TESTNET11_GENESIS.bits {
            panic!("Testnet 11: new bits: {}\nnew hash: {:#04x?}", scaled_bits, Block::from(&genesis).hash().as_bytes());
        }
    }

    #[test]
    fn print_all_new_genesis_hash() {
        let block = Block::from(&GENESIS);
        // computer hash_merkle_root
        let hash_merkle_root = calc_hash_merkle_root(block.transactions.iter(), false);
        // computer block hash
        let new_hash = block.hash();

        println!("New genesis hash: {}", new_hash);
        println!("New genesis hash bytes:\n[{}]", new_hash
            .as_bytes()
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", "));
            
        println!("\nNew hash_merkle_root: {}", hash_merkle_root);
        println!("New hash_merkle_root bytes:\n[{}]", hash_merkle_root
            .as_bytes()
            .iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", "));
    }

    fn assert_hashes_eq(got: Hash, expected: Hash) {
        if got != expected {
            // Special hex print to ease changing the genesis hash according to the print if needed
            panic!("Got hash {:#04x?} while expecting {:#04x?}", got.as_bytes(), expected.as_bytes());
        }
    }

    /// Convert any string to bytes array format
    #[test]
    fn test_string_to_bytes() {
        // Test normal string
        let text = "As entropy increases, energy disperses. As ENX burns, value concentrates. A new digital era is born at this moment.";
        println!("Converting text to bytes: {}", text);
        print_string_as_bytes(text);

        // Test hash string
        let hash_str = "000000000000000000024a78c271693813683c466bda32255055461667dcd942";
        println!("\nConverting hash to bytes: {}", hash_str);
        print_hex_string_as_bytes(hash_str);

        let genesis_hash = "0167572782d2471ca2d9f1ccb1b092e42322214966631dfcdef77c36280b8526";
        println!("\nConverting genesis hash to bytes: {}", genesis_hash);
        print_hex_string_as_bytes(genesis_hash);

        let market_hash = "9bad5c7001e22a4372449e61c0774078eff416d68bf3e99b81422bc7f46ba97b";
        println!("\nConverting market hash to bytes: {}", market_hash);
        print_hex_string_as_bytes(market_hash);
    }

    /// Convert bytes array to string
    #[test]
    fn test_bytes_to_string() {
        // Test bytes array of normal string
        let text_bytes = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x02, 0x4a, 0x78, 0xc2, 0x71, 0x69, 0x38,
            0x13, 0x68, 0x3c, 0x46, 0x6b, 0xda, 0x32, 0x25,
            0x50, 0x55, 0x46, 0x16, 0x67, 0xdc, 0xd9, 0x42,
        ];
        
        // Convert to UTF-8 string
        let text = String::from_utf8_lossy(&text_bytes);
        println!("Text from bytes: {}", text);
        
        // Convert to hex string
        let hex = text_bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        println!("Hex string: 0x{}", hex);
    }

    /// Helper function: Convert normal string to bytes array format and print
    fn print_string_as_bytes(text: &str) {
        println!("Bytes array format:");
        println!("[");
        for chunk in text.as_bytes().chunks(8) {
            let bytes = chunk.iter()
                .map(|b| format!("0x{:02x}", b))
                .collect::<Vec<_>>()
                .join(", ");
            println!("    {},", bytes);
        }
        println!("]");
    }

    /// Helper function: Convert hex string to bytes array format and print
    fn print_hex_string_as_bytes(hex_str: &str) {
        let clean_hex = hex_str.trim_start_matches("0x");
        let bytes: Vec<String> = (0..clean_hex.len())
            .step_by(2)
            .map(|i| {
                let byte_str = &clean_hex[i..i + 2];
                format!("0x{}", byte_str)
            })
            .collect();

        println!("Bytes array format:");
        println!("[");
        for chunk in bytes.chunks(8) {
            println!("    {}", chunk.join(", ") + ",");
        }
        println!("]");
    }

    /// Parse ENX value test function
    #[test]
    fn test_parse_enx_value() {
        let subsidy_bytes = [0x00, 0xad, 0xd5, 0xc0, 0x07, 0x00, 0x00, 0x00];
        
        // Print each byte position and value
        println!("Byte positions (0-7): ");
        for (i, byte) in subsidy_bytes.iter().enumerate() {
            println!("Position {}: 0x{:02x}", i, byte);
        }
        
        // Try little endian
        let value_le = u64::from_le_bytes(subsidy_bytes);
        let enx_value_le = value_le as u64 / 100_000_000;
        println!("\nLittle Endian:");
        println!("Raw value (sats): {}", value_le);
        println!("ENX value: {} ENX", enx_value_le);
        
        // Test expected value of 333 ENX
        let expected_value: u64 = 333 * 100_000_000;
        let expected_bytes = expected_value.to_le_bytes();
        println!("\nExpected bytes for 333 ENX:");
        println!("Raw bytes: [{}]", expected_bytes.iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", "));
    }

    /// Generate ENX value bytes
    #[test]
    fn test_generate_enx_bytes() {
        let enx_value: u64 = 333;
        let sats_value = enx_value * 100_000_000;
        let bytes = sats_value.to_le_bytes();
        
        println!("For {} ENX:", enx_value);
        println!("Sats value: {}", sats_value);
        println!("Bytes (little endian): [{}]", bytes.iter()
            .map(|b| format!("0x{:02x}", b))
            .collect::<Vec<_>>()
            .join(", "));
    }

    /// Hexadecimal to decimal test function
    #[test]
    fn test_hex_to_decimal() {
        let bits = 0x1d02ca33;
        println!("Bits value:");
        println!("Hex: 0x{:x}", bits);
        println!("Decimal: {}", bits);
    }

    #[test]
    fn test_decimal_to_hex() {
        let bits = 486722099;
        println!("Bits value:");
        println!("Decimal: {}", bits);
        println!("Hex: 0x{:x}", bits);
    }
}
