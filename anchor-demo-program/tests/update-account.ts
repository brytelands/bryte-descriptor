import * as anchor from "@coral-xyz/anchor";
import web3, {LAMPORTS_PER_SOL} from "@solana/web3.js";
import solanaWeb3, {sendAndConfirmTransaction} from "@solana/web3.js";
import {updateAccount, UpdateAccountAccounts, UpdateAccountArgs} from "../app/generated/instructions";
import {ExampleAccount} from "../app/generated/accounts";
// @ts-ignore
import payer_demo_keypair from './payer_demo_keypair.json';
// @ts-ignore
import example_account_keypair from './example_account_keypair.json';

describe("update-account-demo", () => {

    it("Is initialized!", async () => {
        await publish_event();
    });
});

async function publish_event() {
    anchor.setProvider(anchor.AnchorProvider.env());
    let connection = anchor.AnchorProvider.env().connection;

    const exampleAccountArraySecret = Object.values(example_account_keypair._keypair.secretKey);
    // @ts-ignore
    const exampleAccountSecret = new Uint8Array(exampleAccountArraySecret)
    const example_account = web3.Keypair.fromSecretKey(exampleAccountSecret);

    const payerArraySecret = Object.values(payer_demo_keypair._keypair.secretKey);
    // @ts-ignore
    const payerSecret = new Uint8Array(payerArraySecret)
    const payer = web3.Keypair.fromSecretKey(payerSecret);

    const signature = await connection.requestAirdrop(payer.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(signature);

    const transaction = new solanaWeb3.Transaction();

    let publishAccounts: UpdateAccountAccounts = {
        exampleAccount: example_account.publicKey,
        signer: payer.publicKey
    }

    let updateAccountArgs: UpdateAccountArgs = {
        newName: "New Name: " + Date.now()
    }

    const ix = updateAccount(updateAccountArgs, publishAccounts)
    transaction.add(ix)

    await sendAndConfirmTransaction(connection, transaction, [payer, example_account]);

    let fetch = await ExampleAccount.fetch(connection, example_account.publicKey);
    console.log("ExampleAccount data:")
    console.log(fetch.toJSON());
}