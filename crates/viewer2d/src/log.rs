pub(crate) fn setup_logging() {
    use tracing_subscriber::prelude::*;

    let subscriber =
        tracing_subscriber::Registry::default().with(tracing_tree::HierarchicalLayer::new(2));
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
