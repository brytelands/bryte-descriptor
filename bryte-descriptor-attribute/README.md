# bryte-descriptor-attribute

This library provides macros for account and event structs that generate "descriptor" structs. These descriptor structs
hold the Borsh Schemas for your accounts or events. These schemas can be retrieved on-chain and used to deserialize 
account or event data.

This library supports both Solana programs and Anchor programs.

Please see the anchor-demo-program or bryte-descriptor-solana-demo for example usage.