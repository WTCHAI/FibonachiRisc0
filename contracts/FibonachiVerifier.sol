// SPDX-License-Identifier: Apache-2.0

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";
import { ImageID } from "./ImageID.sol";
contract FibonachiVerifier {
    // copy from risczero govenor 
    IRiscZeroVerifier public immutable verifier;
    bytes32 public immutable imageId = ImageID.FIBONACHI_ID ;

    uint256 private fibonachiResult ; 

    event ProofSubmittedLogged(
        address indexed prover,
        uint256 timestamp,
        bool status
    );


    constructor(address verifierAddress ){
        verifier = IRiscZeroVerifier(verifierAddress);
    }

    function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal) public  {
        // journal digested 
        bytes32 journalDiegst = sha256(journal) ; 
        // verify the proof
        try (verifier.verify(seal, imageId , journalDiegst)){ 
            // If verification is successful, update fibonachiResult and emit success event
            fibonachiResult = abi.decode(journal, (uint256));
            emit ProofSubmittedLogged(msg.sender, block.timestamp, true);
        }
        catch (bytes memory reason) {
            // If verification fails, emit the failed event with the reason
            emit ProofFailedLogged(msg.sender, block.timestamp,false);
        }
        fibonachiResult = abi.decode(journal, (uint256));

        emit ProofSubmittedLogged(msg.sender , block.timestamp);
    }

    function getFinalizeFibonachiResult() public view returns(uint256){
        return fibonachiResult ;
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