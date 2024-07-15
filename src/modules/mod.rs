pub mod ip;

#[macro_export]
macro_rules! execute {
    ($($module:path),*) => {
        async {
            let mut tasks = vec![];
            $(
                tasks.push(tokio::spawn(async { let _ = $module().await; }));
            )*

            for task in tasks {
                let _ = task.await;
            }
        }
    };
}
