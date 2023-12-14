import * as anchor from "@coral-xyz/anchor";
import web3, {Connection} from "@solana/web3.js";
import solanaWeb3, {Keypair, sendAndConfirmTransaction, clusterApiUrl} from "@solana/web3.js";
import {updateAccount, UpdateAccountAccounts, UpdateAccountArgs} from "../app/generated/instructions";
import {ExampleAccount} from "../app/generated/accounts";
import express, {Request, Response} from 'express';
import bodyParser from 'body-parser';

const app = express();
const port = 3000;

//We are expecting the body to contain valid JSON from the Helius request
app.use(bodyParser.json());

//This is our actual webhook that is served locally at http://localhost:3000/webhook.
//By using ngrok, the ngrok provided public url will map to our local webhook
//https://3214-199-223-251-92.ngrok-free.app/webhook -> http://localhost:3000/webhook
app.post('/webhook', (request: Request, response: Response) => {

    const requestBody = request.body;
    //Print the JSON to the console
    console.log('Data received by webook: ', requestBody);
    publish_event();

    //Send a response that we received and processed the request.
    response.status(200).send('Webhook Request Received!');
});

app.listen(port, () => {
    console.log(`Server is running on port ${port}`);
});

async function publish_event() {
    let keypair = Keypair.generate();
    let connection = new Connection(clusterApiUrl("devnet"));

    let example_account = Keypair.fromSecretKey(Uint8Array.from(  [
            193, 116,  63,   6, 204,  84, 193,  95, 231, 195,  22,
            32, 223, 137,  98,  24, 205,  62, 237,  81, 125, 112,
            0, 223, 196, 216, 171, 210, 157,  21,  79,  34, 237,
            71, 202,  14, 166, 182,  41,  59,  92,   1,  95, 152,
            98, 170, 126, 202, 241, 170,  11, 252,  57, 228,  68,
            182,  94, 148, 194, 117, 207, 134, 137,  61
        ]
    ));
    const payer = Keypair.fromSecretKey(Uint8Array.from([221,167,145,0,107,5,15,45,239,234,184,39,135,139,96,110,143,143,141,248,29,148,193,194,177,238,177,76,253,93,1,246,177,237,204,154,176,189,224,84,225,141,57,103,141,2,83,181,229,244,249,17,73,245,150,118,135,72,24,24,87,87,230,43]))
    console.log("payer");
    console.log(example_account.publicKey);
    console.log(example_account.secretKey);

    console.log(JSON.stringify(web3.SystemProgram.programId));

    const transaction = new solanaWeb3.Transaction();
    //
    let publishAccounts: UpdateAccountAccounts = {
        exampleAccount: example_account.publicKey,
        signer: payer.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
    }
    //
    let updateAccountArgs: UpdateAccountArgs = {
        newName: "Name:" + Date.now()
    }
    console.log(updateAccountArgs)

    const ix = updateAccount(updateAccountArgs, publishAccounts)
    transaction.add(ix)

    let s = await sendAndConfirmTransaction(connection, transaction, [payer, example_account]);

    let fetch = await ExampleAccount.fetch(connection, example_account.publicKey);

    console.log(fetch.toJSON());
}