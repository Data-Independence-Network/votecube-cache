pub trait App {

    fn get_response(
        &self,
        path: &str,
        request_body: &[u8],
    ) -> Vec<u8>;

    fn get_update_response(
        &self,
        _path: &str,
        _request_body: &[u8],
    ) -> Vec<u8>;

}
