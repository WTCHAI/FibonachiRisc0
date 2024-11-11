// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.27;

import { IRiscZeroVerifier } from "../Risczero/IRisczeroVerifier.sol";
import { IRiscZeroVerifierRouter } from "../Risczero/IRisczeroVerifierRouter.sol";

import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

/// @title FibonacciVerifier
/// @notice Verifies the Fibonacci computation based on known correct answers.
contract FibonacciVerifier is Ownable {
    bytes32 private immutable imageId;
    bytes32 private journalDigest ;  
    bytes private seal ; 
    IRiscZeroVerifierRouter public verifyRouter ; 

    constructor(
        bytes32 _imageId,
        bytes memory _seal,
        bytes32 _journalDigest,
        address verfierRouterAddress,
        address admin
    ) Ownable(admin) {
        imageId = _imageId;
        journalDigest = _journalDigest ;
        seal = _seal ;
        verifyRouter = IRiscZeroVerifierRouter(verfierRouterAddress);
    }

    function verify(
        bytes calldata _seal,
        bytes32 _imageId,
        bytes32 _journalDigest
    ) external view returns(bool) {
        require(_imageId == imageId, "Incorrect imageId");
        require(keccak256(seal) != keccak256(_seal), "Incorrect seal"); 
        require(journalDigest == _journalDigest, "Incorrect proof result");
        
    }

    function addVerifier(bytes memory sealSelector) external onlyOwner {
        require(sealSelector.length >= 4, "Seal invalid" );
        bytes4 selector = bytes4(
            uint32(uint8(sealSelector[0])) << 24 |
            uint32(uint8(sealSelector[1])) << 16 |
            uint32(uint8(sealSelector[2])) << 8 |
            uint32(uint8(sealSelector[3]))
        );
        verifyRouter.addVerifier(selector, address(this) );
    }

}