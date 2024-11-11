// // SPDX-License-Identifier: Apache-2.0
// pragma solidity ^0.8.27;

// import { IRiscZeroVerifier } from "../Risczero/IRisczeroVerifier.sol";

// /// @title FibonacciChallenge
// /// @notice Allows students to generate and submit Fibonacci proofs.
// contract FibonacciChallenge {
//     RiscZeroVerifierRouter public router;

//     // Track nonces to prevent replay attacks
//     mapping(address => bytes32) public currentNonces;

//     constructor(address routerAddress) {
//         router = RiscZeroVerifierRouter(routerAddress);
//     }

//     /// @notice Request a unique nonce for proof generation
//     function requestNonce() public returns (bytes32) {
//         bytes32 nonce = keccak256(abi.encodePacked(msg.sender, block.timestamp));
//         currentNonces[msg.sender] = nonce;
//         return nonce;
//     }

//     /// @notice Submit the proof to the verifier router
//     function submitProof(
//         bytes calldata seal,
//         bytes32 imageId,
//         bytes32 journalDigest
//     ) public {
//         // Check that the nonce is valid
//         bytes32 nonce = currentNonces[msg.sender];
//         require(nonce != bytes32(0), "Nonce not found. Request a nonce first.");

//         // Submit to the router, which routes to the correct verifier
//         router.verify(seal, imageId, journalDigest);

//         // Clear the nonce once used
//         delete currentNonces[msg.sender];
//     }
// }