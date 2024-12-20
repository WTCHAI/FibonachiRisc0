// SPDX-License-Identifier: MIT

pragma solidity ^0.8.27;

import { IRiscZeroVerifier } from "./Risczero/IRisczeroVerifier.sol";
import { ImageID } from "./ImageID.sol";

contract FibonachiVerifier {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.FINALIZE_FIBONACHI_ID;
    uint256 public correct_public_output ;

    event ChallengeLog(address indexed challenger, bool isCorrect);

    mapping (address => bool ) IsProverCorrect ;

    constructor(IRiscZeroVerifier _verifier , uint256 correct_public_output_) {
        verifier = _verifier ;
        correct_public_output = correct_public_output_ ; 
    }

    function challenge(
        uint256 fibo_result,
        bytes memory seal
    ) public {
        bytes memory journal = abi.encode(fibo_result);
        address challengerAddr = msg.sender;
        try verifier.verify(seal, imageId, sha256(journal)) {
            emit ChallengeLog(challengerAddr, true);
            IsProverCorrect[challengerAddr] = (fibo_result == correct_public_output);
        } catch {
            emit ChallengeLog(challengerAddr, false);
            IsProverCorrect[challengerAddr] = false;
        }   
    }

    function challengeStatus(address challenger) public view returns (bool) {
        return IsProverCorrect[challenger] ; 
    }
}