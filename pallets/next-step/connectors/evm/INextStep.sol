// SPDX-License-Identifier: GPL-3.0

pragma solidity >=0.8.0;

/**
 * @title Pallet NextStep Interface
 *
 * The interface through which solidity contracts will interact with pallet-next-step.mapping
 * Address :    0x0000000000000000000000000000000000000800
 */


interface INextStep {

    /**
     * @dev return whether a Decentralized Process has ended or not
     * Selector: a9059cbb
     * @param deprocess Decentralized Process ID.
     */
    function is_ended(uint256 deprocess) external returns (bool);

    /**
     * @dev return whether a Decentralized Process Action has been acted or not
     * Selector: a9059cbb
     * @param deprocess Decentralized Process ID.
     * @param action Decentralized Process Action name.
     */
    function is_acted(uint256 deprocess, string calldata action) external returns (bool);


    /**
     * @dev Event emited when a Decentralized Process has started.
     * Selector: 8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925
     * @param deprocess Decentralized Process ID.
     */
    event Start(
        uint256 deprocess
    );

    /**
     * @dev Event emited when an Decentralized Process Action has been acted(Stepped).
     * Selector: 8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925
     * @param deprocess Decentralized Process ID.
     * @param action Decentralized Process Action name.
     */
    event Step(
        uint256 indexed deprocess,
        string indexed action
    );
}

address constant NextStep_PRECOMPILED = address(0x2000);

INextStep constant NextStep = INextStep(NextStep_PRECOMPILED);
