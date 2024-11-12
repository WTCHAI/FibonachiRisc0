
import hre from "hardhat";
import { ethers } from "hardhat";

async function Deployer(contractName:string){
    try{
        //Getting contract object wth specified contract 
        const ContractFactory = await hre.ethers.getContractFactory(contractName);
        
        const risczeroVerifierContractAddressHole = "0xf70aBAb028Eb6F4100A24B203E113D94E87DE93C" ; 
        const FibonachiContract = await ContractFactory.deploy(risczeroVerifierContractAddressHole,825)
        await FibonachiContract.deploymentTransaction()?.wait(3)

        const FibonachiContractAddress = await FibonachiContract.getAddress();
        
        try {
            await hre.run("verify:verify", {
                address : FibonachiContractAddress,
                constructorArguments : [risczeroVerifierContractAddressHole,825]
            })
        }catch(err){
            console.error("Error during verification:",err);
            process.exit(1);
        }
        console.log("Contract deployed to : ",await FibonachiContract.getAddress());
    }catch(err){
        console.error("Error during deployment:", err);
        process.exit(1);
    }
}

// After deploy 
// running script 
// npx hardhat run scripts/deploy.ts --network holesky
// npx hardhat run scripts/deployFibonachiVerifier.ts --network holesky
// contract address : 0x206ADBa7b67FF84519Add28bDa6b8b3fE14bd6CA

Deployer("FibonachiVerifier");

// Journal (Hex): 0xbe23000000000000
// Seal (Hex): 0x50bd176902ec380bfe4b997d3e927880a1f16ee2266ec67c0064f5c6f5fb1458248c588e2981d51d8019a7f6b9ffbd0b538190dba950c09330b10af7b2b15ac090b4b4182875e3a8e39f158b566290510a09fc29b5c079e6204c58464ca0370b630ac2f017cefaf0f9e0129a84bddecc819dc6a808588a03b4dc02efed91c1383add219b2b7dfc79c0aa21748f08d815074ad09db7908e37686d208c846be94ab2abfe952b2839ec925b32c932ff17fd7ef966077e81b7b73c2a758a24a0e875e1b94f9326ff6e0ff1a93eb46d5ec0599104eab7ccd1eef49ea100f409744d3216e94d3b0c3cac33787da061f6a962f4458a4531ed6688c48be3ec4b01ceee6291b6a219
// payload : JournalPayload { challenger: 0x41649a1f8b2499e2f7884184d062639cef9d0601, journal: [190, 35, 0, 0, 0, 0, 0, 0] }