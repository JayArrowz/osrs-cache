mod common;

mod osrs {
    use super::common;
    use osrscache::{checksum::Checksum, util::Huffman, Cache};

    #[test]
    fn setup_cache() -> osrscache::Result<()> {
        let cache = common::osrs::setup();

        assert!(cache.is_ok());

        Ok(())
    }

    #[test]
    fn setup_cache_fails() -> osrscache::Result<()> {
        let result = Cache::new("./wrong/path");

        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn create_checksum() -> osrscache::Result<()> {
        let cache = common::osrs::setup()?;

        assert!(Checksum::new(&cache).is_ok());

        Ok(())
    }

    #[test]
    fn encode_checksum() -> osrscache::Result<()> {
        let cache = common::osrs::setup()?;

        let buffer = Checksum::new(&cache)?.encode()?;

        let hash = common::hash(&buffer);
        assert_eq!(&hash, "0cb64350dc138e91bb83bc9c84b454631711f5de");
        assert_eq!(buffer.len(), 173);

        Ok(())
    }

    #[test]
    fn validate_checksum() -> osrscache::Result<()> {
        let cache = common::osrs::setup()?;
        let checksum = Checksum::new(&cache)?;

        let crcs: Vec<u32> = vec![
            1593884597, 1029608590, 16840364, 4209099954, 3716821437, 165713182, 686540367,
            4262755489, 2208636505, 3047082366, 586413816, 2890424900, 3411535427, 3178880569,
            153718440, 3849392898, 3628627685, 2813112885, 1461700456, 2751169400, 2927815226,
        ];

        assert!(checksum.validate(&crcs).is_ok());

        Ok(())
    }

    #[test]
    fn get_huffman_table() -> osrscache::Result<()> {
        let cache = common::osrs::setup()?;

        let huffman_table = cache.huffman_table()?;

        let hash = common::hash(&huffman_table);
        assert_eq!(&hash, "664e89cf25a0af7da138dd0f3904ca79cd1fe767");
        assert_eq!(huffman_table.len(), 256);

        Ok(())
    }

    #[test]
    fn huffman_decompress() -> osrscache::Result<()> {
        let cache = common::osrs::setup()?;

        let huffman_table = cache.huffman_table()?;
        let huffman = Huffman::new(&huffman_table);

        let compressed_msg = &[174, 128, 35, 32, 208, 96];
        let decompressed_len = 8;
        let decompressed_msg = huffman.decompress(compressed_msg, decompressed_len);
        let msg = String::from_utf8(decompressed_msg).unwrap_or_default();
        assert_eq!(msg, "rs-cache");

        Ok(())
    }
}
