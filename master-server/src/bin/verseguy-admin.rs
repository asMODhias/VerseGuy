use master_server::admin_cli;

fn main() -> anyhow::Result<()> {
    admin_cli::run_from_std_args()
}
