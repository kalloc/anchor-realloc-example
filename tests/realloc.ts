import * as anchor from "@coral-xyz/anchor";
import {Program} from "@coral-xyz/anchor";
import {Realloc} from "../target/types/realloc";
import {expect} from "chai";

describe("realloc", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.Realloc as Program<Realloc>;

    it("Is initialized!", async () => {
        console.log("Calling initialize");
        await program.methods.initialize().rpc();
    });

    it("realloc", async () => {
        const [programPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("data")], program.programId);
        {
            const info = await program.provider.connection.getAccountInfo(programPDA);
            console.log(info.data);
            // @ts-ignore
            expect(info.space!).to.equal(1000);
        }
        console.log("Calling overwrite_discriminator");
        try {
            await program.methods.overwriteDiscriminator().rpc();
        } catch (e) {
            console.log(e);
            throw e;
        }
        {
            const info = await program.provider.connection.getAccountInfo(programPDA);
            console.log(info.data);
            // @ts-ignore
            expect(info.space!).to.equal(1000);
        }
        console.log("Calling realloc");
        await program.methods.realloc().rpc();
        {
            const info = await program.provider.connection.getAccountInfo(programPDA);
            console.log(info.data);
            // @ts-ignore
            expect(info.space!).to.equal(2000);
        }
    })
});
