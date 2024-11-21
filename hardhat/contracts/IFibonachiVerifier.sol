// SPDX-License-Identifier: MIT 

pragma solidity ^0.8.27;

interface IFibonachiVerifier {
    /// @notice Returns the address of the RISC Zero verifier.
    /// @return Address of the IRiscZeroVerifier instance.
    function verifier() external view returns (address);

    /// @notice Returns the image ID used for verification.
    /// @return Bytes32 representation of the image ID.
    function imageId() external view returns (bytes32);

    /// @notice Returns the correct public Fibonacci output.
    /// @return The correct Fibonacci result.
    function correct_public_output() external view returns (uint256);

    /// @notice Allows a challenger to submit their proof.
    /// @param fibo_result The Fibonacci result to verify.
    /// @param seal The cryptographic seal proving correctness.
    function challenge(uint256 fibo_result, bytes memory seal) external;

    /// @notice Checks the challenge status of a challenger.
    /// @param challenger The address of the challenger.
    /// @return `true` if the challenge was successful, `false` otherwise.
    function challengeStatus(address challenger) external view returns (bool);
}