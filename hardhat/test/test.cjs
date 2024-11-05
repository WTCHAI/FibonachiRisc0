// const { expect } = require("chai");
// const { ethers } = require("hardhat");

// describe("FibonachiVerifier", function () {
//     let fibonachiVerifier;
//     let verifierMock; // Mock of IRiscZeroVerifier for testing
//     let owner;
//     let otherAccount;

//     beforeEach(async function () {
//         [owner, otherAccount] = await ethers.getSigners();
//         // Deploy a mock contract for IRiscZeroVerifier to simulate `verify` function behavior.
//         const VerifierMock = await ethers.getContractFactory("FibonachiVerifier");
//         verifierMock = await VerifierMock.deploy();
//         await verifierMock.deployed();

//         // Deploy the FibonachiVerifier contract with the mock verifier address
//         const FibonachiVerifier = await ethers.getContractFactory("FibonachiVerifier");
//         fibonachiVerifier = await FibonachiVerifier.deploy(verifierMock.address);
//         await fibonachiVerifier.deployed();
//     });

//     it("Should log success on valid proof", async function () {
//         // Mock a valid seal and journal
//         const journal = ethers.utils.defaultAbiCoder.encode(["uint256"], [6100]);
//         const seal = ethers.utils.randomBytes(32);

//         // Simulate the verifier mock to accept the proof
//         await verifierMock.setVerificationResult(true);

//         const tx = await fibonachiVerifier.verifyAndFinalizeFibonachi(seal, journal);
//         await expect(tx)
//             .to.emit(fibonachiVerifier, "ProofSubmittedLogged")
//             .withArgs(owner.address, anyValue, true);

//         // Check that fibonachiResult is set correctly
//         const result = await fibonachiVerifier.getFinalizeFibonachiResult();
//         expect(result).to.equal(6100);
//     });

//     it("Should log failure on invalid proof", async function () {
//         // Mock an invalid seal and journal
//         const journal = ethers.utils.defaultAbiCoder.encode(["uint256"], [6100]);
//         const seal = ethers.utils.randomBytes(32);

//         // Simulate the verifier mock to reject the proof
//         await verifierMock.setVerificationResult(false);

//         const tx = await fibonachiVerifier.verifyAndFinalizeFibonachi(seal, journal);
//         await expect(tx)
//             .to.emit(fibonachiVerifier, "ProofSubmittedLogged")
//             .withArgs(owner.address, anyValue, false);

//         // Check that fibonachiResult is still zero
//         const result = await fibonachiVerifier.getFinalizeFibonachiResult();
//         expect(result).to.equal(0);
//     });
// });