## Test instructions:

1. Pull repository code.
2. Go to the project root.
3. Create a `.env` file (copy `.env.sample`).
4. Add your desired nodes IP/ports in `./data/nodes.csv` (some provided for demo purposes).
5. Modify the desired log level in the `.env` file.
6. Execute `cargo run`.

**Notes:** Compatible with nodes over version 70001.

## Environment versions

- cargo 1.78.0 (54d8815d0 2024-03-26)
- rustc 1.78.0 (9b00956e5 2024-04-29)
- node v20.13.1

**Notes:** An output log example can be found at `./data/log.txt`.

## Bitcoin commands | TODO for full node

- **Version:** Nodes use this to communicate their protocol versions upon connecting.
- **Verack:** This is an acknowledgment message used in response to a 'version' message.
- **Addr:** Provides network address information, useful for node discovery.
- **Inv:** Allows a node to advertise its knowledge of one or more objects.
- **Getdata:** Used by a node to request specific objects from another node.
- **Notfound:** A response to a 'getdata' request when the requested data is not available.
- **Getblocks:** Requests a list of blocks starting from a specific ID.
- **Getheaders:** Similar to 'getblocks', but used to request block headers.
- **Tx:** Represents a bitcoin transaction.
- **Block:** Represents a full block in the blockchain.
- **Headers:** Sends block headers to a node.
- **Getaddr:** Requests an address from a peer.
- **Mempool:** Requests information about transactions a node has in its transaction pool but not yet in a block.
- **Ping:** Used to confirm that the TCP/IP connection is still valid.
- **Pong:** Response to a 'ping' command to indicate that the connection is still open.
