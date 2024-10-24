# Vote Processing

## Vote Verification

Once a vote is received, the Voting Server VS
validates it.

1. The binary structure of the vote must be correct,
and it should parse into the BallotEnvelope, BallotData, etc.
1. The binding signature must match the sighash
1. The ZKP must be valid
1. The spend signatures must match the sighash and `rk`

The votes are available (in encrypted form) to everyone
in the forms of blocks produced by the VS.
