// SPDX-License-Identifier: MIT

contract INextStep {
    function is_acted(uint32 deprocess, string action) returns (bool);
    function act(uint32 deprocess, uint256 action_data, string action) returns (bool);
}

contract CriticalLevel {

    address INEXT_STEP_PRECOMPILED = address(0x0000000000000000000000000000000000000800);
    INextStep NextStep = INextStep(INEXT_STEP_PRECOMPILED);

    function act(uint32 deprocess, uint256 action_data) public returns (bool) {
        return NextStep.act(deprocess, action_data, "Critical Level");
    }
}
