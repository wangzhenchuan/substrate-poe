import { WsProvider, ApiPromise, Keyring } from "@polkadot/api";
const { u8aToString } = require("@polkadot/util");

import '@polkadot/api-augment'
import '@polkadot/types'

const sleep = (ms: number) => new Promise(r => setTimeout(r, ms));

const WEB_SOCKET = "ws://127.0.0.1:9944";
const connect = async () => {
    const wsProvider = new WsProvider(WEB_SOCKET);
    const api = await ApiPromise.create({ provider: wsProvider, types: {} });
    await api.isReady;
    return api;
}

const main = async () => {
    const api = await connect();

    let value = await api.rpc.offchain.localStorageGet('PERSISTENT', "testkey");
    const hexValue = value.toHex();
    const u8aValue = new Uint8Array(
      (hexValue.match(/.{1,2}/g) || []).map((byte) => parseInt(byte, 16))
    );
    const stringValue = u8aToString(u8aValue);
    console.log("value in offchain storage: ", stringValue);
}

main().then(function () {
    console.log("sucess")
})