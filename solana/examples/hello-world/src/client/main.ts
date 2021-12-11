import { establishConnection, establishPayer, checkProgram, sayHello, reportGreetings } from "./hello-world";

async function hello() {
    console.log("lets say ello to solana account");

    await establishConnection();

    await establishPayer();

    await checkProgram();

    await sayHello();

    await reportGreetings();

    console.log("success");
}

hello().then(
    () => process.exit(),
    err => {
        console.log(err);
        process.exit(-1);
    }
);