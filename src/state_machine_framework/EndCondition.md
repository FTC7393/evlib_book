This is an interface used to tell when a state is done. Every implementation of it must have an “init” method, where sensors or motors can be reset, and values can be initialized. It also must have an “isDone” method, where it can read the sensors, motors, etc., and report back to tell if the condition has been met. EndCondition objects are created in the [EndConditions](EndConditions.md) factory class, and used in the [AbstractState](AbstractState.md) class with a [StateMap](StateMap.md).

[ftc/electronvolts/statemachine/EndCondition.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/statemachine/EndCondition.java)

```java
package ftc.electronvolts.statemachine;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * An EndCondition is used as a condition to tell when something is done. If the
 * end condition is true, the next state is run.
 */
public interface EndCondition {
    /**
     * Run once when the end condition is created Can be used to tell helper
     * classes to initialize Be careful to reset all variables here in case the
     * state is run a second time
     */
    void init();

    /**
     * Run every cycle after init() Used to get values from helper classes to
     * see if the condition is met
     *
     * @return true if condition is met.
     */
    boolean isDone();
}
```