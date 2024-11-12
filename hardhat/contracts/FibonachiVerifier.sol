// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.20;

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";
import { ImageID } from "./ImageID.sol";

contract FibonachiVerifier {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.FINALIZE_FIBONACHI_ID;
    uint256 public correct_public_output ;

    mapping (address => bool ) IsProverCorrect ;

    constructor(IRiscZeroVerifier _verifier , uint256 correct_public_output_) {
        verifier = _verifier ;
        correct_public_output = correct_public_output_ ; 
    }

    function challenge(address challengerAddr , uint256 fibo_result , bytes memory seal) public {
        bytes memory journal = abi.encode(fibo_result);
        require(fibo_result == correct_public_output,"Public output mismatch!") ; 
        try verifier.verify(seal, imageId, sha256(journal)) {
            IsProverCorrect[challengerAddr] = true;
        } catch {
            IsProverCorrect[challengerAddr] = false;
        }   
    }

    function challengeStatus(address challenger) public view returns (bool) {
        return IsProverCorrect[challenger] ; 
    }
}