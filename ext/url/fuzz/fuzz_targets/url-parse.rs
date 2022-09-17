#![no_main]

use deno_core::OpState;
use libfuzzer_sys::fuzz_target;

#[derive(Debug, arbitrary::Arbitrary)]
enum Target {
  UrlParse(String),
  UrlReparse(String, u8, String),
  UrlParseSearchParams(Option<String>, Option<Vec<u8>>),
  UrlStringifySearchParams(Vec<(String, String)>),
}

fuzz_target!(|target: Target| {
  match target {
    Target::UrlParse(url) => {
      let mut op = OpState::new(64);
      let mut buf = [0; 32];
      let _ = deno_url::op_url_parse::call(&mut op, url, &mut buf);
    }
    Target::UrlReparse(url, setter, setter_val) => {
      let mut op = OpState::new(64);
      let mut buf = [0; 32];
      let _ = deno_url::op_url_reparse::call(
        &mut op, url, setter, setter_val, &mut buf,
      );
    }
    Target::UrlParseSearchParams(args, buf) => {
      let zc = buf.map(serde_v8::ZeroCopyBuf::new_temp);
      let _ = deno_url::op_url_parse_search_params::call(args, zc);
    }
    Target::UrlStringifySearchParams(args) => {
      let _ = deno_url::op_url_stringify_search_params::call(args);
    }
  };
});
