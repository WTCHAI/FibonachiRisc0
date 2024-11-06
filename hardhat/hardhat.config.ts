import { HardhatUserConfig } from "hardhat/config";

import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-verify";

require("dotenv").config();

const config: HardhatUserConfig = {
  solidity: "0.8.27",
  networks: {
    // sepolia: {
    //   url: `https://eth-sepolia.g.alchemy.com/v2/${process.env.ALCHEMY_API_KEY}`,
    //   accounts: [`0x${process.env.PRIVATE_KEY_MY_WALLET_SEPOLIA}`],
    // },
    holesky:{
      url: `${process.env.HOLESKY_RPC_URL}`,
      accounts: [process.env.PRIVATE_KEY_HOLESKY!]
    }
  },
  etherscan: { 
    apiKey : process.env.ETHERSCAN_API_KEY
  },
  sourcify: {
    enabled: false
  },
  gasReporter:{
    enabled: true
  }
};

export default config;
