// SPDX-License-Identifier: MIT

contract INextStep {
    function is_acted(uint32 deprocess, string action) external returns(bool);
    function act(uint32 deprocess, string action, uint256 action_data) external returns(bool);
}

contract HelloNextStep {

    address INEXT_STEP_PRECOMPILED = address(0x0000000000000000000000000000000000000800);
    INextStep NextStep = INextStep(INEXT_STEP_PRECOMPILED);

    function is_acted(uint32 adeprocess, string aaction) public returns (bool) {                 
        return NextStep.is_acted(adeprocess, aaction);
    }

    function act(uint32 adeprocess, string aaction, uint256 aaction_data) public returns (bool) {
        return NextStep.act(adeprocess, aaction, aaction_data);
    }
}
