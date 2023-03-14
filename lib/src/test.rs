#[cfg(not(target_os = "solana"))]
pub mod tests {
    use crate::{
        client::{Mint, Root, Token},
        program,
        program::{ExchangeMechanics, ImageUrl, SaleType},
        program_id,
    };
    use kaizen::{prelude::*, result::Result, utils::sol_to_lamports};

    #[async_std::test]
    async fn example_test() -> Result<()> {
        kaizen::init()?;
        use std::str::FromStr;

        const USE_EMULATOR: bool = false;
        let with_sample_data = true;
        const AUTHORITY: &str = "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f";

        println!("init transport...");
        let transport = if USE_EMULATOR {
            Transport::try_new_for_unit_tests(
                program_id(),
                Some(Pubkey::from_str(AUTHORITY)?),
                TransportConfig::default(),
            )
            .await?
        } else {
            Transport::try_new("http://127.0.0.1:8899", TransportConfig::default()).await?
        };

        create_root().await?;
        if with_sample_data {
            create_sample_data().await?;
        }

        log_info!("");
        if transport.mode().is_emulator() {
            transport.simulator().store.list().await?.to_log();
            log_info!("");
        }

        log_trace!("all looks good ... 😎");

        Ok(())
    }

    pub async fn create_root() -> Result<()> {
        let root = reload_container::<program::Root>(&Root::pubkey()).await?;
        if root.is_none() {
            log_info!("creating root");
            let transport = Transport::global()?;
            let authority = transport.get_authority_pubkey()?;
            let args = program::RootCreationArgs {};
            let tx = Root::create(&authority, &args).await?;
            let target_account_pubkey = tx.target_account()?;
            tx.execute().await?;
            let root_container = load_container::<program::Root>(&target_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");
            log_info!("root creation ok {}", root_container.pubkey());
        }

        Ok(())
    }

    pub async fn create_sample_data() -> Result<()> {
        log_info!("creating sample data");

        let transport = Transport::global()?;
        let authority = transport.get_authority_pubkey()?;
        let mint_images = vec![
            "https://images.pexels.com/photos/1108099/pexels-photo-1108099.jpeg?auto=compress&cs=tinysrgb&w=1000&h=400&dpr=2",
            "https://images.pexels.com/photos/45170/kittens-cat-cat-puppy-rush-45170.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
            "https://images.pexels.com/photos/5011647/pexels-photo-5011647.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
            "https://images.pexels.com/photos/9436715/pexels-photo-9436715.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
            "https://images.pexels.com/photos/4818709/pexels-photo-4818709.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
            "https://images.pexels.com/photos/1475938/pexels-photo-1475938.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
            "https://images.pexels.com/photos/2640604/pexels-photo-2640604.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        ];

        let images = vec![
            "https://tinyurl.com/3nnzazpv",
            "https://images.freeimages.com/365/images/previews/f7e/abstract-rounded-rectangles-vector-graphic-3664.jpg",
            "https://images.freeimages.com/365/images/previews/953/ham-pattern-background-17545.jpg",
            "https://images.freeimages.com/365/images/previews/c34/abstract-colorful-yarn-background-vector-free-34204.jpg",
            "https://images.freeimages.com/365/images/previews/f84/tulip-vector-bouquet-illustration-free-33999.jpg",
            "https://images.freeimages.com/vhq/images/previews/7ae/exploding-gift-box-with-colorful-star-for-celebration-74578.jpg",
            "https://images.freeimages.com/365/images/previews/def/abstract-colorful-light-waves-vector-background-3259.jpg",
            "https://images.freeimages.com/vhq/images/previews/a6e/abstract-blue-wave-background-vector-graphic-73825.jpg",
            "https://images.freeimages.com/vhq/images/previews/9ea/bright-stars-gorgeous-special-effects-02-vector-6216.jpg",
            "https://images.freeimages.com/vhq/images/previews/f3d/gorgeous-diploma-certificate-template-02-vector-6248.jpg",
        ];
        let mint_names = vec!["The Golden Dog", "The Golden Cat"];
        let names = vec![
            "The Golden Dog - A",
            "The Golden Dog - B",
            "The Golden Dog - C",
            "The Golden Dog - D",
            "The Golden Dog - E",
            "The Golden Cat - A",
            "The Golden Cat - B",
            "The Golden Cat - C",
            "The Golden Cat - D",
            "The Golden Cat - E",
        ];

        // ----------------------------------------------------------------------------
        const MAX_MINTS: usize = 1;
        const MAX_TOKENS: usize = 1;
        // ----------------------------------------------------------------------------

        let mut mint_pubkeys = vec![];

        let root = reload_container::<program::Root>(&Root::pubkey())
            .await?
            .unwrap();
        let mut mint_index = root.mints.len();

        for mint_seq in 0..MAX_MINTS {
            log_info!("creating mint {mint_seq}");
            let data_types = vec![
                program::DataType::String,
                program::DataType::u32,
                program::DataType::u8,
                program::DataType::Bool,
                program::DataType::String,
                program::DataType::ImageUrl,
                program::DataType::ImageUrl,
                program::DataType::ImageUrl,
                program::DataType::ImageUrl,
                program::DataType::ImageUrl,
                program::DataType::String,
                program::DataType::String,
                program::DataType::String,
                program::DataType::String,
                program::DataType::String,
            ];

            let args = program::MintCreationArgs {
                name: mint_names
                    .get(mint_index)
                    .unwrap_or(&format!("Mint {}", mint_index + 1).as_str())
                    .to_string(),
                image: ImageUrl::new(mint_images.get(mint_seq).unwrap()),
                data_types: Some(data_types),
                names: Some(vec![
                    "Name".to_string(),
                    "Weight".to_string(),
                    "Score".to_string(),
                    "Testing".to_string(),
                    "Stringvalue".to_string(),
                    "Image".to_string(),
                    "Image".to_string(),
                    "Image".to_string(),
                    "Image".to_string(),
                    "Image".to_string(),
                    "Stringvalue".to_string(),
                    "Stringvalue".to_string(),
                    "Stringvalue".to_string(),
                    "Stringvalue".to_string(),
                    "Stringvalue".to_string(),
                ]),
                descriptions: Some(vec![
                    "Token name".to_string(),
                    "Any number".to_string(),
                    "Score".to_string(),
                    "Are you testing?".to_string(),
                    "Bool testing".to_string(),
                    "Use any url shortening service".to_string(),
                    "Use any url shortening service".to_string(),
                    "Use any url shortening service".to_string(),
                    "Use any url shortening service".to_string(),
                    "Use any url shortening service".to_string(),
                    "String field".to_string(),
                    "String field".to_string(),
                    "String field".to_string(),
                    "String field".to_string(),
                    "String field".to_string(),
                ]),
            };

            mint_index += 1;

            let tx = Mint::create(&authority, &args).await?;
            let mint_account_pubkey = tx.target_account()?;
            tx.execute().await?;
            let mint_container = load_container::<program::Mint>(&mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");
            assert_eq!(&mint_account_pubkey, mint_container.pubkey());
            mint_pubkeys.push(mint_account_pubkey);
            log_info!("mint {mint_seq} creation ok - {}", mint_container.pubkey());
        }

        let mut img_index = 0;
        for mint_seq in 0..MAX_MINTS {
            for token_seq in 0..MAX_TOKENS {
                log_info!("creating token {mint_seq}:{token_seq}");

                let mint_account_pubkey = mint_pubkeys.get(mint_seq).unwrap();
                let mint_container = reload_container::<program::Mint>(mint_account_pubkey)
                    .await?
                    .expect("¯\\_(ツ)_/¯");

                let sol = (token_seq as f64) + 1.0 / 100.0;
                let args = program::TokenCreateFinalArgs {
                    for_sale: program::ForSale::Yes,
                    exchange_mechanics: ExchangeMechanics::sale(sol_to_lamports(sol), None),
                    sale_type: SaleType::Sale,
                    data: vec![
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::u32((token_seq * 15) as u32),
                        program::Data::u8((token_seq + 1) as u8),
                        program::Data::Bool(false),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                        program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                        program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                        program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                        program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                        program::Data::String(names.get(img_index).unwrap().to_string()),
                    ],
                };
                img_index += 1;
                if img_index == images.len() {
                    img_index = 0;
                }
                let tx = Token::create(&authority, mint_container.pubkey(), &args).await?;
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

        let root = reload_container::<program::Root>(&Root::pubkey())
            .await?
            .expect("¯\\_(ツ)_/¯");

        let max_mints = root.mints.len();
        for mint_seq in 0..max_mints {
            let mint_account_pubkey = root.mints.get_pubkey_at(&program_id(), mint_seq as u64)?;
            let mint_container = reload_container::<program::Mint>(&mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");

            let token_len = mint_container.tokens.len();
            log_info!("mint {mint_seq} {mint_account_pubkey} [{token_len}]");

            for token_seq in 0..token_len {
                let token_account_pubkey = mint_container
                    .tokens
                    .get_pubkey_at(&program_id(), token_seq as u64)?;

                let _token = reload_container::<program::Token>(&token_account_pubkey)
                    .await?
                    .expect("¯\\_(ツ)_/¯");

                log_info!("\ttoken {token_seq} {token_account_pubkey}");
            }
        }

        Ok(())
    }
}
