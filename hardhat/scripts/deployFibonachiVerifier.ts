
import hre from "hardhat";

async function Deployer(contractName:string){
    try{
        //Getting contract object wth specified contract 
        const ContractFactory = await hre.ethers.getContractFactory(contractName);
        
        const risczeroVerifierContractAddress = "0xf70aBAb028Eb6F4100A24B203E113D94E87DE93C" ; 
        // Getting list of accounts avaliable from hardhat 
        // Deployer in case didn't set the account private key in hardhat config & env
        // const deployers = await hre.ethers.getSigners() ; 
        // console.log("Deploying contract with the account : ",deployers)

        // Deploying the contract using the deployer's signer [0] related on hardhat config
        // Which contract is require owner so we have to connect owner = msg.sender  
        // Refactor deploying without telling who is signers cause already declare in env & hardhat config
        const FibonachiContract = await ContractFactory.deploy(risczeroVerifierContractAddress)
        await FibonachiContract.deploymentTransaction()?.wait(3)

        // Verifing contract 
        const FibonachiContractAddress = await FibonachiContract.getAddress();
        console.log("Contract deployed to : ",await FibonachiContract.getAddress());
        try {
            await hre.run("verify:verify", {
                address : FibonachiContractAddress,
                constructorArguments : [risczeroVerifierContractAddress]
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
// contract address : 0x9e9Dcc9D7ace2d759126fEa9EefCf72e2eB398Ec
Deployer("FibonachiVerifier");
