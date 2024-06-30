mod scenario;

mod state_prepare;

mod state_send_request;

mod state_send_body;

mod state_recv_response;

trait TestSliceExt {
    fn as_str(&self) -> &str;
}

impl TestSliceExt for [u8] {
    fn as_str(&self) -> &str {
        std::str::from_utf8(self).unwrap()
    }
}
