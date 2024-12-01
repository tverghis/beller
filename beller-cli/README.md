# Beller CLI

A command-line interface for interacting with ATProto/Bluesky Personal Data Server (PDS) services.

> [!NOTE]
> Currently, the implemented commands focus on operations required to set up a labeler service. Other operations may be added in the future. Contributions are welcome!

## Installation

To install from source:

```sh
cargo install --path .
```

## Usage

The CLI supports the following commands:

### Create Session

Creates a new session on the PDS using your credentials:

```bash
beller create-session -u your-handle-or-did -p your-password
```

The command will return a JSON response containing your session information, including the access token.

### Request PLC Operation Signature

Requests a PLC operation signature, which will trigger an email confirmation:

```bash
beller request-plc-operation-signature --access-token your-access-token
```

After running this command, check your email for a confirmation code.

### Global Options

- `-s, --pds <URL>`: Specify the PDS URL (defaults to https://bsky.social)

## Examples

1. Create a session with custom PDS:

```bash
beller create-session -u alice.bsky.social -p mypassword --pds https://custom-pds.example.com
```

2. Create a session with default PDS:

```bash
beller create-session -u alice.bsky.social -p mypassword
```

3. Request PLC operation signature:

```bash
beller request-plc-operation-signature --access-token "jwt_token_from_create_session"
```

## Error Handling

The CLI will exit with status code 1 and display an error message if any operation fails. Successful operations will display their results and exit with status code 0.

## Dependencies

- [beller-lib](../beller-lib): Core library implementing ATProto interactions

## License

[MIT](LICENSE)
