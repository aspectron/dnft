globalThis.WebSocket = require('websocket').w3cwebsocket;

let dnft = require("./dnft");

dnft.init_console_panic_hook();

console.log("dnft program id:", dnft.dnft_program_id().toString());