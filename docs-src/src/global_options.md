## Global Options

These are the options that apply to all subcommands and can be passed in at any level.

```bash
cooper <option> <subcommand> <subcommand>
```

```bash
cooper <subcommand> <option> <subcommand>
```

```bash
cooper <subcommand> <subcommand> <option>
```

## Options

-r, --rpc <rpc> The RPC endpoint to use for commands.

Cooper will try to read your Solana config settings for both the RPC endpoint and also the Commitment setting by reading from `$HOME/.config/solana/cli/config.yml`. If it can't find a config file it defaults to using `https://psytrbhymqlkfrhudd.dev.genesysgo.net:8899/` and `confirmed`.

Running Cooper with the `--rpc` option will override the above with whatever RPC endpoint the user provides.

-t, --timeout <timeout> The timeout in seconds to use for RPC calls.

This defaults to 90 seconds which should be fine for most cases but can be overriden if needed.

Example:

```bash
cooper snapshot holders -r https://ssc-dao.genesysgo.net/ -t 120 -u DC2mkgwhy56w3viNtHDjJQmc7SGu2QX785bS4aexojwX
```