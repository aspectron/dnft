cfg_if::cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        mod result;
        mod error;
        use result::*;
        use clap::{Parser, Subcommand};

        #[derive(Debug, Parser)]
        #[clap(name = "mint")]
        #[clap(about, author, version)]
        struct Args {
            #[clap(subcommand)]
            action: Action,
        }

        #[derive(Subcommand, Debug)]
        enum Action {
            /// Mint Delta NFT
            Mint {

            },
        }

        pub async fn async_main() -> Result<()> {
            let Args {
                action,
            } = Args::parse();


            match action {
                Action::Mint {} => {
                    println!("mint...");
                }
            }

            Ok(())
        }

        #[tokio::main]
        async fn main() -> Result<()> {
            if let Err(err) = async_main().await {
                println!("{err}");
                std::process::exit(1);
            }

            Ok(())
        }
    }
    else if #[cfg(target_arch = "wasm32")] {
        fn main() { }
    }
}
