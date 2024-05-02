use embedded_svc::io::Write;
use serde_json::json;
use esp_idf_svc::http::{
    client::EspHttpConnection,
    client::Configuration
};
use embedded_svc::http::client::Client as HttpClient;
// use embedded_svc::http::client::Response;

pub struct Client {
    http_client: HttpClient<EspHttpConnection>,
    base_url: String
}

impl Client {
    pub fn new(base_url: String) -> anyhow::Result<Client> {
        let connection = EspHttpConnection::new(&Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Default::default()
        })?;

        let http_client = HttpClient::wrap(connection);
        
        Ok (
            Client { 
                http_client,
                base_url
            }
        )
    }

    pub fn post_request(&mut self, state: u8) -> anyhow::Result<()> {
        // Define the JSON body
        let body = json!({
            "lleno": state,
            "id_": "1"
        });
        let binding = body.to_string();

        let content_length_header = format!("{}", binding.len());

        let headers = [
            ("content-type", "application/json"),
            ("content-length", &*content_length_header)
        ];

        let mut request = self
            .http_client
            .post(
                &self.base_url,
                &headers
            )?;

        request.write_all(binding.as_bytes())?;
        request.flush()?;

        let response = request.submit()?;

        println!("{}", response.status());

        Ok(())
    }
}
