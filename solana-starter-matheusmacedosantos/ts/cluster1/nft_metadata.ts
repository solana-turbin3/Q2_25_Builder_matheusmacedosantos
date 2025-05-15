import wallet from "../../ts/cluster1/wallet/id.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));



(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

      
        const imageUri = "https://devnet.irys.xyz/9WxECWufvYkwEGr8g3i6YcuzfKiGLjx6ym93tQF5oTXk";
        const metadata = {
            name: "JeffWifHat",
            symbol: "JWH",
            description: "Turbin3 NFT",
            image: imageUri,
            attributes: [
                {trait_type: 'Jeff', value: '01'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageUri,
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
