# RPC

[API Definition](https://editor.jsight.io/r/MBJOPjv/1)
in [JSight](https://jsight.io/) API documentation format.

Available online through the link above.

```
JSIGHT 0.3

INFO
  Title "Zcash Coin Vote API"
  Version 1.0

URL /vote
  Protocol json-rpc-2.0
 
  Method submit // Submit a vote
    Params
      @vote
    Result
      "hex" // Transaction ID
 
  Method getLatestHeight // Get the latest block height
    Params
      @blockRequest
    Result
      100 // Block height
      
  Method getBlock // Get a block by height
    Params
      @blockRequest
    Result
      @block

  Method getBlocks // Get a range of blocks
    Params
      @blockRequestRange
    Result
      [@block]

  Method getVote // Get a ballot by ID
    Params
      "hex"
    Result
      @vote

#======================== TYPES ==========================

TYPE @vote
{
  "electionId": 1,
  "data": "base64string"
}

TYPE @blockRequest
{
  "electionId": 1,
  "height": 100
}

TYPE @blockRequestRange
{
  "electionId": 1,
  "start": 100,
  "end": 200
}

TYPE @block
{
  "electionId": 1,
  "height": 100,
  "data": "base64string"
}
```
