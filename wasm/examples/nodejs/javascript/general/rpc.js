// @ts-ignore
globalThis.WebSocket = require('websocket').w3cwebsocket; // W3C WebSocket module shim

const entropyx = require('../../../../nodejs/entropyx');
const { parseArgs } = require("../utils");
const {
    RpcClient,
    Resolver,
} = entropyx;

entropyx.initConsolePanicHook();

const {
    networkId,
    encoding,
} = parseArgs();

(async () => {

    const rpc = new RpcClient({
        // url : "0.0.0.0",
        // encoding,
        resolver: new Resolver(),
        networkId
    });
    console.log(`Resolving RPC endpoint...`);
    await rpc.connect();
    console.log(`Connecting to ${rpc.url}`)

    const info = await rpc.getBlockDagInfo();
    console.log("GetBlockDagInfo response:", info);

    await rpc.disconnect();
    console.log("bye!");
})();
