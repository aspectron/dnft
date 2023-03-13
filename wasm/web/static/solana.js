//export * from "/static/esm/web3.js/index.iife.js";

import {
    WalletAdapterNetwork, BaseMessageSignerWalletAdapter, WalletReadyState
} from '/static/esm/wallet-adapter-base/lib/esm/index.js';


import {PhantomWalletAdapter} from '/static/esm/wallet-adapter-phantom/lib/esm/index.js';

export {
    BaseMessageSignerWalletAdapter,
    WalletReadyState,
    WalletAdapterNetwork,
    PhantomWalletAdapter
}
