# Blockchain Windows

```json
{
    "start_height": 2540000,
    "end_height": 2574200,
    "close_height": 2576000,
}    
```

## Registration Window

```admonish info
The registration window is between the **Start Height** and the **End Height**.
Only the Orchard notes **created** during the registration window are eligible.
```

If the end of the interval is in the future, the election definition must be
updated after the blockchain reaches that height.

However, the longer the range, the more notes need to be downloaded and
processed.

> Refer to the technical section for more information on the calculations
required.

The election does not necessarily have to include every output from the
blockchain; it may choose to skip some based on publicly agreeable criteria. For
example, notes that are part of transactions with more than 20 outputs could be
omitted to avoid the spam. However, skipping notes based on shielded data is not 
possible.

## Election Window

```admonish info
The Election window is between the **End Height** and the **Close Height**.
Users must **vote** during this period.

```

The close height is when the election stops accepting ballots. The client software should not send votes after this height. The check is enforced on the server side. If a faulty client sends votes past the close height, the election server rejects them.

> This interval is covered in more details in the next section.

## Status

The status is one of the following.
- Registration: the current height is in the election window. Voters should move
their funds and leave them unspent until the window closes. They cannot vote yet
because the window has not ended.
- Opened: Voters can vote.
- Closed: Ballots are not accepted anymore.

This is a recommendation for the client app. The server
decides whether to accept or reject,
based on the ballot content and the time it received
it.

## Note

Heights are not included in the calculation of the election hash and therefore
*may* be adjusted after the election starts.

However, the Election Authority should clearly announce any change as it may affect
the results.

If the Election Authority wants to commit to a given window they should explicitly
include it in the name of the election.
