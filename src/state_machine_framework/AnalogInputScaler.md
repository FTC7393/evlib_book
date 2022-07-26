AnalogInputScaler takes an [InputExtractor](InputExtractor.md) of type Double, and a [Function](Functions.md) to scale the input with.

* The update() method gets the value from the [InputExtractor](InputExtractor.md) and feeds it through the [Function](Functions.md)
* The getValue() method returns the scaled value as it was computed by the last call to update()
* The getRawValue() method returns the raw value as it was retrieved by the last call to update()

AnalogInputScaler also implements [InputExtractor](InputExtractor.md)\<Double\>, which means it can be passed in to a [Logging](Logger.md) function, for example.

See also: [DigitalInputEdgeDetector](DigitalInputEdgeDetector.md)

[ftc/electronvolts/util/AnalogInputScaler.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/AnalogInputScaler.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * Manages the scaling of an analog input such as a joystick
 */
public class AnalogInputScaler implements InputExtractor<Double> {
    private double rawValue = 0;
    private double value = 0;

    private final InputExtractor<Double> extractor;
    private final Function inputScaler;

    /**
     * @param extractor   the input extractor
     * @param inputScaler the input scaler to use for scaling the input
     */
    public AnalogInputScaler(InputExtractor<Double> extractor, Function inputScaler) {
        this.extractor = extractor;
        this.inputScaler = inputScaler;
    }

    /**
     * updates the output value and raw value
     *
     * @return the scaled value
     */
    public double update() {
    	rawValue = extractor.getValue();
        value = inputScaler.f(rawValue);
        return value;
    }

    /**
     * @return the value
     */
    @Override
    public Double getValue() {
        return value;
    }

    /**
     * @return the raw value of the input
     */
    public double getRawValue() {
        return rawValue;
    }
}
```