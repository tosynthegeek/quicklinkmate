#[cfg(test)]
mod test {
    fn test_shorturl_generator() {
        let dt = Local::now();
        let initial_url_1 =
            "https://github.com/tosynthegeek/trademark-nickname-lol/blob/main/src/main.rs";
        let short_url_1 = generate_short_url(initial_url_1, dt);

        let initial_url_2 =
            "https://docs.rs/bitcoin-base58/0.1.16-alpha.0/bitcoin_base58/fn.encode_base58.html";
        let short_url_2 = generate_short_url(initial_url_2, dt);

        let initial_url_3 =
            "https://docs.google.com/document/d/1AP5qHJukdAns-_FEfEfLRZQ1YDmjgMIzmRrVwnXd-bQ/edit";
        let short_url_3 = generate_short_url(initial_url_3, dt);

        assert_eq(short_url_1); // Come back to this
        assert_eq(short_url_2); // Come back to this
        assert_eq(short_url_3); // Come back to this
    }
}
