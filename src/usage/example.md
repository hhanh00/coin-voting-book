# Election Authority / Pollster

On July 2024,
The community wanted to poll the Zcash Holders regarding
the future of the DevFund past Nov 24.

> "What of the following proposals do you support?"

The options were:
- "None of these options",
- "Manufacturing Consent; Re-Establishing a Dev Fund for ECC, ZF, ZCG, Qedit, FPF, and ZecHub (by NoamChom)",
- "Establishing a Hybrid Dev Fund for ZF, ZCG and a Dev Fund Reserve (by Jack Gavigan)",
- "Lockbox For Decentralized Grants Allocation (perpetual 50% option) (by Skylar Saveland)",
- "Hybrid Deferred Dev Fund: Transitioning to a Non-Direct Funding Model (by Jason McGee, Peacemonger, GGuy)",
- "Lockbox For Decentralized Grants Allocation (20% option) (by Kris Nuttycombe)",
- "Masters Of The Universe? (by NoamChom)",
- "End the Dev Fund and return 100% of block rewards to miners"

From YWallet, users could use their funds to vote for their favorite proposal.

[Voting in YWallet](https://youtu.be/c-aFzW6kWNk)

The results were 
[published on the forums](https://forum.zcashcommunity.com/t/coin-weighted-poll/47964/160).

![Vote]({{rootUrl}}/images/2024-10-31_17-00-29.png "vote" =x600)
![Vote]({{rootUrl}}/images/2024-07-11_10-11-37.png "vote" =x600)
![Vote]({{rootUrl}}/images/2024-07-11_10-11-46.png "vote" =x600)
![Vote]({{rootUrl}}/images/2024-07-11_10-12-13.png "vote" =x600)
![Vote]({{rootUrl}}/images/2024-07-11_10-13-11.png "vote" =x600)
![Vote]({{rootUrl}}/images/2024-07-11_10-13-23.png "vote" =x600)

## What did we do to make it happen?

All the parameters that define the election/vote are stored in a
JSON file.

For instance, the previous vote had these settings:

```json
{
    "id": 2,
    "name": "Devfund Poll Proposals 2",
    "start_height": 2540000,
    "end_height": 2574200,
    "close_height": 2576000,
    "submit_url": "/submit/2",
    "question": "What proposal do you support?",
    "candidates": [
        "None of these options",
        "Manufacturing Consent; Re-Establishing a Dev Fund for ECC, ZF, ZCG, Qedit, FPF, and ZecHub (by NoamChom)",
        "Establishing a Hybrid Dev Fund for ZF, ZCG and a Dev Fund Reserve (by Jack Gavigan)",
        "Lockbox For Decentralized Grants Allocation (perpetual 50% option) (by Skylar Saveland)",
        "Hybrid Deferred Dev Fund: Transitioning to a Non-Direct Funding Model (by Jason McGee, Peacemonger, GGuy)",
        "Lockbox For Decentralized Grants Allocation (20% option) (by Kris Nuttycombe)",
        "Masters Of The Universe? (by NoamChom)",
        "End the Dev Fund and return 100% of block rewards to miners"
    ],
    "cmx": "233ea22fc2067c7141bb418f6cb71c308933eac205c3d79879c1c6acbed0a357",
    "nf": "03517198be86743e34ee057d5f41dc97b0a68da7d58eab0fee072af00288d472",
    "status": "Opened"
}
```

It was made manually but Coin Voting V2 will have a tool to assist
in its creation.

```admonish warning
The previous configuration file is for Coin Voting V1.
The second version will have some modifications
to support the improvements to the protocol.
```

Let's look at its content.

