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
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use backtrace::Backtrace;
    use rust_errors::RaccourciBacktrace;
    use crate::RUNTIME;
    use salvo::prelude::*;
    use tracing::warn;
    use test_macros::{test, tokio_test};

    #[test]
    fn test_macro() {
        let hello = "Hello, world!";
        assert_eq!("Hello, world!", hello);
    }

    #[tokio_test]
    async fn tokio_test_macro() {
        assert_eq!("Hello, world!", awaited().await.unwrap());
        async fn awaited<'a>() -> Result<&'a str, ()> {
            Ok("Hello, world!")
        }
    }

    #[tokio_test]
    async fn tokio_test_salvo() {
        let acceptor = TcpListener::new("0.0.0.0:80").bind().await;
        let router = Router::new().get(hello);
        Server::new(acceptor).serve(router).await;

        #[endpoint(
        summary = "/",
        description = "Hello World",
        responses(
        (status_code = 200, body = String),
        (status_code = 500)
        )
        )]
        async fn hello(res: &mut Response) {
            match hello_service() {
                Ok(it) => res.render(it),
                Err(err) => {
                    warn!("An error occurred: {}\nBacktrace:\n{:?}", err, err.backtrace);

                    res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
                },
            }
        }
    }

    fn hello_service() -> Result<String, MyError> {
        // Ok("Hello, World!".to_string())
        Err(MyError::new("Good bye, cruel world!"))
    }

    #[derive(Debug)]
    struct MyError {
        pub backtrace: RaccourciBacktrace,
        message: &'static str
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.message)
        }
    }

    impl Error for MyError {}

    impl MyError {
        pub fn new(message: &'static str) -> Self {
            Self {
                backtrace: RaccourciBacktrace(Backtrace::new()),
                message
            }
        }
    }
}
