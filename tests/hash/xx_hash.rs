// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::hash::{xx_hash, Hasher};

static RANDOM: [u64; 100] = [
    0xd37ff54d2c206886, 0x0c50f5cc3ab25e31, 0xa9cdd20dd9b9e3fb, 0x2438e4d410c6dbc0,
    0xfa7e40bb7f5426fe, 0x351fa2b012c86ab6, 0xdc8dbb9e01a42437, 0xb7e8a9022db323db,
    0x8f53105e1b79338f, 0x33dfbd0c0252f557, 0xdeb14367d46d6f51, 0xb3c15c4d3525b66a,
    0x00c3932663da62af, 0xe15dc005a028c4e4, 0x45129fa761756b38, 0x88442cb85dceed89,
    0x94f844e885d92a04, 0xaa61b75e381ed124, 0x40a37c715c7c8c30, 0x19db238fb04b1c37,
    0xf5b85b76d836eb87, 0xc6cc0320dbe7de56, 0x9e88c4a7e07ebe50, 0x10403a06d144e20f,
    0xe19079360f553d85, 0x9d03f0affa1f2906, 0x914274a79634f6f4, 0x7680745078cfc559,
    0x2540567785332f47, 0xbf3926040a968247, 0x1e19019f7bfca724, 0xaaff2c86500d4f92,
    0x9b0995fed4583c1e, 0x2a23fbdb46eea7db, 0xa59b2b05394e32ff, 0xf951f9393ae96edd,
    0xafa8227aeeef496e, 0x14896f7cf6bb1096, 0x6b9a8637648892a7, 0xc62dd36bfed001dc,
    0xd777ca16d32f58a6, 0x761f7023b46fcb7d, 0x25b207786452a4a6, 0x5c9dd22438513a1b,
    0xb6399f6cc48d1d06, 0x319d28dab83a0b71, 0x4ef70b4f74e04e0e, 0x1b39a8fa9617cc92,
    0x3046826e6a750d93, 0xa5da676df15df117, 0x0ca54535fee849c3, 0xc050c42e1707b090,
    0x2b3adbac8aef596d, 0x2be417098ddf4bad, 0x75e1b9fc20fc8de3, 0x15e521fb37627c68,
    0xea086a31c1c84e29, 0xc847f081d7852c5b, 0xc4bef12de728349c, 0x7accea724a0a6977,
    0x445adc6ff6f6e989, 0x4cfaf6ab89054df0, 0xb0e423c4c04d1b50, 0xfc659b8336b7d175,
    0xcf7e07d534478bd6, 0xbf8efbb5f96ac72c, 0x7489aa3d4927eaad, 0xeb1e2210873379e9,
    0xbc24c955ba2c4fcc, 0x13e12ed0fda35a5d, 0x9f786036b9bbf559, 0xe4a10fdaad7e4b4f,
    0x72b4e1435710c1e7, 0xdd72c612e772e84d, 0x355950f792b54350, 0x09d27cfbf499ce19,
    0xecc3f39f3b1b8ca7, 0x6efdd9c07dd2ca55, 0x7048073d553bd4de, 0x36bfa2a632ff5865,
    0xcaa839cca3c1f5b9, 0xab9ad6324ce9c670, 0x7915e2ca6436c5d0, 0xc0a5504a5e9c9181,
    0x49ad1cfb94c867f4, 0x137317e82603a15c, 0xaf81a33cf30a66e7, 0x27ab981d41dd7b06,
    0x35ec99bf8142dac4, 0x2f59e80f3d4b0879, 0x7db56348db2e29d8, 0xbfd4bc3338d110dd,
    0x0b716aa6110d0ab8, 0xec644262234d638d, 0xc4edf2e19fd4ac84, 0x354916893e0cc6cb,
    0x7544401867418c0f, 0xa2744a5f989f5c24, 0xbe4166051385558b, 0x3d3d3a1c0989cb4e,
];

static SEED: [u64; 100] = [
    0xa089c5a1d28a462e, 0x8164df233be386ee, 0x37de05207c0594e6, 0x7ab4e4d9d0196252,
    0x912e91b4a1fab802, 0x208c9bcd4b268703, 0xdfbce5fad37f60c4, 0xe2bd7e9d840afa92,
    0xaa3e7651eb9dadf7, 0xa2d9700fc9d191c2, 0xd6aede4d8991dcb3, 0x27a142d6a4159273,
    0x34e3d575c0a4c2a7, 0x2b63be53c6696a45, 0x99a2ebbd7b59031c, 0x1db0f127c3260964,
    0x17b37ab8c37ae19f, 0xae583821a28f0d2e, 0x197849beb9b3b1a2, 0xbe5956e0f78dd5a0,
    0x67a813a8b9488430, 0xc11e6db14399f99a, 0x36c6aca9e4cbf926, 0x3f7d727080dcd317,
    0x54176a60f36da80d, 0x9ce3664f2ea41077, 0x554114eb4a220fee, 0x53aeefe50587a0e9,
    0x3b4463c437df672e, 0xcfb5a58f0acb752a, 0xcd5dba5264e99962, 0x219898b686bbc299,
    0x6c210d8b86751337, 0xcbe6249066fc8ef0, 0x217f3196e5c9bab7, 0x63cca62eaea45041,
    0xfcdcb684455e51d3, 0x54036a0c60416747, 0x10e0479db1d4f01c, 0x369056e61911ea07,
    0x34a9ada934f0ead3, 0x5e2174b2baa1190d, 0x3f7ae1c7004de66e, 0xb23e3916ee111a3f,
    0x458049babfc8f31c, 0x29f0c5e9cffa8eb1, 0xc42b6e3bbdd79e80, 0x3f3d753b92de21a7,
    0x0dd5846081cdc946, 0xf8d382167e33dd68, 0x048c7697149dc70f, 0x246dc241c583242b,
    0x1f8c8529f1e43599, 0xd221ec41fcf8ccdc, 0x90f34b3144011564, 0x1adbe246417a2912,
    0x61250ce5a0fb9548, 0x361e92fda1c5e022, 0xdf2c0fb7bc5d3912, 0xe5775c5072d857c3,
    0x316e98db6e3fd2db, 0xb8896f6ebfab6a78, 0x1e6ceacf34e4e8f5, 0x9eb38f23a8f2971c,
    0xc8dd3264eb6d09af, 0xd79c2cfaf27f4c32, 0xd732deba4c2a24d1, 0xedf7dfd21080eaba,
    0x57b04a0295498c95, 0x0e9f5419ea1713c6, 0xdc417b11ac783b8e, 0x7a666525b4af7d52,
    0xa816f9b155f2b82b, 0x1fa20723e27039f9, 0x7ddc74b0dc05cd74, 0xc5b07d652f739a3d,
    0xc4fd25a8eac91f99, 0x400891c91ee26b43, 0x8efea88bf41bbdd5, 0x4bddc75bbdd231f7,
    0x3abd72bac26afa13, 0x8f09163443b0d390, 0x7e614b4978d5cf05, 0x8e5c80ca4ac61fbc,
    0x38a6ad31875f699f, 0x389e4353c4c7fe03, 0xb1d93ade8333f6ce, 0x96eb9bccb0a4a834,
    0xcc5c95b5c577c67e, 0x4b5d7bf5d2bf3188, 0x55532b2f1d10c58d, 0xedd0d6774976e885,
    0x3e8f367c8fa29603, 0xc408ec328979444d, 0xe2cffe6199ffec96, 0x844cd9b150340d8b,
    0x1adfede22a90ec65, 0xbc714e7b28252089, 0xdd15bd86b7d65a7c, 0x6676ca73c1da27a3,
];

#[test]
fn u32hash_inline() {
    for i in 0..RANDOM.len() {
        let seed = SEED[i] as u32;
        test!(xx_hash::u32hash_u8(RANDOM[i] as u8, seed)
                == xx_hash::u32hash_bytes((RANDOM[i] as u8).as_ref(), seed));
        test!(xx_hash::u32hash_u16(RANDOM[i] as u16, seed)
                == xx_hash::u32hash_bytes((RANDOM[i] as u16).as_ref(), seed));
        test!(xx_hash::u32hash_u32(RANDOM[i] as u32, seed)
                == xx_hash::u32hash_bytes((RANDOM[i] as u32).as_ref(), seed));
        test!(xx_hash::u32hash_u64(RANDOM[i] as u64, seed)
                == xx_hash::u32hash_bytes((RANDOM[i] as u64).as_ref(), seed));
        test!(xx_hash::u32hash_usize(RANDOM[i] as usize, seed)
                == xx_hash::u32hash_bytes((RANDOM[i] as usize).as_ref(), seed));
    }
}

#[test]
fn u64hash_inline() {
    for i in 0..RANDOM.len() {
        let seed = SEED[i] as u64;
        test!(xx_hash::u64hash_u8(RANDOM[i] as u8, seed)
                == xx_hash::u64hash_bytes((RANDOM[i] as u8).as_ref(), seed));
        test!(xx_hash::u64hash_u16(RANDOM[i] as u16, seed)
                == xx_hash::u64hash_bytes((RANDOM[i] as u16).as_ref(), seed));
        test!(xx_hash::u64hash_u32(RANDOM[i] as u32, seed)
                == xx_hash::u64hash_bytes((RANDOM[i] as u32).as_ref(), seed));
        test!(xx_hash::u64hash_u64(RANDOM[i] as u64, seed)
                == xx_hash::u64hash_bytes((RANDOM[i] as u64).as_ref(), seed));
        test!(xx_hash::u64hash_usize(RANDOM[i] as usize, seed)
                == xx_hash::u64hash_bytes((RANDOM[i] as usize).as_ref(), seed));
    }
}

static DATA: &'static [u8] = include_bytes!("testfile");
// RHS below produced by xxhsum in the xxhash distribution

#[test]
fn u64hash() {
    test!(xx_hash::u64hash_bytes(DATA, 0) == 0xd53d43d4bf1c86c5);
    test!(xx_hash::u64hash_bytes(DATA, 31) == 0xde5608413dbcb784);
}

#[test]
fn u32hash() {
    test!(xx_hash::u32hash_bytes(DATA, 0) == 0x2008167c);
    test!(xx_hash::u32hash_bytes(DATA, 31) == 0xec4bd9d9);
}

#[test]
fn u64hash_state() {
    let mut hasher = xx_hash::XxHash64::new(0u64);
    hasher.write_bytes(DATA);
    test!(hasher.digest() == 0xd53d43d4bf1c86c5);

    hasher.reset(31u64);
    hasher.write_bytes(DATA);
    test!(hasher.digest() == 0xde5608413dbcb784);
}

#[test]
fn u32hash_state() {
    let mut hasher = xx_hash::XxHash32::new(0u32);
    hasher.write_bytes(DATA);
    test!(hasher.digest() == 0x2008167c);

    hasher.reset(31u32);
    hasher.write_bytes(DATA);
    test!(hasher.digest() == 0xec4bd9d9);
}
