#[cfg(test)]
mod test {
    use std::time::Duration;
    use tokio::runtime::{Handle, RuntimeFlavor};
    use tracing::info;
    use tracing_test_macros::{test, tokio_test};

    #[test]
    fn test_macro() {
        info!("custom test macro");
        assert!(tracing_subscriber::fmt().try_init().is_err());
    }

    #[tokio_test]
    async fn test_tokio_macro() {
        info!("custom tokio test macro");
        assert!(tracing_subscriber::fmt().try_init().is_err());
        let handle = Handle::current();
        assert!(matches!(
            handle.runtime_flavor(),
            RuntimeFlavor::MultiThread
        ));
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
