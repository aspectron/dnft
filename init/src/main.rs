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
    let use_devnet = false;
    const AUTHORITY: &str = "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f";

    println!("init transport...");
    let transport = if USE_EMULATOR {
        Transport::try_new_for_unit_tests(
            program_id(),
            Some(Pubkey::from_str(AUTHORITY)?),
            TransportConfig::default(),
        )
        .await?
    } else if use_devnet{
        Transport::try_new("https://api.devnet.solana.com", TransportConfig::default()).await?
    }else{
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
        "https://images.pexels.com/photos/1108099/pexels-photo-1108099.jpeg?auto=compress&cs=tinysrgb&w=1000&h=400&dpr=2",
        "https://images.pexels.com/photos/45170/kittens-cat-cat-puppy-rush-45170.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        "https://images.pexels.com/photos/5011647/pexels-photo-5011647.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        "https://images.pexels.com/photos/9436715/pexels-photo-9436715.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        "https://images.pexels.com/photos/4818709/pexels-photo-4818709.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        "https://images.pexels.com/photos/1475938/pexels-photo-1475938.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
        "https://images.pexels.com/photos/2640604/pexels-photo-2640604.jpeg?auto=compress&cs=tinysrgb&w=1000&h=350&dpr=2",
    ];

    let images = vec![
        "https://tinyurl.com/2p9df348",
        "https://cdn.pixabay.com/photo/2019/07/30/05/53/dog-4372036_1280.jpg",
        "https://cdn.pixabay.com/photo/2019/12/03/22/22/dog-4671215_1280.jpg",
        "https://cdn.pixabay.com/photo/2017/08/18/16/14/dog-2655472_1280.jpg",
        "https://cdn.pixabay.com/photo/2019/05/23/19/29/dog-4224638_1280.jpg",

        "https://cdn.pixabay.com/photo/2020/04/08/08/33/cat-5016374_1280.jpg",
        "https://cdn.pixabay.com/photo/2022/02/18/14/27/cat-7020822_1280.jpg",
        "https://cdn.pixabay.com/photo/2021/11/21/22/17/british-shorthair-6815384_1280.jpg",
        "https://cdn.pixabay.com/photo/2022/04/19/18/14/cat-7143536_1280.jpg",
        "https://cdn.pixabay.com/photo/2021/03/13/13/42/cat-6091733_1280.jpg",
    ];
    let mint_names = vec!["Dog Tokens", "Cat Tokens"];
    let names = vec![
        "Dog Token - A",
        "Dog Token - B",
        "Dog Token - C",
        "Dog Token - D",
        "Dog Token - E",
        "Cat Token - A",
        "Cat Token - B",
        "Cat Token - C",
        "Cat Token - D",
        "Cat Token - E",
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
            program::DataType::ImageUrl,
            program::DataType::u32,
            program::DataType::u64,
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
                "Image".to_string(),
                "Weight".to_string(),
                "Score".to_string(),
            ]),
            descriptions: Some(vec![
                "Token name".to_string(),
                "Use any url shortening service".to_string(),
                "Any number".to_string(),
                "Score".to_string(),
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
                    program::Data::Url(program::Url::image(images.get(img_index).unwrap())),
                    program::Data::u32((token_seq * 15) as u32),
                    program::Data::u64((token_seq + 1) as u64),
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
