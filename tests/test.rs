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
    use super::*;
    use tracing_test_macros::{test, tokio_test};

    #[test]
    fn test_macro() {
        assert_eq!(true, INIT.is_completed());
    }

    #[tokio_test]
    async fn test_tokio_macro() {
        assert_eq!(true, INIT.is_completed());
    }
}
