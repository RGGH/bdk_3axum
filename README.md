# bdk_3axum

Attempt to combine BDK & Axum (learning exercise)
uses : Bitcoin Testnet + Electrum

- Create Bitcoin Wallet and save xrpv to json (totally insecure, but this is for learning)
- Load wallet from xprv
- Create PSBT + Sign + Broadcast

  Note : tb1 = testnet bech32 addresses / corresponding prefix on mainnet would be bc1

#### https://redandgreen.co.uk/bitcoin-testnet-test-faucet/bitcoin-programming/

#### endpoints
    /
    api/gen_wallet
    api/load_wallet

![alt](t-rec.gif)
