extern crate alloc;

pub mod category;
pub mod exchange;
pub mod prelude;

#[cfg(not(target_os = "solana"))]
pub mod client;
pub mod error;
pub mod program;

use crate::prelude::*;

pub mod dnft_program {
    use kaizen::container::Utf8String;

    use super::*;

    pub struct DnftHandler;

    impl DnftHandler {
        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello handler test");
            Ok(())
        }
    }

    declare_handlers!(DnftHandler, [DnftHandler::test,]);

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    pub enum ContainerTypes {
        DnftContainer = 1,
    }

    // data passed to the create() function
    #[derive(Clone, BorshSerialize, BorshDeserialize)]
    pub struct CreationData {
        pub msg: String,
        pub data: RecordArgs,
    }

    #[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize)]
    pub struct RecordArgs {
        pub int64: u64,
        pub int32: u32,
        pub pubkey: Pubkey,
        pub int8: u8,
    }

    #[derive(Meta, Clone, Copy)]
    #[repr(packed)]
    pub struct RecordData {
        pub int8: u8,
        pub int32: u32,
        // pub int64: u64,
        pub pubkey: Pubkey,
    }

    impl From<RecordArgs> for RecordData {
        fn from(args: RecordArgs) -> Self {
            log_trace!("{:?}", args);
            RecordData {
                int8: args.int8,
                int32: args.int32,
                // int64: args.int64,
                pubkey: args.pubkey,
            }
        }
    }

    #[container(ContainerTypes::DnftContainer)]
    pub struct DnftContainer<'info, 'refs> {
        pub message: Utf8String<'info, 'refs>,
        pub records: Array<RecordData, 'info, 'refs>,
    }

    impl<'info, 'refs> DnftContainer<'info, 'refs> {
        pub fn test(_ctx: &ContextReference) -> ProgramResult {
            log_trace!("hello container test!");
            Ok(())
        }

        pub fn create(ctx: &ContextReference) -> ProgramResult {
            let args = CreationData::try_from_slice(ctx.instruction_data)?;
            let allocation_args = AccountAllocationArgs::default();
            // pre-calculate additional data needed for the account to avoid realloc()
            // of the account during the record insert operation
            let extra_data = std::mem::size_of::<RecordData>() + args.msg.as_bytes().len();

            let container = DnftContainer::try_allocate(ctx, &allocation_args, extra_data)?;

            unsafe {
                let record_data_dst = container.records.try_allocate(false)?;
                *record_data_dst = args.data.into();

                container.message.store(&args.msg)?;
                ctx.sync_rent(container.account(), &RentCollector::default())?;
            }

            Ok(())
        }
    }

    declare_handlers!(
        DnftContainer::<'info, 'refs>,
        [DnftContainer::test, DnftContainer::create,]
    );
}

#[cfg(not(target_os = "solana"))]
pub mod dnft_client {
    use super::*;
    use kaizen::{result::Result, utils};
    use std::str::FromStr;
    pub struct DnftHandlerClient;
    use rand;
    declare_client!(dnft_program::DnftHandler, DnftHandlerClient);

    impl DnftHandlerClient {
        pub async fn run_test(authority: &Pubkey) -> Result<TransactionList> {
            let builder = DnftHandlerClient::execution_context_for(dnft_program::DnftHandler::test)
                .with_authority(authority)
                .seal()?;

            let transaction = Transaction::new_without_accounts(
                "Container test", //.to_string(),
                builder.try_into()?,
            );

            Ok(TransactionList::new(vec![transaction]))
        }
    }

    pub struct DnftContainerClient;
    declare_client!(dnft_program::DnftContainer, DnftContainerClient);

    impl DnftContainerClient {
        pub async fn create(
            authority: &Pubkey,
            data: &dnft_program::CreationData,
        ) -> Result<TransactionList> {
            let random_seed = rand::random::<[u8; 8]>();
            let builder = Self::execution_context_for(dnft_program::DnftContainer::create)
                .with_authority(authority)
                .with_account_templates_with_custom_suffixes(&[&random_seed])
                // .with_account_templates(1)
                .with_instruction_data(&data.try_to_vec()?)
                .seal()?;

            let accounts = builder.gather_accounts(Some(Gather::Authority), None)?;
            let transaction = Transaction::new_with_accounts(
                format!("Creating example account {}", accounts[0]).as_str(),
                accounts,
                builder.try_into()?,
            );

            Ok(TransactionList::new(vec![transaction]))
        }
    }

    #[wasm_bindgen]
    pub async fn run_test() -> Result<()> {
        let transport = Transport::global()?;
        if let Some(emulator) = transport.emulator() {
            let authority = Pubkey::from_str("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f")?;
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

        let tx = dnft_client::DnftHandlerClient::run_test(&authority).await?;
        tx.execute().await?;

        let pubkey = Pubkey::from_str("9ZNTfG4NyQgxy2SWjSiQoUyBPEvXT2xo7fKc5hPYYJ7b")?;
        let data = dnft_program::CreationData {
            msg: "hello container".to_string(),
            data: dnft_program::RecordArgs {
                int8: 1,
                int32: 2,
                int64: 3,
                pubkey,
            },
        };
        let tx = dnft_client::DnftContainerClient::create(&authority, &data).await?;
        let target_account_pubkey = tx.target_account()?;

        tx.execute().await?;

        let container = load_container::<dnft_program::DnftContainer>(&target_account_pubkey)
            .await?
            .expect("¯\\_(ツ)_/¯");

        let message = container.message.to_string();
        let record = container.records.try_get_at(0)?;
        let int8 = record.get_int8();
        let int32 = record.get_int32();
        // let int64 = record.get_int64();
        let incoming_pubkey = record.get_pubkey();

        log_trace!("container data - message: {message} int8: {int8} int32: {int32} pubkey: {incoming_pubkey}");

        assert_eq!(int8, 1);
        assert_eq!(int32, 2);
        // assert_eq!(int64, 3);
        assert_eq!(pubkey, incoming_pubkey);

        Ok(())
    }

    #[wasm_bindgen]
    pub async fn init_transport_for_unit_tests() -> kaizen::result::Result<()> {
        println!("init transport...");
        Transport::try_new_for_unit_tests(
            crate::program_id(),
            Some(Pubkey::from_str(
                "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f",
            )?),
            TransportConfig::default(),
        )
        .await?;
        Ok(())
    }

    #[wasm_bindgen]
    pub fn dnft_program_id() -> Pubkey {
        program_id()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use kaizen::result::Result;
    use std::str::FromStr;

    #[async_std::test]
    async fn example_test() -> Result<()> {
        kaizen::init()?;

        println!("init transport...");
        Transport::try_new_for_unit_tests(
            crate::program_id(),
            Some(Pubkey::from_str(
                "42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f",
            )?),
            TransportConfig::default(),
        )
        .await?;
        println!("run test...");

        dnft_client::run_test().await?;

        Ok(())
    }
}

declare_program!(
    "dnft",
    "5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH",
    [dnft_program::DnftHandler, dnft_program::DnftContainer,]
);
