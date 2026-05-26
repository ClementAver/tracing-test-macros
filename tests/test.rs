#[cfg(test)]
static INIT: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
static RUNTIME: std::sync::LazyLock<tokio::runtime::Runtime> = std::sync::LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("failed to init tokio runtime")
});

#[cfg(test)]
mod test {
    use crate::RUNTIME;
    use test_macros::{test, tokio_test};

    #[test]
    fn test_macro() {
        let hello = "Hello, world!";
        assert_eq!("Hello, world!", hello);
    }

    #[tokio_test]
    fn tokio_test_macro() {
        assert_eq!("Hello, world!", awaited().await.unwrap());
        async fn awaited<'a>() -> Result<&'a str, ()> {
            Ok("Hello, world!")
        }
    }
}
