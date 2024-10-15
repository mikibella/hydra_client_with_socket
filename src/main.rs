#[tokio::main]
async fn main() {}
#[cfg(test)]
mod tests {
    use hyper::{body::Body, client::conn, Request};
    use reqwest::Client;
    use tokio::net::UnixStream;

    #[tokio::test]
    async fn test_if_admin_api_is_reachable() {
        let mut ory_config = ory_hydra_client::apis::configuration::Configuration::new();
        ory_config.base_path = "http://hydra:4445".to_string();

        //requeet client form admin api
        let response = ory_hydra_client::apis::o_auth2_api::list_o_auth2_clients(
            &ory_config,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

        assert_eq!(response.len(), 0);
    }
    #[tokio::test]
    async fn test_if_public_api_is_reachable_over_unix_socket() {
        

}
