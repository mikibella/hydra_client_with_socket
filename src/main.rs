#[tokio::main]
async fn main() {
    let mut ory_config = ory_hydra_client::apis::configuration::Configuration::new();
    ory_config.base_path = "http://hydra:4444".to_string();

    //requeet health status and print to console
    let alive = ory_hydra_client::apis::metadata_api::is_alive(&ory_config)
        .await
        .unwrap();

    print!("{:?}", alive);
}
