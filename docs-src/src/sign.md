## Sign

**Warning: These commands modify your NFT and are for advanced users. Use with caution.**

Sign metadata for an unverified creator.

### Sign One

Sign the metadata for a single mint account.

#### Usage

```bash
cooper sign one --keypair <PATH_TO_KEYPAIR> --account <MINT_ACCOUNT>
```

Outputs a TxId to the command line so you can check the result.

### Sign All

Sign all metadata from a JSON list or for a given first verified creator. First verified creator can be the tars creator id or whatever the first verified creator in the creators array is for your NFTs.

#### Usage

```bash
cooper sign all --keypair <PATH_TO_KEYPAIR> --creator <FIRST_CREATOR>
```

**For tars v2, you can add the `--v2` option when using it with the tars id.**
Tars v2 has a separate creator id from the tars account id. 

```bash
cooper sign all --keypair <PATH_TO_KEYPAIR> --creator <TARS_ID> --v2
```

or you can use the tars creator id which will be the first creator in the creators array.

```bash
cooper sign all --keypair <PATH_TO_KEYPAIR> --creator <TARS_CREATOR_ID>
```

With a mint acconts JSON list:

```bash
cooper sign all --keypair <PATH_TO_KEYPAIR> --mint-accounts-file <PATH_TO_MINT_ACCOUNTS_FILE>
```

For the latter usage, the mint accounts file should be a JSON file with a list of mint accounts to be signed:

```json
[
    "C2eGm8iQPnKVWxakyo8QhwJUvYrZHKF52DPQuAejpTWG",
    "8GcRqxy4VAocTcAkoxCXkPCEmM36HMtjBc8ZarWhAD6o",
    "CK2npuck3WTRNFXSdZv8YjudJJEa69EVGd6GFfeSzfGP"
]
```

Outputs a TxId to the command line so you can check the result.
