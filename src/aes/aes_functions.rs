use crate::aes::aes_constants::{CMDS, GF_MUL_TABLE, INV_CMDS, INV_SBOX, RCON, SBOX};

pub struct AES {
    pub round_key: Vec<u8>,
    #[allow(dead_code)]
    n_w: u32, // Number of words in state.
    #[allow(dead_code)]
    n_r: u32, // Number of rounds.
}

// FIPS 197 compliant functions.
impl AES {
    pub fn new(key: &[u8]) -> Self {
        // Convert bytes to bits
        let key_length = key.len() * 8;
        // set values of n_w and n_r based on key bit length.
        let (n_w, n_r) = match key_length {
            128 => (4, 10),
            192 => (6, 12),
            256 => (8, 14),
            _ => panic!("Unsupported key length"),
        };

        // Initilize round_key vector based on size needed.
        let mut round_key = key.to_vec();
        // Generate all key rounds.
        Self::key_expansion(&mut round_key, n_w, n_r);

        AES {
            round_key,
            n_w,
            n_r,
        }
    }

    // Cipher function to encrypt a state block.
    pub fn encrypt_block(input: &mut [u8], block_index: usize, round_keys: &[u8]) {
        // Number of columns in the state matrix
        const NB: usize = 4;

        // Initialize the state matrix
        let mut state = [[0u8; NB]; NB];
        for j in 0..NB {
            for i in 0..NB {
                state[i][j] = input[block_index + i + 4 * j];
            }
        }

        // Number of rounds
        let nr = round_keys.len() / (4 * NB) - 1;

        Self::add_round_key(&mut state, round_keys);

        for round in 1..=nr - 1 {
            Self::sub_bytes(&mut state);
            Self::shift_rows(&mut state);
            Self::mix_columns(&mut state);
            Self::add_round_key(
                &mut state,
                &round_keys[round * 4 * NB..(round + 1) * 4 * NB],
            );
        }

        Self::sub_bytes(&mut state);
        Self::shift_rows(&mut state);
        Self::add_round_key(&mut state, &round_keys[nr * 4 * NB..(nr + 1) * 4 * NB]);

        // Replace input with encrypted state.
        for j in 0..NB {
            for i in 0..NB {
                input[block_index + i + 4 * j] = state[i][j];
            }
        }
    }

    // InvCipher function to decrypt a state block.
    pub fn decrypt_block(input: &mut [u8], block_index: usize, round_keys: &[u8]) {
        // Number of columns in the state matrix
        const NB: usize = 4;

        // Initialize the state matrix
        let mut state = [[0u8; NB]; NB];
        for j in 0..NB {
            for i in 0..NB {
                state[i][j] = input[block_index + i + 4 * j];
            }
        }

        // Number of rounds
        let nr = round_keys.len() / (4 * NB) - 1;

        Self::add_round_key(&mut state, &round_keys[nr * 4 * NB..(nr + 1) * 4 * NB]);

        for round in (1..=nr - 1).rev() {
            Self::inv_shift_rows(&mut state);
            Self::inv_sub_bytes(&mut state);
            Self::add_round_key(
                &mut state,
                &round_keys[round * 4 * NB..(round + 1) * 4 * NB],
            );
            Self::inv_mix_columns(&mut state);
        }

        Self::inv_shift_rows(&mut state);
        Self::inv_sub_bytes(&mut state);
        Self::add_round_key(&mut state, round_keys);

        // Replace input with decrypted state.
        for j in 0..NB {
            for i in 0..NB {
                input[block_index + i + 4 * j] = state[i][j];
            }
        }
    }

    // The transformation of the state in which a round key is combined
    // with the state. No inverse because XOR is its own inverse.
    fn add_round_key(state: &mut [[u8; 4]; 4], round_key: &[u8]) {
        for i in 0..4 {
            for j in 0..4 {
                //state[i][j] = state[i][j] ^ round_key[i + 4 * j];
                state[i][j] ^= round_key[i + 4 * j];
            }
        }
    }

    // The transformation of the state that applies the S-box independently
    // to each byte of the state.
    fn sub_bytes(state: &mut [[u8; 4]; 4]) {
        for j in 0..4 {
            for i in 0..4 {
                state[i][j] = SBOX[state[i][j] as usize];
            }
        }
    }

    // The inverse of sub_bytes().
    fn inv_sub_bytes(state: &mut [[u8; 4]; 4]) {
        for j in 0..4 {
            for i in 0..4 {
                state[i][j] = INV_SBOX[state[i][j] as usize];
            }
        }
    }

    // The transformation of the state in which the last three rows are
    // cyclically shifted by different offsets.
    fn shift_rows(state: &mut [[u8; 4]; 4]) {
        for (x, row) in state.iter_mut().enumerate().take(4).skip(1) {
            for _ in 0..x {
                let first = row[0];
                row.rotate_left(1); // Rotate the row left by 1
                row[3] = first;
            }
        }
    }

    // The inverse of shift_rows().
    fn inv_shift_rows(state: &mut [[u8; 4]; 4]) {
        for (x, row) in state.iter_mut().enumerate().take(4).skip(1) {
            for _ in 0..x {
                row.rotate_right(1); // Rotate the row right by 1
            }
        }
    }

    // The transformation of the state that takes all of the columns of the
    // state and mixes their data (independently of one another) to produce
    // new columns.
    fn mix_columns(state: &mut [[u8; 4]; 4]) {
        let mut temp_state = [[0u8; 4]; 4];

        for i in 0..4 {
            for k in 0..4 {
                for j in 0..4 {
                    if CMDS[i][k] == 1 {
                        temp_state[i][j] ^= state[k][j];
                    } else {
                        temp_state[i][j] ^= GF_MUL_TABLE[CMDS[i][k] as usize][state[k][j] as usize];
                    }
                }
            }
        }

        for i in 0..4 {
            state[i].copy_from_slice(&temp_state[i]);
        }
    }

    // The inverse of mix_columns().
    fn inv_mix_columns(state: &mut [[u8; 4]; 4]) {
        let mut temp_state = [[0u8; 4]; 4];

        for i in 0..4 {
            for k in 0..4 {
                for j in 0..4 {
                    temp_state[i][j] ^= GF_MUL_TABLE[INV_CMDS[i][k] as usize][state[k][j] as usize];
                }
            }
        }

        for i in 0..4 {
            state[i].copy_from_slice(&temp_state[i]);
        }
    }

    // The routine that generates the round keys from the key.
    fn key_expansion(key: &mut Vec<u8>, n_w: u32, n_r: u32) {
        key.resize_with(16 * (n_r as usize + 1), Default::default);
        let mut temp = [0u8; 4];

        let mut i = 4 * n_w;
        while i < (16 * (n_r as usize + 1)).try_into().unwrap() {
            for j in 0..4 {
                temp[j] = key[i as usize - 4 + j];
            }

            if i / 4 % n_w == 0 {
                Self::rot_word(&mut temp);
                Self::sub_word(&mut temp);
                Self::rcon(&mut temp, (i / (n_w * 4)) as usize - 1);
            } else if n_w > 6 && i / 4 % n_w == 4 {
                Self::sub_word(&mut temp);
            }

            key[i as usize] = key[i as usize - 4 * n_w as usize] ^ temp[0];
            key[i as usize + 1] = key[i as usize + 1 - 4 * n_w as usize] ^ temp[1];
            key[i as usize + 2] = key[i as usize + 2 - 4 * n_w as usize] ^ temp[2];
            key[i as usize + 3] = key[i as usize + 3 - 4 * n_w as usize] ^ temp[3];

            i += 4;
        }

        //return key;
    }

    // The transformation of words in which the four bytes of the word
    // are permuted cyclically
    fn rot_word(temp: &mut [u8; 4]) {
        let temp2 = temp[0];
        for x in 0..3 {
            temp[x] = temp[x + 1];
        }
        temp[3] = temp2;
    }

    // The transformation of words in which the S-box is applied to each
    // of the four bytes of the word.
    fn sub_word(temp: &mut [u8; 4]) {
        for x in 0..4 {
            temp[x] = SBOX[temp[x] as usize];
        }
    }

    // Word array for the round constant.
    fn rcon(temp: &mut [u8; 4], round: usize) {
        temp[0] ^= RCON[round];
    }
}

pub fn apply_pcks7_padding(input: &mut Vec<u8>) {
    let num_of_needed_padding = 16 - (input.len() % 16);
    let padding_value = num_of_needed_padding as u8;

    for _i in 0..num_of_needed_padding {
        input.push(padding_value);
    }
}

pub fn remove_pcks7_padding(input: &mut Vec<u8>) {
    let padding_value = input[input.len() - 1] as usize;

    let amount_to_remove = input.len() - padding_value;
    input.truncate(amount_to_remove)
}

pub fn xor_blocks(a: &mut [u8], b: &[u8]) {
    // Determine the minimum length between the two blocks
    let len = a.len().min(b.len());

    // XOR each byte up to the minimum length
    for i in 0..len {
        a[i] ^= b[i];
    }
}
