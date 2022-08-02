#DEVELOPMENT ACCOUNTS
ALICE="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
BOB="5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
CHARLIE="5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y"
DAVE="5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy"
EVE="5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw"
FERDIE="5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL"

#make sure all account have balance to do transactions
polkadot-js-api tx.balances.transfer $BOB 12345 --seed "//Alice"
polkadot-js-api tx.balances.transfer $CHARLIE 12345 --seed "//Alice"
polkadot-js-api tx.balances.transfer $DAVE 12345 --seed "//Alice"
polkadot-js-api tx.balances.transfer $EVE 12345 --seed "//Alice"
polkadot-js-api tx.balances.transfer $FERDIE 12345 --seed "//Alice"

# #assing role names to accounts, only these accounts can assume roles for next Decentralized Process to be deployed by Alice
polkadot-js-api tx.nextStep.assign "Farmer" "$FERDIE" --seed "//Alice"
polkadot-js-api tx.nextStep.assign "Distributor" "$DAVE" --seed "//Alice"
polkadot-js-api tx.nextStep.assign "Cheese shop" "$CHARLIE" --seed "//Alice"
polkadot-js-api tx.nextStep.assign "Buyer" "$BOB" --seed "//Alice"

# deploy a Decentralized Process to Next Step

# deploy Escrow Contract

# do Actions Steps using Assigned Accounts, uptill specific Escrow Step

# Withdraw From Escrow Account

