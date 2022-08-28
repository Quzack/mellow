use mellow::Client;

#[test]
fn test_client_lt() {
    let c = Client::from_token("pretend-this-is-a-token");

    assert_eq!(c.intents.to_owned(), 0);
}