// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

import { IRiscZeroVerifier, Receipt } from "./IRisczeroVerifier.sol";

interface IRiscZeroVerifierRouter is IRiscZeroVerifier {
    function addVerifier(bytes4 selector, address verifier) external ;

    function removeVerifier(bytes4 selector) external;

    function getVerifier(bytes4 selector) external view returns (IRiscZeroVerifier);

    function getVerifier(bytes calldata seal) external view returns (IRiscZeroVerifier);

    function verify(bytes calldata seal, bytes32 imageId, bytes32 journalDigest) external view override;

    function verifyIntegrity(Receipt calldata receipt) external view override;
}