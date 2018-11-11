pub trait App {
    fn get_response(
        &self,
        path: &str,
        request_body: &[u8],
    ) -> Vec<u8>;
}
