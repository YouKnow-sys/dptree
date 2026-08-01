#[cfg(target_arch = "wasm32")]
fn main() {
    // This example uses tokio::spawn/multithreading, which isn't
    // supported on wasm32-unknown-unknown.
}

#[cfg(not(target_arch = "wasm32"))]
fn assert_num_string_handler(
    expected_num: u32,
    expected_string: &'static str,
) -> dptree::Endpoint<'static, ()> {
    // The handler requires `u32` and `String` types from the input storage.
    dptree::endpoint(move |num: u32, string: String| async move {
        assert_eq!(num, expected_num);
        assert_eq!(string, expected_string);
    })
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Init the storage with `u32` and `String` values.
    let store = dptree::deps![10u32, "Hello".to_owned()];

    let h = assert_num_string_handler(10u32, "Hello");

    let _ = h.dispatch(store.clone()).await;

    // This will cause a panic because we do not store `Ipv4Addr` in out store.
    let handle = tokio::spawn(async move {
        let ip_handler: dptree::Endpoint<_> =
            dptree::endpoint(|ip: std::net::Ipv4Addr| async move {
                assert_eq!(ip, std::net::Ipv4Addr::new(0, 0, 0, 0));
            });
        let _ = ip_handler.dispatch(store).await;
    });
    let result = handle.await;
    assert!(result.is_err())
}
