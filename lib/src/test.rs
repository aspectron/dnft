#[cfg(not(target_os = "solana"))]
pub mod tests {
    use crate::client::schema::Schema;
    use crate::prelude::*;
    use kaizen::result::Result;
    use program::MintCreationArgs;
    use std::str::FromStr;
    use workflow_log::style;

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
        const MAX_MINTS: usize = 3;
        const MAX_TOKENS: usize = 5;
        // ----------------------------------------------------------------------------

        let mut mint_pubkeys = vec![];

        for mint_seq in 0..MAX_MINTS {
            log_info!("creating mint {mint_seq}");
            let data_types = vec![
                program::DataType::u32,
                program::DataType::u8,
                program::DataType::String,
                program::DataType::u64,
            ];

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
            assert_eq!(&mint_account_pubkey, mint_container.pubkey());
            mint_pubkeys.push(mint_account_pubkey);
            log_info!("mint {mint_seq} creation ok - {}", mint_container.pubkey());
        }

        // ----------------------------------------------------------------------------

        for mint_seq in 0..MAX_MINTS {
            for token_seq in 0..MAX_TOKENS {
                log_info!("creating token {mint_seq}:{token_seq}");

                let mint_account_pubkey = mint_pubkeys.get(mint_seq).unwrap();
                let mint_container = reload_container::<program::Mint>(mint_account_pubkey)
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
                log_info!(
                    "token {mint_seq}:{token_seq} creation ok - {}",
                    token_container.pubkey()
                );
            }
        }
        // ----------------------------------------------------------------------------

        for (idx, pk) in mint_pubkeys.iter().enumerate() {
            log_info!("mint pubkey {idx} : {pk}");
        }

        let root = reload_container::<program::Root>(&client::Root::pubkey())
            .await?
            .expect("¯\\_(ツ)_/¯");

        let max_mints = root.mints.len();
        for mint_seq in 0..max_mints {
            let mint_account_pubkey = root
                .mints
                .get_pubkey_at(&crate::program_id(), mint_seq as u64)?;
            log_info!("domain: {:?}", root.mints);
            log_info!(
                "{}",
                style(format!(
                    "reloading mint pubkey {mint_seq} : {mint_account_pubkey}"
                ))
                .red()
            );
            // let mint_account_pubkey = mint_pubkeys.get(mint_seq).unwrap();
            let mint_container = reload_container::<program::Mint>(&mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");

            let token_len = mint_container.tokens.len();
            log_info!("mint {mint_account_pubkey} ... tokens: {token_len}");

            for token_seq in 0..token_len {
                let token_account_pubkey = mint_container
                    .tokens
                    .get_pubkey_at(&crate::program_id(), token_seq as u64)?;
                let _token = reload_container::<program::Token>(&token_account_pubkey)
                    .await?
                    .expect("¯\\_(ツ)_/¯");

                log_info!("\ttoken {token_seq} {token_account_pubkey}");
            }

            // let schema = mint_container.data_types.load()?;
            // log_info!("\n\nmint container schema: {:#?}\n", schema);
        }

        Ok(())
    }

    #[wasm_bindgen(js_name = "createDnftMint")]
    pub async fn create_dnft_mint(schema: Schema) -> Result<Pubkey> {
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

        log_info!("creating root ----------------------------------------------------------------------------");
        let args = program::RootCreationArgs {};
        let tx = client::Root::create(&authority, &args).await?;
        let target_account_pubkey = tx.target_account()?;
        tx.execute().await?;
        let root_container = load_container::<program::Root>(&target_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");
        log_info!("root creation ok {}", root_container.pubkey());

        // ----------------------------------------------------------------------------

        log_info!("creating mint ----------------------------------------------------------------------------");
        let args: MintCreationArgs = schema.into();

        let tx = client::Mint::create(&authority, &args).await?;
        let mint_account_pubkey = tx.target_account()?;
        tx.execute().await?;
        let mint_container = load_container::<program::Mint>(&mint_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");
        let key = mint_container.pubkey();
        log_info!("mint creation ok {}", key);

        Ok(*key)
    }
}
