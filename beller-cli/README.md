# Beller CLI

A command-line interface for interacting with ATProto/Bluesky Personal Data Server (PDS) services.

> [!NOTE]
> Currently, the implemented commands focus on operations required to set up a labeler service. Other operations may be added in the future. Contributions are welcome!

> [!WARNING]
> This software is currently under heavy development. Not all commands have been implemented, and the ones that have will likely change.
> See [TODO](TODO.md) for more details.

## Installation

To install from source:

```sh
$ cargo install --path .
```

## Usage

The CLI organizes commands into the following groups:

- `api`: Commands for interacting with the XRPC API
- `crypto`: Cryptography-related commands

### API Commands

> [!NOTE]
> API commands may specify a PDS URL. If not specified, Bluesky's PDS
> (`https://bsky.social`) will be used.

#### Create Session

Creates a new session on the PDS using your credentials:

```bash
$ beller api create-session -u your-handle-or-did -p your-password
```

The command will return a JSON response containing your session information, including the access token.

#### Request PLC Operation Signature

Requests a PLC operation signature, which will trigger an email confirmation:

```bash
$ beller api request-plc-operation-signature <access_token>
```

After running this command, check your email for a confirmation code.

#### Get Recommended DID Credentials

Retrieves the recommended DID credentials for a given account:

```bash
$ beller api get-recommended-did-credentials <access_token>
```

The command returns a JSON response containing the recommended DID credentials for the account.

### Cryptography Commands

#### Generate Private Key

Generates an ECDSA private key using the secp256k1 curve:

```bash
$ beller crypto generate-private-key
```

The command outputs a base16-encoded string of the private key bytes.

#### Retrieve Public Key

Derives the public key from a base16-encoded private key:

```bash
$ beller crypto retrieve-public-key <private_key>
```

The command outputs the public key encoded according to [this specification](https://atproto.com/specs/cryptography#public-key-encoding).

## Examples

1. Create a session with custom PDS:

```bash
$ beller api create-session -u alice.bsky.social -p mypassword --pds https://custom-pds.example.com
```

2. Create a session with default PDS:

```bash
$ beller api create-session -u alice.bsky.social -p mypassword
```

3. Generate a private key:

```bash
$ beller crypto generate-private-key
```

## Error Handling

The CLI will exit with status code 1 and display an error message if any operation fails. Successful operations will display their results and exit with status code 0.

## Dependencies

- [beller-lib](../beller-lib): Core library implementing ATProto interactions

## License

[MIT](LICENSE)
