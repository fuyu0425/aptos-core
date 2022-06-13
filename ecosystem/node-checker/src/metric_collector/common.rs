use reqwest::Url;

fn fix_url(url: &mut Url) {
    if !url.path().contains("metrics") {
        url.set_path("metrics");
    }
}
