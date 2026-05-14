pub type IdHash = [u8; 8];
pub type PerceptualHash = [u8; 18];

pub fn hash_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x?}", b)).collect::<String>()
}
