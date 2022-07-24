ControlLoop defines an interface for control loops such as a [[PID controller|PIDController]] or a [[proportional controller|ProportionalController]]. The computeCorrection function takes a target value, or setPoint, and an actual value, or input. It computes the power necessary for a motor or other output to bring the input closer to the setPoint.

The initialize() function is meant to reset the persistent (stored between loop cycles) variables. It should be called when the controller has been off for a while.

[ftc/electronvolts/util/ControlLoop.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/ControlLoop.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * An interface that allows for any type of control loop to be used
 * @see PIDController
 */
public interface ControlLoop {
    double computeCorrection(double setPoint, double input);
    void initialize();
}
```