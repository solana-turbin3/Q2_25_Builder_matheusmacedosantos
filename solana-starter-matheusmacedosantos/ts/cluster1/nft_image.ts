import wallet from "../../ts/cluster1/wallet/id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"
import path from "path";



// Create a devnet connection
const umi = createUmi('https://turbine-solanad-4cde.devnet.rpcpool.com/168dd64f-ce5e-4e19-a836-f6482ad6b396');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader({ address: "https://devnet.irys.xyz/" })); // updated
umi.use(signerIdentity(signer));
const fileName = "rug.png";

(async () => {
    try {
        // 1. Load image
        // const file = await readFile("./" + fileName);
        const file = await readFile(path.join(__dirname, fileName));
        console.log("File loaded: ", file);

        // 2. Convert image to generic file.
        const genericFile = createGenericFile(file, fileName, { contentType: "image/png" });

        // 3. Upload image
        const [myUri] = await umi.uploader.upload([genericFile]);
        const irysURI = myUri.replace(
            "https://arweave.net/",
            "https://devnet.irys.xyz/"
        );
        umi.use(irysUploader({ address: "https://devnet.irys.xyz/" }));

        // const image = ???

        // const [myUri] = ??? 
        console.log("Your image URI: ", myUri);
    } catch (error) {
        console.log("Oops.. Something went wrong", error);
    }
})();