/*
const exportSolana = ()=>{
    let keys = Object.keys(window.solanaWeb3);
    //console.log("keys", keys)
    let items = [];
    //keys.forEach(key=>{
        //items.push(`export const ${key} = solanaWeb3.${key};`);
    //});

    //console.log(items.join("\n"))
    //console.log("solanaWeb3:", solanaWeb3)
    items.push("export const {");
    items.push("\t"+keys.join(",\n\t"));
    items.push("} = solanaWeb3");

    console.log(items.join("\n")+";\n")
}
_exportSolana();
*/

export const {
	TransactionStatus,
	Account,
	AddressLookupTableInstruction,
	AddressLookupTableProgram,
	Authorized,
	BLOCKHASH_CACHE_TIMEOUT_MS,
	BPF_LOADER_DEPRECATED_PROGRAM_ID,
	BPF_LOADER_PROGRAM_ID,
	BpfLoader,
	COMPUTE_BUDGET_INSTRUCTION_LAYOUTS,
	ComputeBudgetInstruction,
	ComputeBudgetProgram,
	Connection,
	Ed25519Program,
	Enum,
	EpochSchedule,
	FeeCalculatorLayout,
	Keypair,
	LAMPORTS_PER_SOL,
	LOOKUP_TABLE_INSTRUCTION_LAYOUTS,
	Loader,
	Lockup,
	MAX_SEED_LENGTH,
	Message,
	NONCE_ACCOUNT_LENGTH,
	NonceAccount,
	PACKET_DATA_SIZE,
	PublicKey,
	SIGNATURE_LENGTH_IN_BYTES,
	SOLANA_SCHEMA,
	STAKE_CONFIG_ID,
	STAKE_INSTRUCTION_LAYOUTS,
	SYSTEM_INSTRUCTION_LAYOUTS,
	SYSVAR_CLOCK_PUBKEY,
	SYSVAR_EPOCH_SCHEDULE_PUBKEY,
	SYSVAR_INSTRUCTIONS_PUBKEY,
	SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
	SYSVAR_RENT_PUBKEY,
	SYSVAR_REWARDS_PUBKEY,
	SYSVAR_SLOT_HASHES_PUBKEY,
	SYSVAR_SLOT_HISTORY_PUBKEY,
	SYSVAR_STAKE_HISTORY_PUBKEY,
	Secp256k1Program,
	SendTransactionError,
	SolanaJSONRPCError,
	SolanaJSONRPCErrorCode,
	StakeAuthorizationLayout,
	StakeInstruction,
	StakeProgram,
	Struct,
	SystemInstruction,
	SystemProgram,
	Transaction,
	TransactionExpiredBlockheightExceededError,
	TransactionExpiredTimeoutError,
	TransactionInstruction,
	VALIDATOR_INFO_KEY,
	VOTE_PROGRAM_ID,
	ValidatorInfo,
	VoteAccount,
	VoteAuthorizationLayout,
	VoteInit,
	VoteInstruction,
	VoteProgram,
	clusterApiUrl,
	sendAndConfirmRawTransaction,
	sendAndConfirmTransaction
} = solanaWeb3;


