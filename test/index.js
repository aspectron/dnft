const {createMint, getMint, mintTo, 
    AccountLayout, TOKEN_PROGRAM_ID, 
    getAccount, getOrCreateAssociatedTokenAccount,
    transfer,
} = require("@solana/spl-token");
const { 
    clusterApiUrl, Connection, Keypair, 
    LAMPORTS_PER_SOL, PublicKey ,
    
} = require("@solana/web3.js");

const base58 = require("bs58");

let program_id = new PublicKey("5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH");

let key_data = Uint8Array.from([199,8,110,17,134,223,143,213,213,118,61,54,185,122,37,153,126,183,243,252,165,177,1,156,58,179,30,234,5,103,1,166,137,204,62,6,108,70,181,61,102,137,201,245,209,108,20,107,218,144,142,9,237,45,167,71,176,4,165,192,75,135,58,183]);
const payer = Keypair.fromSecretKey(key_data);
const connection = new solanaWeb3.Connection(
    "http://localhost:8899",
    'confirmed'
);

async function testWallet(){
    const connection = new solanaWeb3.Connection(
        "http://localhost:8899",
        'confirmed'
    );

    let blockhash = (await connection.getLatestBlockhash()).blockhash;

    // let tx = new solanaWeb3.Transaction({
    //     "recentBlockhash": blockhash,//"CScCTs3vxCTGioKssbQc3X4pHNFhwASxB6711ABN1gja",
    //     "feePayer": new solanaWeb3.PublicKey("J92gL9eTqSLMGZQzr2yw2Jh2Wbsk1UEtJEnsMNY2HS9D"),
    //     "nonceInfo": null,
    // });
    let feePayer = new solanaWeb3.PublicKey("J92gL9eTqSLMGZQzr2yw2Jh2Wbsk1UEtJEnsMNY2HS9D");
    let tx = new solanaWeb3.Transaction();
    tx.recentBlockhash = blockhash;
    tx.feePayer = feePayer;
    tx.add({
        "keys":[{
            "pubkey":new solanaWeb3.PublicKey("J92gL9eTqSLMGZQzr2yw2Jh2Wbsk1UEtJEnsMNY2HS9D"),
            "isSigner":true,
            "isWritable":true
        },{
            "pubkey":new solanaWeb3.PublicKey("YA7NvczboDEtoBUUqFQzhX1NLtDf6qKEYQFiLqrNubm"),
            "isSigner":false,
            "isWritable":true
        }],
        "programId":new solanaWeb3.PublicKey("5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH"),
        //"data": solanaWeb3.Buffer.from([1,0,0,0,0,0,0,0,0,17,0,17,0,2,0,1,0,0,0,1,1,0,0,184,100,217,69,0,0,0])
        "data": Uint8Array.from([1,0,0,0,0,0,0,0,0,17,0,17,0,2,0,1,0,0,0,1,1,0,0,184,100,217,69,0,0,0])
    });

    let a = await solana.signAndSendTransaction(tx);
    console.log("sssss", a);

}


async function check_mint(){
    

    let mint = new PublicKey("EWeCSgW6pmu5u6fnwoofC7CijnaULaqYvKDqgXTMG98H");
    

    console.log(mint.toString());
    // AQoKYV7tYpTrFZN6P5oUufbQKAUr9mNYGe1TTJC9wajM

    const mintInfo = await getMint(
        connection,
        mint,
        undefined,
        program_id
    )
      
    console.log(mintInfo.supply);


}

async function run(){
    //console.log("payer: key", payer.secretKey.toString());
    console.log("payer: publicKey", payer.publicKey.toString());
    const mintAuthority = payer;
    //const freezeAuthority = Keypair.generate();


    const connection = new Connection(
        "http://localhost:8899",
        'confirmed'
    );

    const airdropSignature = await connection.requestAirdrop(
        payer.publicKey,
        LAMPORTS_PER_SOL,
    );

    let mint = await createMint(
        connection,
        payer,
        payer.publicKey,
        payer.publicKey,
        0 // We are using 9 to match the CLI decimal default exactly
    );

    let mint_str = mint.toString();
    console.log("mint", mint_str)

    //mint = new PublicKey(mint_str);
    //mint = new PublicKey("EWeCSgW6pmu5u6fnwoofC7CijnaULaqYvKDqgXTMG98H");

    //let program_id = new PublicKey("5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH");

    const tokenAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        payer,
        mint,
        payer.publicKey,
        //false,
        //undefined,
        //undefined,
        //program_id,
        //program_id
    )
      
    console.log("tokenAccount", tokenAccount.address.toBase58());
    

    const tokenAccountInfo = await getAccount(
        connection,
        tokenAccount.address
      )
      
    console.log("tokenAccountInfo", tokenAccountInfo.amount);

    await mintTo(
        connection,
        payer,
        mint,
        tokenAccount.address,
        mintAuthority,
        1 // because decimals for the mint are set to 9 
    )

    
    const mintInfo = await getMint(
        connection,
        mint
    )
      
    console.log(mintInfo.supply);

    let check_accounts = async (pubkey)=>{
        const tokenAccounts = await connection.getTokenAccountsByOwner(
            pubkey,
            {
            programId: TOKEN_PROGRAM_ID,
            }
        );
        
        console.log("Token                                         Balance");
        console.log("------------------------------------------------------------");
        tokenAccounts.value.forEach((tokenAccount) => {
            const accountData = AccountLayout.decode(tokenAccount.account.data);
            console.log(`${new PublicKey(accountData.mint)}   ${accountData.amount}`);
        })
    }

    await check_accounts(payer.publicKey);

    let toWalletPublicKey = new PublicKey("J92gL9eTqSLMGZQzr2yw2Jh2Wbsk1UEtJEnsMNY2HS9D");

      // Get the token account of the toWallet address, and if it does not exist, create it
    const toTokenAccount = await getOrCreateAssociatedTokenAccount(
        connection, 
        payer, 
        mint, 
        toWalletPublicKey
    );

    
      // Transfer the new token to the "toTokenAccount" we just created
    let signature = await transfer(
        connection,
        payer,
        tokenAccount.address,
        toTokenAccount.address,
        payer.publicKey,
        1
    );

    console.log("signature", signature);

    await check_accounts(payer.publicKey);
    await check_accounts(toWalletPublicKey);
}

async function fetch_accounts(){

    let mint = new PublicKey("6QJ6DFusKuQMdZLrQixdGFmC8WLnzMSuq9KJPpavbhrS");
    let bytes = Buffer.from(mint.toBytes()).toString("hex")
    console.log("mint", bytes);
    let account = await connection.getAccountInfo(mint);
    console.log("mint account", account.data.toString("hex"))
    let config = {
        filters: [
            {
                memcmp: {
                    offset: 8, // number of bytes
                    bytes: mint.toString(), // base58 encoded string
                },
            },
            {
                memcmp: {
                    offset: 40, // number of bytes
                    bytes: base58.encode([0]), // base58 encoded string
                },
            },
        ],
    };

    console.log("config:", JSON.stringify(config, null, "\t"));
    const accounts = await connection.getProgramAccounts(
        program_id,
        config
    );

  console.log(
    `Found ${accounts.length} token account(s) for Mint ${mint}: `
  );
  accounts.forEach(({account, pubkey}, i) => {
    console.log("account.data", pubkey.toString(), account.data.toString("hex"))
    /*
    console.log(
      `-- Token Account Address ${i + 1}: ${account.pubkey.toString()} --`
    );
    console.log(`Mint: ${account.account.data["parsed"]["info"]["mint"]}`);
    console.log(
      `Amount: ${account.account.data["parsed"]["info"]["tokenAmount"]["uiAmount"]}`
    );
    */
  });
}

//run();

fetch_accounts();

