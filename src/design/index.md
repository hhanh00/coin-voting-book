# Design

Coin Voting is essentially the same as a token system, i.e., the same as Zcash
itself, with some adjustments. Instead of coins, we have tokens that represent
the voting power. We want to distribute the voting power based on a verified
amount of ZEC. 

In principle, we are airdropping voting power. 

Transactions are
exchanges of voting power between participants. Essentially, we allow electors
to delegate their vote. The election's winner is the candidate or the choice
with the most votes at the end.

The Coin Voting leverages the Orchard system and uses its transaction system to support delegation.

It extends it with the ability to include existing notes from the Orchard pool. Coinbase transactions add grants that reward critical organizations and influential individuals. 

Otherwise, Voting Power is a self-contained system: There is no minting or
burning.