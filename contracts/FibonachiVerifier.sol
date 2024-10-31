// SPDX-License-Identifier: Apache-2.0

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";


contract FibonachiVerifier {
    // copy from risczero govenor 
    IRiscZeroVerifier public immutable verifier;
    bytes32 public immutable imageId;

    constructor(address verifierAddress , bytes32 imageId_){
        verifier = IRiscZeroVerifier(verifierAddress);
        imageId = imageId_;
    }

    function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal) external {
        // journal digested 
        bytes32 journalDiegst = sha256(journal) ; 
        // verify the proof
        verifier.verify(seal, imageId , journalDiegst);

        // decode the journal and extract to Fibonachi result
        // uint256 result;
        // assembly {
        //     result := mload(add(journal, 32))
        // }
        // lastVerifiedFibonacci = result;

        // emit FibonacciVerified(result);
        // _finalizeVotes(proposalId, ballotHash, votingData);
    }
}

    // function verifyAndFinalizeVotes(bytes calldata seal, bytes calldata journal) public {
    //     // verify the proof
    //     verifier.verify(seal, imageId, sha256(journal));

    //     // decode the journal
    //     uint256 proposalId = uint256(bytes32(journal[0:32]));
    //     bytes32 ballotHash = bytes32(journal[32:64]);
    //     bytes calldata votingData = journal[64:];

    //     _finalizeVotes(proposalId, ballotHash, votingData);
    // }

// contract FibonacciVerifier {
//     IRiscZeroVerifier public verifier;
//     bytes32 public imageId;

//     event Verified(uint256 position, uint256 result);

//     constructor(bytes32 _imageId, IRiscZeroVerifier _verifier) {
//         imageId = _imageId;
//         verifier = _verifier;
//     }

//     function verifyFibonacciProof(bytes calldata seal, bytes32 journalDigest) external {
//         // Verify the proof using RISC Zero's zk-STARK verifier
//         verifier.verify(seal, imageId, journalDigest);

//         // Assuming the journal contains the Fibonacci result, emit it
//         emit Verified( /* Position */, /* Result from journalDigest */ );
//     }
// }