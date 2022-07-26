StateTimer is a simple timer that can be used in a simple state machine. It is not used in the current state machine framework, but it is here in case anyone needs it for something. For a timer that is used in the framework, see [EndConditions](EndConditions.md).

[ftc/electronvolts/util/StateTimer.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/StateTimer.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A timer that can be used in a simplistic state machine
 */
public class StateTimer {
    private long endTime;

    /**
     * Start the timer
     *
     * @param durationMillis how long the timer will run in milliseconds
     */
    public void init(long durationMillis) {
        this.endTime = durationMillis + System.currentTimeMillis();
    }

    /**
     * @return whether or not the time has elapsed yet
     */
    public boolean isDone() {
        return System.currentTimeMillis() >= endTime;
    }
}
```