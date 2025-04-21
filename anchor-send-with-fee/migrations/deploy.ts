import * as anchor from "@coral-xyz/anchor";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);

  console.log(
    "🏋️ Welcome to the Solana workbench - where your transactions get a workout!"
  );
};
