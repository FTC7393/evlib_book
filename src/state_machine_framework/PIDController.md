PIDController implements [ControlLoop](ControlLoop.md). It computes the correction on an output based on an input and 4 constants: P, I, D, and maximum output. These 4 constants are given when PIDController is created.

The main function is computeCorrection. It takes the target position (setPoint) and the measured actual position (input) as parameters. It finds the delta time (timeChange), and from that calculates the change in the integral-term (iTerm) and the change in input per time (dInput). Then it calculates output:
```
output = pGain * error + iTerm - dGain * dInput;
```
* `pGain * error` is the **P**roportional term
* `iTerm` is the **I**ntegral term that is accumulated over time
* `dGain * dInput` is the **D**erivative term that is calculated from the delta input

The `initialize()` function should be called when the controller has been inactive for a while.

[ftc/electronvolts/util/PIDController.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/PIDController.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A PID controller to use for controlling motors and other outputs
 */
public class PIDController implements ControlLoop {
    private final double pGain, iGain, dGain, maxOutput;
    private final boolean hasIOrDComponent;
    private double iTerm = 0, output = 0;
    private double input = 0, lastInput = 0;
    private long lastTime = -1;

    /**
     * create a new PID controller
     *
     * @param pGain p constant (cannot be negative)
     * @param iGain i constant (cannot be negative)
     * @param dGain d constant (cannot be negative)
     * @param maxOutput the max value of the output and iTerm (cannot be
     *            negative)
     */
    public PIDController(double pGain, double iGain, double dGain, double maxOutput) {
        if (pGain < 0) {
            throw new IllegalArgumentException("Illegal pGain constant \"" + pGain + "\". PID constants cannot be negative.");
        }
        if (iGain < 0) {
            throw new IllegalArgumentException("Illegal iGain constant \"" + iGain + "\". PID constants cannot be negative.");
        }
        if (dGain < 0) {
            throw new IllegalArgumentException("Illegal dGain constant \"" + dGain + "\". PID constants cannot be negative.");
        }
        if (maxOutput < 0) {
            throw new IllegalArgumentException("Illegal maxOutput constant \"" + maxOutput + "\". PID constants cannot be negative.");
        }

        this.pGain = pGain;
        this.iGain = iGain;
        this.dGain = dGain;
        this.maxOutput = maxOutput;

        hasIOrDComponent = (iGain != 0 || dGain != 0);
    }

    /**
     * @param setPoint the target value
     * @param input the actual value
     * @return the output of the PID
     */
    @Override
    public double computeCorrection(double setPoint, double input) {
        long now = System.currentTimeMillis();
        if (lastTime < 0) {
            lastTime = now;
        }

        //time passed since last cycle
        double dTime = now - lastTime;
        if (dTime > 0 || !hasIOrDComponent) {
            this.input = input;

            // Compute all the working error variables
            double error = setPoint - input;
            iTerm += iGain * error * dTime;
            iTerm = Utility.mirrorLimit(iTerm, maxOutput);

            // compute dInput instead of dError to avoid spikes
            double dInput = 0;
            if (dTime > 0) dInput = (input - lastInput) / dTime;

            // Compute PID Output
            output = pGain * error + iTerm - dGain * dInput;
            output = Utility.mirrorLimit(output, maxOutput);

            // Remember some variables for next time
            lastInput = input;
            lastTime = now;
        }

        return output;
    }

    /**
     * Reset the PIDController to its initial state by resetting the iTerm and
     * the lastTime
     */
    @Override
    public void initialize() {
        lastInput = input;
        iTerm = 0;
        lastTime = -1;
        if (hasIOrDComponent) {
            output = 0;
        }
    }
}
```