#[link(name = "sphincsplus", kind = "static")]
extern "C" {
    // int crypto_sign_keypair(unsigned char *pk, unsigned char *sk)
    fn crypto_sign_keypair(pk: *mut u8, sk: *mut u8) -> i32;
    // int crypto_sign(unsigned char *sm, unsigned long long *smlen,
    //     const unsigned char *m, unsigned long long mlen,
    //     const unsigned char *sk)
    fn crypto_sign(sm: *mut u8, smlen: *mut u64, m: *const u8, mlen: u64, sk: *const u8) -> i32;

    // int crypto_sign_open(unsigned char *m, unsigned long long *mlen,
    //     const unsigned char *sm, unsigned long long smlen,
    //     const unsigned char *pk)
    fn crypto_sign_open(
        m: *mut u8,
        mlen: *mut u64,
        sm: *const u8,
        smlen: u64,
        pk: *const u8,
    ) -> i32;
}

pub struct SphincsPlus {
    pub pk: [u8; 128],
    pub sk: [u8; 128],
}

impl SphincsPlus {
    pub fn new() -> Self {
        let mut s = Self {
            pk: [0u8; 128],
            sk: [0u8; 128],
        };
        let ret = unsafe { crypto_sign_keypair(s.pk.as_mut_ptr(), s.sk.as_mut_ptr()) };
        if ret != 0 {
          panic!("gen keypair failed");
        }

        s
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        let mut s = Vec::new();
        s.resize(29792 + 32, 0);
        let mut sm_len = s.len() as u64;

        let ret = unsafe {
            crypto_sign(
                s.as_mut_ptr(),
                &mut sm_len,
                msg.as_ptr(),
                msg.len() as u64,
                self.sk.as_ptr(),
            )
        };
        assert_eq!(ret, 0);

        s
    }

    pub fn verify(&self, sign: &[u8]) -> Result<(), ()> {
        let mut sm = Vec::new();
        sm.resize(32, 0xFF);
        let mut smlen = sm.len() as u64;

        unsafe {
            crypto_sign_open(
                sm.as_mut_ptr(),
                &mut smlen,
                sign.as_ptr(),
                sign.len() as u64,
                self.pk.as_ptr(),
            );
        }

        Ok(())
    }
}
