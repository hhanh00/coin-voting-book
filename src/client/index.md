# Vote Client

The client is responsible for creating and submitting votes to the server.

A Ballot is built:
- from the (secret) data associated with an Orchard Note that the voter owns,
- and the note commitment and nullifier trees that the client obtains from the
public Zcash & Voting chains.

```admonish warning
The rust code in the next sections are for illustration purposes
and may not compile as is.
```
