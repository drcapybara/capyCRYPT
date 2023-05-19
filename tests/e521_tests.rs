#[cfg(test)]
mod e521_tests {
    use capycrypt::{
        curve::{Generator, IdPoint, E521},
        sha3::aux_functions::byte_utils::get_random_big,
    };

    use rand::{thread_rng, Rng};
    use rug::Integer as big;

    #[test]
    // 0 * G = 𝒪
    fn test_zero_times_g() {
        let mut point = E521::generator(false);
        let s = big::from(0);
        point = point * (s);
        let id_point = E521::id_point();
        assert!(
            &id_point == &point,
            "points are not equal, check addition function"
        )
    }

    // G * 1 = G
    #[test]
    fn test_g_times_one() {
        let mut point = E521::generator(false);
        let s = big::from(1);
        let g = E521::generator(false);
        point = point * (s);
        // println!("point x: {}", point.x);
        // println!("point y: {}", point.y);
        assert!(
            &g == &point,
            "points are not equal, check mul and add functions"
        )
    }

    // G + (-G) = 𝒪
    #[test]
    fn test_g_plus_neg_g() {
        let g = E521::generator(false);
        assert!(
            g.clone() + -g == E521::id_point(),
            "points are not equal, check mul and add functions"
        )
    }

    #[test]
    // 2 * G = G + G
    fn test_two_times_g() {
        let s = big::from(2);
        let two_g = E521::generator(false) * (s);
        let mut sum_g = E521::generator(false);
        sum_g = sum_g.clone() + sum_g.clone();
        assert!(
            &sum_g == &two_g,
            "points are not equal, check mul and add functions"
        )
    }

    #[test]
    // 4 * G = 2 * (2 * G)
    fn test_four_g() {
        let mut four_g = E521::generator(false);
        four_g = four_g * (big::from(4));
        let two = big::from(2);
        let two_times_two_g = E521::generator(false) * (two.clone()) * (two.clone());
        assert!(&four_g == &two_times_two_g)
    }

    #[test]
    //4 * G != 𝒪
    fn test_four_g_not_id() {
        let four_g = E521::generator(false) * (big::from(4));
        let id = E521::id_point();
        assert!(!(&four_g == &id))
    }

    #[test]
    //r*G = 𝒪
    fn r_times_g_id() {
        let g = E521::generator(false) * (E521::id_point().r);
        assert!(&g == &E521::id_point())
    }

    #[test]
    // k*G = (k mod r)*G
    fn k_g_equals_k_mod_r_times_g() {
        let mut rng = thread_rng();
        let k_u128: u64 = rng.gen();
        let k = big::from(k_u128);
        let same_k = k.clone();
        let g = E521::generator(false) * (k);
        let r = E521::generator(false).r;
        let k_mod_r = same_k % r;
        let mut k_mod_r_timesg = E521::generator(false);
        k_mod_r_timesg = k_mod_r_timesg * (k_mod_r);
        assert!(&g == &k_mod_r_timesg)
    }

    #[test]
    //(k + 1)*G = (k*G) + G
    fn k_plus_one_g() {
        let k = get_random_big(256);
        let k_2 = k.clone();
        let k1g = E521::generator(false) * (k + 1);

        let mut kgg = E521::generator(false) * (k_2);
        kgg = kgg + E521::generator(false);
        assert!(&k1g == &kgg)
    }

    #[test]
    //(k + t)*G = (k*G) + (t*G)
    fn k_t() {
        let mut rng = thread_rng();
        let rnd: u64 = rng.gen();

        let k = big::from(rnd);
        let k_2 = k.clone();

        let t = big::from(rnd);
        let t_2 = t.clone();

        // (k + t)*G
        let r0 = E521::generator(false) * (k + t);
        // (k*G)
        let mut r1 = E521::generator(false) * (k_2);
        // (t*G)
        let r2 = E521::generator(false) * (t_2);
        r1 = r1 + r2;
        assert!(&r1 == &r0)
    }

    #[test]
    //k*(t*P) = t*(k*G) = (k*t mod r)*G
    fn test_ktp() {
        let r = E521::generator(false).r;
        let k = get_random_big(256);
        let k_2 = k.clone();
        let k_3 = k.clone();

        let t = get_random_big(256);
        let t_2 = t.clone();
        let t_3 = t.clone();

        let ktp = E521::generator(false) * (t) * (k);
        let tkg = E521::generator(false) * (k_2) * (t_2);
        let k_t_mod_r_g = E521::generator(false) * ((k_3 * t_3) % r);

        assert!(&ktp == &tkg && &k_t_mod_r_g == &tkg && &k_t_mod_r_g == &ktp)
    }
}
