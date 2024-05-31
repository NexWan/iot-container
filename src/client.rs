use embedded_svc::io::Write;
use serde_json::json;
use esp_idf_svc::http::{
    client::EspHttpConnection,
    client::Configuration
};
use embedded_svc::http::client::Client as HttpClient;
use esp_idf_hal::gpio::{PinDriver, AnyOutputPin, Output};
use std::time::Duration;
// use embedded_svc::http::client::Response;

pub struct Client {
    http_client: HttpClient<EspHttpConnection>,
    base_url: String,
    container_id: String
}

impl Client {
    pub fn new(
        base_url: String,
        container_id: String
    ) -> anyhow::Result<Client> {
        let connection = EspHttpConnection::new(&Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            timeout: Some(Duration::from_secs(20)),
            ..Default::default()
        })?;

        let http_client = HttpClient::wrap(connection);
        
        Ok (
            Client { 
                http_client,
                base_url,
                container_id
            }
        )
    }

    pub fn process_request(
        &mut self,
        state: u8,
        led: &mut PinDriver<AnyOutputPin, Output>
    ) -> anyhow::Result<()> {

        led.set_high()?;
        self.send_data(state)?;
        led.set_low()?;

        Ok(())
    }

    fn send_data(
        &mut self,
        state: u8
    ) -> anyhow::Result<()> {
        // Define the JSON body
        let state = &format!("{}", state);
        println!("state: {}, container: {}", &state, 1);
        let body = json!({
            "lleno": &state,
            "id": 1 //This is hardoded since I'm using it for testing
        });
        let binding = body.to_string();

        let content_length_header = format!("{}", binding.len());

        let mut _tunnelReminder = ""; //Since I'm using localtunnel I need to have a header to
                                     //bypass the password in localtunnel

        if state.eq("1"){
            _tunnelReminder = "true";
        }else{
            _tunnelReminder = "false";
        }

        let headers = [
            ("content-type", "application/json"),
            ("content-length", &*content_length_header),
            ("bypass-tunnel-reminder", &_tunnelReminder)
        ];

        let mut request = self
            .http_client
            .post(
                &self.base_url,
                &headers
            )?;

        request.write_all(binding.as_bytes())?;

        request.flush()?;

        let response = request.submit()?; //This needs to be handled



        println!("{}", response.status());

        Ok(())
    }
}
