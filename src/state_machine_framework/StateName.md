The StateName interface is meant to be implemented by an enum which contains the names of all the states you want to set up.

[ftc/electronvolts/statemachine/StateName.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/statemachine/StateName.java)
```java
package ftc.electronvolts.statemachine;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * This class is meant to be implemented with an enum containing the list of
 * state names.
 */
public interface StateName {
    /**
     * @return Returns the name for telemetry purposes.
     */
    String name();
}
```