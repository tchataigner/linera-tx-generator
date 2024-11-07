# Linera transaction generator

The goal for this repository is to create a binary that can generate linera transactions
against a given network according to some test cases.

> **Note**
> The current repository is based of the `linera-protocol` `main` branch at
> commit [`d169df69`](https://github.com/linera-io/linera-protocol/tree/d169df69bcc6e861c3c54e8874c45f93401be08f).

## CLI

The current CLI can be ran with the following:

```bash
$ cargo run --release -- help

Usage: linera-tx-generator <COMMAND>

Commands:
  local   Local mode with `storage-address` and `path` arguments
  remote  Remote mode with `url` argument
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```

> **Warning**
> The current CLI only handles local network

## Local network

To run a local network, make sure that you have properly installed the linera
binary. If not you can follow [its documentation](https://linera.dev/developers/getting_started/installation.html).

Otherwise, it is possible to simple add the binaries available in this repository with:

```bash
export PATH="$PWD/target/release:$PATH"
source /dev/stdin <<<"$(linera net helper 2>/dev/null)"
```

## Test Cases

### Counter

The counter case
leverages [the counter example](https://github.com/linera-io/linera-protocol/tree/d169df69bcc6e861c3c54e8874c45f93401be08f/examples/counter)
from the `linera-protocol` repository.

It essentially publishes the counter bytecode and service, then proceeds to
increase the counter while checking through a node service that it has been properly
increased.

To run the test case, you can use the following to run a local linera network:

```bash
$ linera net up --testing-prng-seed 37
```

Once the network is up make sure that you note the path to its `/tmp/.tmp*`
directory.

Then you can run the test case in another CLI window with:

```bash
$ cargo run --release -- local --path <TMP_FOLDER> --case counter
```

### Blob

The blob case is meant to test the publication of different blob sizes on a network.
It publishes blobs of three different sizes: 500kb, 2mb and 5mb.

To run the test case you will first need to run a local linera network. Different
network configuration will result in different results. For example, the default
configuration will allow for a maximum blob size of 18446744073709551615mb.
While the devnet configuration will allow for a maximum blob size of 1mb.

The configuration can be run with the following:

```bash
# Default configuration
$ linera net up --testing-prng-seed 37

# Devnet configuration
$ linera net up --testing-prng-seed 37 --policy-config devnet
```

Once the network is up make sure that you note the path to its `/tmp/.tmp*`
directory.

To run the test case, you can use the following:

```bash
$ cargo run --release -- local --path <TMP_FOLDER> --case blob
```