ProportionalController implements [[ControlLoop]]. It provides a curve that is shaped like the following graph (parameters `new ProportionalController(0.2, 0.05, 0.1, 0.5)`)

![graph](https://github.com/FTC7393/state-machine-framework/blob/master/images/PropGraph.png?raw=true)

The parameters are as follows:
* gain -- the slope of the line
* deadzone -- the portion of the line where y=0
* minOutput -- the height of the small flat parts on either side of the deadzone
* maxOutput -- the height of the large flat parts on the edges of the graph

[ftc/electronvolts/util/ProportionalController.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/ProportionalController.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A proportional controller to use for controlling motors and other outputs
 */
public class ProportionalController implements ControlLoop {
    private final double gain, maxOutput, deadzone, minOutput;

    public ProportionalController(double gain, double deadzone, double minOutput, double maxOutput) {
        if (gain < 0) {
            throw new IllegalArgumentException("Illegal gain constant \"" + gain + "\". constants cannot be negative.");
        }
        if (maxOutput < 0) {
            throw new IllegalArgumentException("Illegal maxOutput constant \"" + maxOutput + "\". constants cannot be negative.");
        }
        if (deadzone < 0) {
            throw new IllegalArgumentException("Illegal deadzone constant \"" + deadzone + "\". constants cannot be negative.");
        }
        if (minOutput < 0) {
            throw new IllegalArgumentException("Illegal minOutput constant \"" + minOutput + "\". constants cannot be negative.");
        }
        if (minOutput > maxOutput) {
            throw new IllegalArgumentException("minOutput (" + minOutput + ") cannot be grater than maxOutput (" + maxOutput + ")");
        }
        if (deadzone > minOutput) {
            throw new IllegalArgumentException("deadzone (" + deadzone + ") cannot be grater than minOutput (" + minOutput + ")");
        }

        this.gain = gain;
        this.maxOutput = maxOutput;
        this.deadzone = deadzone;
        this.minOutput = minOutput;
    }

    @Override
    public double computeCorrection(double setPoint, double input) {
        double correction = gain * (setPoint - input);

        if (Math.abs(correction) <= deadzone) {
            //return 0 if the error is in the deadzone
            return 0;
        } else if (Math.abs(correction) < minOutput) {
            //return the minimum if it is below
            return Math.signum(correction) * minOutput;
        } else if (Math.abs(correction) > maxOutput) {
            //cap the correction at +/- maxOutput
            return Math.signum(correction) * maxOutput;
        } else {
            return correction;
        }
    }

    @Override
    public void initialize() {
        //not needed for a proportional controller since there are no persistent state variables
    }

}
```