// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0;

address constant INEXT_STEP_PRECOMPILED = address(0x0000000000000000000000000000000000000800);

interface INextStep {
    /**
     * return whether action is done
     * task_id: aa5ac585
     *
     * @param deprocess Decentralized Process id
     * @param action Action name to be checked for completeness
     */
    function is_acted(bytes32 deprocess, string memory action) external view returns(bool);
}

INextStep constant NextStep = INextStep(INEXT_STEP_PRECOMPILED);
