use dnft::{
    client::{Mint, Root, Token},
    program,
    program::{ExchangeMechanics, ImageUrl, SaleType},
    program_id,
};
use kaizen::{prelude::*, result::Result, utils::sol_to_lamports};
use std::str::FromStr;

pub async fn async_main(with_sample_data: bool) -> Result<()> {
    kaizen::init()?;

    const USE_EMULATOR: bool = false;
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

async fn create_root() -> Result<()> {
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

async fn create_sample_data() -> Result<()> {
    log_info!("creating sample data");

    let transport = Transport::global()?;
    let authority = transport.get_authority_pubkey()?;
    let mint_images = vec![
        "/file/mint/pexels-abhishek-rajesh-669030.jpg",
        "/file/mint/pexels-karolina-grabowska-4040655.jpg",
        "/file/mint/pexels-angela-roma-7319331.jpg",
        "/file/mint/pexels-mike-b-109548.jpg",
        "/file/mint/pexels-chevanon-photography-1108099.jpg",
        "/file/mint/pexels-petr-ganaj-4055736.jpg",
        "/file/mint/pexels-denniz-futalan-2523934.jpg",
        "/file/mint/pexels-pok-rie-239659.jpg",
        "/file/mint/pexels-karolina-grabowska-4040649.jpg",
        "/file/mint/pexels-ravi-kant-5161266.jpg",
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
    const MAX_MINTS: usize = 2;
    const MAX_TOKENS: usize = 5;
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
            program::DataType::u64,
            program::DataType::ImageUrl,
        ];

        let args = program::MintCreationArgs {
            name: mint_names
                .get(mint_index)
                .unwrap_or(&format!("Mint {}", mint_index+1).as_str())
                .to_string(),
            image: ImageUrl::new(mint_images.get(mint_seq).unwrap()),
            data_types: Some(data_types),
            names: Some(vec![
                "Name".to_string(),
                "Weight".to_string(),
                "Score".to_string(),
                "Index".to_string(),
                "Image".to_string(),
            ]),
            descriptions: Some(vec![
                "Token name".to_string(),
                "Any number".to_string(),
                "Score".to_string(),
                "".to_string(),
                "Use any url shortening service".to_string(),
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

    //let mint_pubkeys = vec![Pubkey::from_str("8bmnuP1HuDMmM2Yz8gZ5KLRJA8pYXboFVd3uZtLnF3nx").unwrap()];

    // ----------------------------------------------------------------------------
    // let sale_types = [
    //     SaleType::Sale,
    //     SaleType::Auction,
    //     SaleType::Barter,
    //     SaleType::None,
    //     SaleType::Raffle,
    //     SaleType::Rent,
    // ];
    let mut img_index = 0;
    for mint_seq in 0..MAX_MINTS {
        //let mut sale_type_index = 0;
        for token_seq in 0..MAX_TOKENS {
            log_info!("creating token {mint_seq}:{token_seq}");

            let mint_account_pubkey = mint_pubkeys.get(mint_seq).unwrap();
            let mint_container = reload_container::<program::Mint>(mint_account_pubkey)
                .await?
                .expect("¯\\_(ツ)_/¯");

            //let sale_type = sale_types[sale_type_index];
            // sale_type_index += 1;
            // if sale_type_index == sale_types.len() {
            //     sale_type_index = 0;
            // }
            let sol = (token_seq as f64) + 1.0 / 100.0;
            let args = program::TokenCreateFinalArgs {
                for_sale: program::ForSale::Yes,
                exchange_mechanics: ExchangeMechanics::sale(sol_to_lamports(sol), None),
                sale_type: SaleType::Sale,
                data: vec![
                    program::Data::String(names.get(img_index).unwrap().to_string()),
                    program::Data::u32((token_seq * 15) as u32),
                    program::Data::u8((token_seq + 1) as u8),
                    program::Data::u64((token_seq * 11) as u64),
                    program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
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

        // let schema = mint_container.data_types.load()?;
        // log_info!("\n\nmint container schema: {:#?}\n", schema);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    log_info!("DNFT initializing.....");
    async_main(true)
        .await
        .map_err(|err| {
            log_error!("DNFT initialize: Error: {}", err);
            println!();
        })
        .ok();
}
