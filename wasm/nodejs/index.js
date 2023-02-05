globalThis.WebSocket = require('websocket').w3cwebsocket;

let dnft = require("./dnft");

dnft.init_console_panic_hook();

console.log("dnft program id:", dnft.dnft_program_id().toString());

(async () => {
    dnft.init_kaizen(dnft,{},{});
    let program_id = dnft.dnft_program_id();
    let authority = new dnft.Pubkey("42bML5qB3WkMwfa2cosypjUrN7F2PLQm4qhxBdRDyW7f");
    console.log("program_id: ",program_id, "authority: ",authority);
    let transport = await dnft.Transport.InProcUnitTests(program_id,authority);
    console.log("init kaizen done");
    dnft.run_test();
})();
