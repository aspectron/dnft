globalThis.WebSocket = require('websocket').w3cwebsocket;

let dnft = require("./dnft");

dnft.init_console_panic_hook();

console.log("dnft program id:", dnft.dnft_program_id().toString());

(async () => {
    dnft.init_kaizen(dnft,{},{});
    // let program_id = dnft.dnft_program_id();
    // let authority = new dnft.Pubkey("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f");
    // console.log("program_id: ",program_id, "authority: ",authority);
    // let transport = await dnft.Transport.InProcUnitTests(program_id,authority);
    // console.log("init kaizen done");
    // dnft.run_test();

    console.log("building schema...");
    const { Field, DataType, Data } = dnft;

    let vec = [];
    let dataTypes = Object.keys(DataType).filter(k => !isFinite(+k));
    for (let dataType of dataTypes) {
        let name = ("field-"+dataType).toLowerCase();
        let descr = `Descr for ${dataType}`;

        vec.push(new Field(DataType[dataType], name, descr));
    }

    let schema = new dnft.Schema(vec);
    schema.display();

    // let v0 = new Data(DataType.Pubkey, 123);
    let v1 = new Data(DataType.i32, -500);
    console.log(v1+"");
    
    let pk = new dnft.Pubkey("5UAQGzYRWKEgdbpZCqoUjKDKiWpNbHeataWknRpvswEH");
    console.log("pk js: ",pk);
    let v2 = new Data(DataType.Pubkey, pk);
    console.log(pk instanceof dnft.Pubkey);
    console.log(pk.constructor);
    console.log(pk.constructor.name);

    let data = new Data(DataType.Array,[
        new Data(DataType.Pubkey, pk),
        new Data(DataType.u8, 1),
        new Data(DataType.u16, 2),
        new Data(DataType.u32, 3),
        new Data(DataType.u64, 4),
        new Data(DataType.i8, 5),
        new Data(DataType.i16, 6),
        new Data(DataType.i32, 7),
        new Data(DataType.i64, 8),
    ]);

    console.log(data.toString());
    // console.log(pk.__proto__);
    // console.log(JSON.stringify(pk, null, 4));
})();
