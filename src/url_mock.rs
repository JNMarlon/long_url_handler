#[cfg(test)]
mod tests {
    use crate::url::{base32_normalize, digest};
    use crate::url::truncate_base32;
    #[test]
    fn test_digest() {

        const LONG_URL: &str = "https://jinArchive.com/long-url-1";
        let digest_1: Vec<u8> = digest(LONG_URL);

        const DIGEST_TARGET_LEN: usize = 32;
        assert_eq!(DIGEST_TARGET_LEN, digest_1.len());
        assert_eq!(digest_1, digest(LONG_URL));
    }

    #[test]
    fn test_base32(){
        const TRUNCATE_LEN: usize = 2;
        const LONG_URL: &str = "https://long-url";
        let data = digest(LONG_URL);
        let base32 = truncate_base32(&data, TRUNCATE_LEN);

        const BASE_32_TARGET_LEN:usize = 4;
        const BASE_32_EXAMPLE:&str = "CGF0";
        assert_eq!(base32.len(),BASE_32_TARGET_LEN);
        assert_eq!(BASE_32_EXAMPLE,base32);

    }

    #[test]
    fn test_base32_normalize(){
        const BASE1:&str = "CGFO";
        const TARGET1:&str = "CGF0";

        assert_eq!(TARGET1,base32_normalize(BASE1).unwrap());
        const BASE2:&str = "1ilIA";
        const TARGET2:&str = "1111A";

        assert_eq!(TARGET2,base32_normalize(BASE2).unwrap())
    }

}
