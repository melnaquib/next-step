pragma solidity >=0.8.0;

/**
 * @title Pallet NextStep Interface
 *
 * The interface through which solidity contracts will interact with pallet-next-step.mapping
 * Address :    0x0000000000000000000000000000000000000807
 */

interface NextStep {
    /**
     * Complete task
     * task_id: aa5ac585
     *
     * @param action The taskId of task to be marked complete
     * @param data free form string with information about task, which can be an empty one.
     */
    function act(bytes32 deprocess, string action, string calldata data) external;

    /**
     * Clear existing association
     * Selector: 7354c91d
     *
     * @param nimbus_id The nimbusId to be cleared
     */
    function act(bytes32 deprocess, string action) external;

    /**
     * @dev Returns whether task is done
     * deprocess: 0x3644e515
     */
    function is_ended(bytes32 deprocess) external view returns (bool);

    /**
     * @dev Returns data entered for task when completed
     * task_id: 0x3644e515
     */
    function data(bytes32 task_id) external view returns (memory string);

    /**
     * @dev Returns account assigned to task
     * task_id: 0x3644e515
     */
    function actor(bytes32 task_id) external view returns (address);
}
