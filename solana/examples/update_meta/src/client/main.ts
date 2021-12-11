import { establishConnection, establishPayer, checkProgram, update_data, check_data } from "./update-metadata";

async function main() {
    console.log("lets say ello to solana account");

    await establishConnection();

    await establishPayer();

    await checkProgram();

    await update_data();

    await check_data();

    console.log("success");
}

main().then(
    () => process.exit(),
    err => {
        console.log(err);
        process.exit(-1);
    }
);