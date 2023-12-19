use secrets;

impl secrets::Secrets {
    pub fn get_access_token(&self) -> String {
        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "https://cloud.lightspeedapp.com/oauth/access_token.php?refresh_token={}&client_id={}&client_secret={}&grant_type=refresh_token",
                self.access_token,
                self.client_id,
                self.client_secret
            ))
            .send()
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();
        let access_token = res["access_token"].as_str().unwrap();
        access_token.to_string()
    }
}
