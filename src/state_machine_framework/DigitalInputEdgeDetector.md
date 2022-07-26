DigitalInputEdgeDetector takes an [InputExtractor](InputExtractor.md) of type Boolean. It detects changes in state between calls of the update() method.

* The justPresssed() method returns true if the input has transitioned from false to true between the last two calls of update().
* The justReleased() method returns true if the input has transitioned from true to false between the last two calls of update().
* The isPressed() method returns the value of the input at the last call of update(), regardless of any transitions.

DigitalInputEdgeDetector also implements [InputExtractor](InputExtractor.md)\<Boolean\> and returns the value of the input in the getValue() method.

See also: [AnalogInputScaler](AnalogInputScaler.md)

[ftc/electronvolts/util/DigitalInputEdgeDetector.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/DigitalInputEdgeDetector.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A class that is very useful for joystick control.
 * It does edge detection for a digital input.
 */
public class DigitalInputEdgeDetector implements InputExtractor<Boolean> {
    private Boolean currentValue = null;
    private Boolean previousValue = null;
    private InputExtractor<Boolean> extractor;

    /**
     * @param extractor the InputExtractor to do edge detection on
     */
    public DigitalInputEdgeDetector(InputExtractor<Boolean> extractor) {
        this.extractor = extractor;
    }

    /**
     * update the current and previous value of the input
     *
     * @return the current value of the input
     */
    public boolean update() {
        // if this is the first call to update()
        if (currentValue == null) {
            // set currentValue and previousValue to the reading so no edges are
            // triggered
            currentValue = extractor.getValue();
            previousValue = currentValue;
        } else {
            previousValue = currentValue;
            currentValue = extractor.getValue();
        }
        return currentValue;
    }

    /**
     * @return whether or not the input is true right now
     */
    @Override
    public Boolean getValue() {
        return currentValue;
    }

    /**
     * @return whether or not the input is true right now
     */
    public boolean isPressed() {
        return currentValue;
    }

    /**
     * @return if the input just turned from false to true
     */
    public boolean justPressed() {
        return currentValue && !previousValue;
    }

    /**
     * @return if the input just turned from true to false
     */
    public boolean justReleased() {
        return !currentValue && previousValue;
    }
}
```