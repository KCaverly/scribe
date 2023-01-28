use bytes::Bytes;

pub struct Link {
    url: String,
    title: String,
}

impl Link {
    fn normalize_to_file_path(title: &str) -> String {
        return title.replace(" ", "_").to_lowercase();
    }

    pub fn new(url: String, title: String) -> Self {
        return Self {
            url: url,
            title: title,
        };
    }

    pub fn get_from_web(&self) -> Bytes {
        // Name your user agent after your app?
        static APP_USER_AGENT: &str =
            concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        let client = reqwest::blocking::Client::builder()
            .user_agent(APP_USER_AGENT)
            .build()
            .expect("Count not build client!");

        let resp = client
            .get(&self.url)
            .send()
            .expect("Could not download file!");

        println!("URL: {}", self.url);
        let body = resp.bytes().expect("body invalid");
        // let mut out = File::create("test.pdf").expect("failed to create file");
        // out.write_all(&body);
        return body;
    }
}
