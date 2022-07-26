We have established that all states need an "act" method, but what if the state needs a method that runs once before the main loop starts or a method that runs after the state ends? And what about moving from one state to the next?

AbstractState is a more complex (than [BasicAbstractState](BasicAbstractState.md)) implementation of the [State](State.md) interface that separates the State's functionality from the transitions to other states. It separates the act() method into three methods: init(), run(), and dispose().

* The init() method is called the first time the state is run. You can reset values, turn on motors, etc.
* The run() method is called every loop after init(). You can update motor powers, display values on telemetry, etc.
* The dispose() method is called after the state is done. You can turn off motors, close files, etc.

The way AbstractState handles the transitions to other states is by using [StateMap](StateMap.md). When it is created, a map of StateName to EndCondition is passed in.

You can see examples of AbstractState (and [BasicAbstractState](BasicAbstractState.md)) in the [States](States.md) factory class.

[ftc/electronvolts/statemachine/AbstractState.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/statemachine/AbstractState.java)
```java
package ftc.electronvolts.statemachine;

import java.util.Map;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * AbstractState is a simple state that handles transitions.
 */
public abstract class AbstractState implements State {
    // Map of possible transitions to other states
    private final Map<StateName, EndCondition> transitions;

    private boolean isRunning = false;

    /**
     * An abstract state must contain a map of transitions, containing end
     * conditions and their respective next state names.
     *
     * @param transitions the map of EndCondition to NextState
     * @see Transition
     */
    public AbstractState(Map<StateName, EndCondition> transitions) {
        this.transitions = transitions;
    }


    /**
     * Implementation of the act() method in the State interface
     * Cycles through Transitions to run one of the next states if its
     * EndCondition has been met. Does "edge detection" on the state, running
     * init() for the first cycle, run() for subsequent cycles, and dispose()
     * for the last cycle.
     *
     * @return The name of the state to be run next cycle (returns itself if
     *         there are no state changes)
     */
    @Override
    public StateName act() {
        if (!isRunning) {
            init();
            isRunning = true;
            for(Map.Entry<StateName, EndCondition> entry : transitions.entrySet()) {
                EndCondition endCondition = entry.getValue();
                endCondition.init();
            }
        }
        for(Map.Entry<StateName, EndCondition> entry : transitions.entrySet()) {
            StateName stateName = entry.getKey();
            EndCondition endCondition = entry.getValue();
            if (endCondition.isDone()) {
                dispose();
                isRunning = false;
                return stateName;
            }
        }
        run();
        return null;
    }

    /**
     * Run once when the state is initialized.
     * This can run tasks such as starting motors.
     */
    public abstract void init();

    /**
     * Run every cycle after init()
     * This can do tasks such as gyro stabilization or line following
     */
    public abstract void run();

    /**
     * Run once when the state is finished before the next state is initialized.
     * This can do any cleaning up, such as stopping any started motors.
     */
    public abstract void dispose();
}
```