#[cfg(not(target_os = "solana"))]
pub mod tests {
    use crate::api::*;
    use crate::prelude::*;
    use kaizen::result::Result;
    use program::MintCreationArgs;
    use std::str::FromStr;

    const AUTHORITY: &str = "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f";

    #[async_std::test]
    async fn example_test() -> Result<()> {
        kaizen::init()?;

        const USE_EMULATOR: bool = false;

        println!("init transport...");
        let transport = if USE_EMULATOR {
            Transport::try_new_for_unit_tests(
                crate::program_id(),
                Some(Pubkey::from_str(AUTHORITY)?),
                TransportConfig::default(),
            )
            .await?
        } else {
            Transport::try_new("http://127.0.0.1:8899", TransportConfig::default()).await?
        };

        println!("run test...");

        //run_test().await?;
        run_mint_test().await?;

        log_info!("");
        if transport.mode().is_emulator() {
            transport.simulator().store.list().await?.to_log();
            log_info!("");
        }

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
        let root = reload_container::<program::Root>(&client::Root::pubkey()).await?;
        if root.is_none() {
            log_info!("creating root");
            let args = program::RootCreationArgs {};
            let tx = client::Root::create(&authority, &args).await?;
            let target_account_pubkey = tx.target_account()?;
            tx.execute().await?;
            let root_container = load_container::<program::Root>(&target_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");
            log_info!("root creation ok {}", root_container.pubkey());
        }

        // ----------------------------------------------------------------------------
        const MAX_MINTS: usize = 1;
        const MAX_TOKENS: usize = 2;
        // ----------------------------------------------------------------------------

        let mut mint_pubkeys = vec![];

        for mint_seq in 0..MAX_MINTS {
            log_info!("creating mint {mint_seq}");
            let data_types = vec![
                program::DataType::u32,
                program::DataType::u8,
                program::DataType::String,
                program::DataType::u64,
                program::DataType::Url,
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

                let args = program::TokenCreateFinalArgs {
                    available: token_seq as u8,
                    data: vec![
                        program::Data::u32(20),
                        program::Data::u8(token_seq as u8),
                        program::Data::String("hello".to_string()),
                        program::Data::u64(5),
                        program::Data::Url(program::Url::Image(
                            "https://tinyurl.com/mzs8fxya".to_string(),
                        )),
                    ],
                };
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

        let root = reload_container::<program::Root>(&client::Root::pubkey())
            .await?
            .expect("¯\\_(ツ)_/¯");

        let max_mints = root.mints.len();
        for mint_seq in 0..max_mints {
            let mint_account_pubkey = root
                .mints
                .get_pubkey_at(&crate::program_id(), mint_seq as u64)?;
            let mint_container = reload_container::<program::Mint>(&mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");

            let token_len = mint_container.tokens.len();
            log_info!("mint {mint_seq} {mint_account_pubkey} [{token_len}]");

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

    #[wasm_bindgen]
    pub async fn run_mint_test() -> Result<()> {
        let transport = Transport::global()?;
        let pubkeys = crate::client::root::Root::get_mint_pubkeys(0, 100).await?;
        log_trace!("mint pubkeys: {:?}", pubkeys);
        for pubkey in pubkeys {
            let data = crate::client::mint::Mint::get_data(pubkey).await?;
            log_trace!(
                "mint {:02X?} => data: {:?}",
                pubkey.to_bytes().to_vec(),
                data
            );

            let config = GetProgramAccountsConfig::new()
                .add_filters(vec![
                    GetProgramAccountsFilter::MemcmpEncodedBase58(8, pubkey.to_string()),
                    GetProgramAccountsFilter::MemcmpEncodeBase58(40, vec![1]),
                ])?
                .encoding(UiAccountEncoding::Base64)?
                .build()?;

            let accounts = transport
                .get_program_accounts_with_config(&crate::program_id(), config)
                .await?;

            log_trace!("accounts: {accounts:#?}");
        }

        Ok(())
    }
}
