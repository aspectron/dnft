#[cfg(test)]
pub mod tests {
    use crate::prelude::*;
    // use super::*;
    use kaizen::result::Result;
    use std::str::FromStr;

    const AUTHORITY: &str = "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f";

    #[async_std::test]
    async fn example_test() -> Result<()> {
        kaizen::init()?;

        println!("init transport...");
        Transport::try_new_for_unit_tests(
            crate::program_id(),
            Some(Pubkey::from_str(AUTHORITY)?),
            TransportConfig::default(),
        )
        .await?;
        println!("run test...");

        run_test().await?;

        log_info!("");
        Transport::global()?
            .simulator()
            .store
            .list()
            .await?
            .to_log();
        log_info!("");

        log_trace!("all looks good ... 😎");

        Ok(())
    }

    #[wasm_bindgen]
    pub async fn run_test() -> Result<()> {
        let transport = Transport::global()?;
        if let Some(emulator) = transport.emulator() {
            let authority = Pubkey::from_str(AUTHORITY)?;
            transport.set_custom_authority(Some(authority))?;
            emulator
                .fund(
                    &authority,
                    &Pubkey::default(),
                    utils::sol_to_lamports(500.0),
                )
                .await?;
        }

        let authority = transport.get_authority_pubkey()?;

        // ----------------------------------------------------------------------------

        log_info!("creating root");
        let args = program::RootCreationArgs {};
        let tx = client::Root::create(&authority, &args).await?;
        let target_account_pubkey = tx.target_account()?;
        tx.execute().await?;
        let root_container = load_container::<program::Root>(&target_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");
        log_info!("root creation ok {}", root_container.pubkey());

        // ----------------------------------------------------------------------------
        use program::{DataType, MintCreationArgs};

        log_info!("creating mint");
        let mut data_types = Vec::<DataType>::new();

        data_types.push(DataType::u32);
        data_types.push(DataType::u8);
        data_types.push(DataType::String);
        data_types.push(DataType::u64);

        let args = MintCreationArgs {
            data_types: Some(data_types),
            ..MintCreationArgs::default()
        };

        let tx = client::Mint::create(&authority, &args).await?;
        let mint_account_pubkey = tx.target_account()?;
        tx.execute().await?;
        let mint_container = load_container::<program::Mint>(&mint_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");
        log_info!("mint creation ok {}", mint_container.pubkey());

        // ----------------------------------------------------------------------------

        for n in 0..10 {
            log_info!("creating token {n}");

            let mint_container = reload_container::<program::Mint>(&mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");

            let args = program::TokenCreationArgs {};
            let tx = client::Token::create(&authority, mint_container.pubkey(), &args).await?;
            let target_account_pubkey = tx.target_account()?;
            tx.execute().await?;
            let token_container = load_container::<program::Token>(&target_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");
            log_info!("");
            log_info!("token {n} creation ok {}", token_container.pubkey());
        }

        // ----------------------------------------------------------------------------

        let mint_container = reload_container::<program::Mint>(&mint_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");

        let token_len = mint_container.tokens.len();
        log_info!("... tokens created: {token_len}");

        let schema = mint_container.data_types.load()?;
        log_info!("\n\nmint container schema: {:#?}\n", schema);

        Ok(())
    }
}
