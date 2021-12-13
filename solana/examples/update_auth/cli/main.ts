import web3 = require('@solana/web3.js');
import { sign } from 'crypto';
import { update } from './update-metadata-test';


const connection = new web3.Connection(web3.clusterApiUrl('devnet'));

let programId = new web3.PublicKey(
    "24uD7rNTrzraSnKJvkwmDcXVKRxhv9DdYijBfbVXNSE7"
);

async function  main() {
    console.log("Run test: update metadata");
    await update();
}

main()
	.then(() => process.exit(0))
	.catch((error) => {
		console.error(error);
		process.exit(1);
});