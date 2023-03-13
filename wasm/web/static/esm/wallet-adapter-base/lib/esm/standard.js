import { SolanaSignAndSendTransaction, SolanaSignTransaction, } from '/static/esm/wallet-standard-features/lib/esm/index.js';
import { StandardConnect, StandardEvents, } from '/static/esm/@wallet-standard/features/lib/esm/index.js';
export function isWalletAdapterCompatibleStandardWallet(wallet) {
    return (StandardConnect in wallet.features &&
        StandardEvents in wallet.features &&
        (SolanaSignAndSendTransaction in wallet.features || SolanaSignTransaction in wallet.features));
}
//# sourceMappingURL=standard.js.map