
import hre from "hardhat";

async function Deployer(contractName:string){
    try{
        const ContractFactory = await hre.ethers.getContractFactory(contractName); 
        const risc0VerifierContractAddressSepolia = "0x925d8331ddc0a1F0d96E68CF073DFE1d92b69187";
        const risczeroVerifierContractAddressHole = "0xf70aBAb028Eb6F4100A24B203E113D94E87DE93C" ; 
        const FibonachiContract = await ContractFactory.deploy(risc0VerifierContractAddressSepolia,825)
        await FibonachiContract.deploymentTransaction()?.wait(3)

        const FibonachiContractAddress = await FibonachiContract.getAddress();
        
        try {
            await hre.run("verify:verify", {
                address : FibonachiContractAddress,
                constructorArguments : [risc0VerifierContractAddressSepolia,825]
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
// contract address : 0xF9CB9306C7158978b18BbEfDFfa98852535bd5A4

// Contract address sepolia : 0x5036Ec86D742098D5f5cf5853094ee1AF37343d8
Deployer("FibonachiVerifier");