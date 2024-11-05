// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";
import { ImageID } from "./ImageID.sol";

contract FibonachiVerifier {
    // copy from risczero govenor 
    IRiscZeroVerifier public immutable verifier;
    bytes32 public immutable imageId = ImageID.FINALIZE_FIBONACHI_ID ;

    uint256 private fibonachiResult ; 

    event ProofSubmittedLogged(
        address indexed prover,
        uint256 timestamp,
        bool status
    );
    event CallingVerifier(address indexed prover);

    constructor(address verifierAddress ){
        verifier = IRiscZeroVerifier(verifierAddress);
    }

    function verifyAndFinalizeFibonachi(bytes calldata seal, bytes calldata journal) public  {
        emit CallingVerifier(msg.sender);
        // journal digested 
        bytes32 journalDiegst = sha256(journal) ; 
        fibonachiResult = abi.decode(journal, (uint256));
        // verify the proof
        try verifier.verify(seal, imageId , journalDiegst) { 
            // If verification is successful, update fibonachiResult and emit success event
            emit ProofSubmittedLogged(msg.sender, block.timestamp, true);
        }
        catch {
            // If verification fails, emit the failed event with the reason
            emit ProofSubmittedLogged(msg.sender, block.timestamp,false);
        }
    }

    function getFinalizeFibonachiResult() public view returns(uint256){
        return fibonachiResult ;
    }
}
