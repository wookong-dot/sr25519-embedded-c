use rand_core::{CryptoRng,RngCore};

pub struct ExternalRng
{
        pub rng_bytes: [u8;32],
        pub len: usize
}
impl ExternalRng
{
    pub fn set_rng(&self, dest: &mut [u8])
    {
        let mut k = 0;
        while k<self.len
        {
            dest[k] = self.rng_bytes[k];
            k= k+1;
        }
    }
}

impl RngCore for ExternalRng {
    fn next_u32(&mut self) -> u32 {  panic!()  }
    fn next_u64(&mut self) -> u64 {  panic!()  }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.set_rng(dest); 
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), ::rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}
impl CryptoRng for ExternalRng {}