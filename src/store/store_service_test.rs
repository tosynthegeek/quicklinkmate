mod store_service;
#[cfg(test)]
mod tests {
    mod store_service;
    #[test]
    fn init() {
        let test_stote_service: &StorageService = store_service::initialize_store();
        assert!(test_stote_service.redis_client != None);
    }

    #[test]
    fn test_save_retrieve() {
        let short_url = "Somedummyvalue";
        let original_url =
            "https://github.com/tosynthegeek/trademark-nickname-lol/blob/main/src/main.rs";

        save_url_mapping(short_url, original_url);
        assert_eq!(original_url, initial_url(short_url));
    }
}
