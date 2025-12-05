use encoding_rs::Encoding;
use napi_derive::napi;

pub(crate) fn is_mime_type_text(mime_type: &str) -> bool {
    mime_type.starts_with("text/")
        || matches!(
            mime_type,
            "application/json"
                | "application/xml"
                | "application/javascript"
                | "application/typescript"
                | "application/x-javascript"
                | "application/xhtml+xml"
                | "application/ld+json"
        )
}

pub(crate) fn decode_text(content: &[u8], encoding_name: &str) -> String {
    let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    let (decoded, _encoding_used, had_errors) = encoding.decode(content);

    if had_errors {
        String::new()
    } else {
        decoded.to_string()
    }
}

#[napi]
#[allow(dead_code)]
pub fn extract_text_content(content: napi::bindgen_prelude::Buffer, encoding: String) -> String {
    decode_text(content.as_ref(), &encoding)
}
