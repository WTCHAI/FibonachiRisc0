// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";
import { ImageID } from "./ImageID.sol";

contract FibonachiVerifier {
    // copy from risczero govenor 
    IRiscZeroVerifier public immutable verifier;
    bytes32 public immutable imageId = ImageID.FINALIZE_FIBONACHI_ID ;

    uint256 private fibonachiResult ; 
    bytes32 private journalDigest ; 
    bytes private seal ; 

    event ProofSubmittedLogged(
        address indexed prover,
        uint256 timestamp,
        bool status
    );
    event CallingVerifier(address indexed prover);

    constructor(address verifierAddress ){
        verifier = IRiscZeroVerifier(verifierAddress);
    }

    function verifyAndFinalizeFibonachi(
        bytes calldata seal_,
        bytes calldata journal
    ) public {
        emit CallingVerifier(msg.sender);
        // journal digested 
        require(journal.length != 0, "Journal data not valid bytes"); 

        journalDigest = sha256(journal) ;
        fibonachiResult = (
            (uint64(uint8(journal[0])) << 56) |
            (uint64(uint8(journal[1])) << 48) |
            (uint64(uint8(journal[2])) << 40) |
            (uint64(uint8(journal[3])) << 32) |
            (uint64(uint8(journal[4])) << 24) |
            (uint64(uint8(journal[5])) << 16) |
            (uint64(uint8(journal[6])) << 8) |
            uint64(uint8(journal[7]))
        );        
        seal = seal_ ; 

        // verify the proof
        try verifier.verify(seal, imageId , journalDigest) { 
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

    function getJournalDigested() public view returns(bytes32){
        return journalDigest ; 
    }

    function getSealDigested() public view returns(bytes memory) { 
        return seal ; 
    }
}
