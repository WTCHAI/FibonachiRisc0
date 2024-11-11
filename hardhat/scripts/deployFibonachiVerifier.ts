
import hre from "hardhat";

async function Deployer(contractName:string){
    try{
        //Getting contract object wth specified contract 
        const ContractFactory = await hre.ethers.getContractFactory(contractName);
        
        const risczeroVerifierContractAddressHole = "0xf70aBAb028Eb6F4100A24B203E113D94E87DE93C" ; 
        const risczeroVerifierContractAddressSepolia = "0x925d8331ddc0a1F0d96E68CF073DFE1d92b69187" ; 
        // Getting list of accounts avaliable from hardhat 
        // Deployer in case didn't set the account private key in hardhat config & env
        // const deployers = await hre.ethers.getSigners() ; 
        // console.log("Deploying contract with the account : ",deployers)

        // Deploying the contract using the deployer's signer [0] related on hardhat config
        // Which contract is require owner so we have to connect owner = msg.sender  
        // Refactor deploying without telling who is signers cause already declare in env & hardhat config
        const FibonachiContract = await ContractFactory.deploy(risczeroVerifierContractAddressHole)
        await FibonachiContract.deploymentTransaction()?.wait(3)

        // Verifing contract 
        const FibonachiContractAddress = await FibonachiContract.getAddress();
        console.log("Contract deployed to : ",await FibonachiContract.getAddress());
        try {
            await hre.run("verify:verify", {
                address : FibonachiContractAddress,
                constructorArguments : [risczeroVerifierContractAddressHole]
            })
        }catch(err){
            console.error("Error during verification:",err);
            process.exit(1);
        }
    }catch(err){
        console.error("Error during deployment:", err);
        process.exit(1);
    }
}

// After deploy 
// running script 
// npx hardhat run scripts/deploy.ts --network holesky
// npx hardhat run scripts/deployFibonachiVerifier.ts --network holesky
// contract address : 0x1E0122e128b316381E439e3aAcDD1aE88E7669F7

// 0x29A4E3723f249317A4Ed0d01739D23ADB4B4DaE7
Deployer("FibonachiVerifier");
