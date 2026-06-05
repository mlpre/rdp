use std::fs;
use std::process::Command;
use url::Url;

fn main() {
    let url = fs::read_to_string("rdp.txt")
        .expect("Not found rdp.txt")
        .trim()
        .to_string();
    let agent =
        ureq::Agent::new_with_config(ureq::Agent::config_builder().max_redirects(0).build());
    let response = agent.post(&url).send_empty().expect("Request failure");
    let location = response
        .headers()
        .get("location")
        .expect("Not found location")
        .to_str()
        .expect("Location format error");
    let remote_url = Url::parse(location).expect("Location format error");
    let host = remote_url.host_str().expect("Not host");
    let port = remote_url.port().expect("Not port");
    Command::new("mstsc")
        .args([format!("/v:{host}:{port}")])
        .spawn()
        .expect("mstsc error");
}
