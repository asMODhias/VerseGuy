use ed25519_dalek::Keypair;
use std::sync::Arc;
use verseguy_storage::RocksDBStorage;

pub struct AppState {
    pub storage: Arc<RocksDBStorage>,
    pub license_secret: Vec<u8>,
    pub keypair: Option<Keypair>,
    /// Optional Prometheus metrics handle used by the /metrics endpoint
    pub metrics_handle: Option<metrics_exporter_prometheus::PrometheusHandle>,
}

impl AppState {
    pub fn new<P: AsRef<std::path::Path>>(
        path: P,
        license_secret: Vec<u8>,
    ) -> anyhow::Result<Self> {
        let storage = RocksDBStorage::open(path)?;

        // If MASTER_KEY_FILE is set, load (or generate) the signing key there; otherwise generate ephemeral keypair
        let keypair = if let Ok(key_path) = std::env::var("MASTER_KEY_FILE") {
            let kp = crate::keystore::load_or_generate(std::path::Path::new(&key_path))?;
            Some(kp)
        } else {
            let mut csprng = rand::rngs::OsRng {};
            let kp = Keypair::generate(&mut csprng);
            Some(kp)
        };

        Ok(Self {
            storage: Arc::new(storage),
            license_secret,
            keypair,
            metrics_handle: None,
        })
    }
}
