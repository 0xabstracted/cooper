## Update List of NFT Metadata

1. Get your NFT mint list using `cooper snapshot` or another tool.
2. Decode all the metadata into files using `cooper decode mint -L <NFT_MINT_LIST_FILE> --full -o <DATA_FILES_DIR>`.
3. Update the specific data you want changed in each file in the `<DATA_FILES_DIR>`.
4. Update the NFTs with `cooper update data-all -d <DATA_FILES_DIR>`.

**Note**: many fields have specific `update <field>-all` commands which are easier to use as they don't require updating a bunch of metadata files manually. Check the `update` and `set` sections first to see if they have what you need.