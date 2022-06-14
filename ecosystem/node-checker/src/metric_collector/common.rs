// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use reqwest::Url;

pub const METRICS_ENDPOINT: &str = "metrics";

fn fix_url(url: &mut Url) {
    if !url.path().contains(METRICS_ENDPOINT) {
        url.set_path(METRICS_ENDPOINT);
    }
}
