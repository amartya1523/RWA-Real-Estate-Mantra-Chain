const mantra = {
    chain_name: "mantrachaintestnet",
    status: "active",
    network_type: "testnet",
    pretty_name: "MANTRA Chain Testnet",
    chain_id: "mantrachain-testnet-1",
    website: "https://www.mantrachain.io",
    bech32_prefix: "mantra",
    bech32_config: {
        bech32PrefixAccAddr: "mantra",
        bech32PrefixAccPub: "mantrapub",
        bech32PrefixValAddr: "mantravaloper",
        bech32PrefixValPub: "mantravaloperpub",
        bech32PrefixConsAddr: "mantravalcons",
        bech32PrefixConsPub: "mantravalconspub",
    },
    daemon_name: "mantrad",
    slip44: 118,
    fees: {
        fee_tokens: [
            {
                denom: "AUM",
                low_gas_price: 0.01,
                average_gas_price: 0.025,
                high_gas_price: 0.03,
            },
        ],
    },
    staking: {
        staking_tokens: [
            {
                denom: "AUM",
            },
        ],
    },
    apis: {
        rpc: [
            {
                address: "https://rpc.testnet.mantrachain.io",
            },
        ],
        rest: [
            {
                address: "https://api.testnet.mantrachain.io",
            },
        ],
    },
};

// Export the mantra object as a named export
export default mantra;