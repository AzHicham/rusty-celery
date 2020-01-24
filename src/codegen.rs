#[macro_export]
macro_rules! celery_app {
    (
        $name:ident,
        broker_url = $broker_url:expr,
        default_queue = $default_queue:expr,
        tasks = [ $( $t:ty ),* ],
    ) => {
        lazy_static! {
            static ref $name: Celery<AMQPBroker> = {
                let broker_url = $broker_url;
                let default_queue = $default_queue;

                // Initialize broker.
                let broker = AMQPBroker::builder(&broker_url)
                    .queue(default_queue)
                    .prefetch_count(2)
                    .build()
                    .unwrap();

                // Initialize Celery app.
                let celery = Celery::builder("celery", broker)
                    .default_queue_name(default_queue)
                    .build();

                // Register tasks.
                $(
                    celery.register_task::<$t>().unwrap();
                )*

                celery
            };
        }
    };
}