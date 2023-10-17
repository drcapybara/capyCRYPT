#[cfg(test)]
pub mod ops_tests {
    use capycrypt::sha3::aux_functions::byte_utils::get_random_bytes;
    use capycrypt::{KeyEncryptable, KeyPair, Message, PwEncryptable, Signable};
    use capycrypt::curves::EdCurves::E448;
    use std::time::Instant;

    #[test]
    pub fn test_sym_enc_512() {
        let pw = get_random_bytes(64);
        let mut msg = Message::new(&mut get_random_bytes(5242880));

        msg.pw_encrypt(&mut pw.clone(), 512);
        msg.pw_decrypt(&mut pw.clone(), 512);

        let res = msg.op_result.unwrap();
        assert!(res);
    }
    #[test]
    pub fn test_sym_enc_256() {
        let pw = get_random_bytes(64);
        let mut msg = Message::new(&mut get_random_bytes(5242880));

        msg.pw_encrypt(&mut pw.clone(), 256);
        msg.pw_decrypt(&mut pw.clone(), 256);

        let res = msg.op_result.unwrap();
        assert!(res);
    }
    #[test]
    fn test_key_gen_enc_dec_256() {
        //check conversion to and from bytes.
        let mut msg = Message::new(&mut get_random_bytes(5242880));
        let key_pair = KeyPair::new(&get_random_bytes(32), "test key".to_string(), E448, 256);

        msg.key_encrypt(&key_pair.pub_key, 256);
        msg.key_decrypt(&key_pair.priv_key, 256);

        let res = msg.op_result.unwrap();
        assert!(res);
    }

    #[test]
    fn test_key_gen_enc_dec_512() {
        //check conversion to and from bytes.
        let mut msg = Message::new(&mut get_random_bytes(5242880));
        let key_pair = KeyPair::new(&get_random_bytes(32), "test key".to_string(), E448, 512);

        msg.key_encrypt(&key_pair.pub_key, 512);
        msg.key_decrypt(&key_pair.priv_key, 512);

        let res = msg.op_result.unwrap();
        assert!(res);
    }
    #[test]
    pub fn test_signature_512() {
        let mut msg = Message::new(&mut get_random_bytes(5242880));
        let mut pw = get_random_bytes(64);
        let key_pair = KeyPair::new(&pw, "test key".to_string(), E448, 512);

        msg.sign(&mut pw, 512);
        msg.verify(key_pair.pub_key, 512);

        assert!(msg.op_result.unwrap());
    }
    #[test]
    fn test_sig_timing_side_channel() {
        for i in 0..32 {
            let mut msg = Message::new(&mut get_random_bytes(16));
            let mut pw = get_random_bytes(2 ^ i);
            let key_pair = KeyPair::new(&pw, "test key".to_string(), E448, 512);

            let now = Instant::now();
            msg.sign(&mut pw, 512);
            println!("{} needed {} microseconds", i, now.elapsed().as_micros());
            msg.verify(key_pair.pub_key, 512);
            assert!(msg.op_result.unwrap());
        }
    }
}
