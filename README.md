# taktctl

`taktctl` is a command line tool for customers that makes it easier to interact with the Takt API for repetitive tasks.

### Usage

```text
USAGE:
    taktctl [OPTIONS] --api-key <API_KEY> --api-key-secret <API_KEY_SECRET> --organization-id <ORGANIZATION_ID> <SUBCOMMAND>

OPTIONS:
        --api-key <API_KEY>
            API Key

        --api-key-secret <API_KEY_SECRET>
            API Key Secret

    -e, --endpoint <ENDPOINT>
            API Endpoint to use for Takt [default: https://api.alpha.takt.io]

    -h, --help
            Print help information

    -o, --organization-id <ORGANIZATION_ID>
            ID of the Organization

    -V, --version
            Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    upload    Upload a new file to a Takt Feed
```

#### Uploading a file to Takt
```text
USAGE:
    taktctl --api-key <API_KEY> --api-key-secret <API_KEY_SECRET> --organization-id <ORGANIZATION_ID> upload --feed-id <FEED_ID> <PATH>

ARGS:
    <PATH>

OPTIONS:
    -f, --feed-id <FEED_ID>    ID of the Takt Feed
    -h, --help                 Print help information
```
