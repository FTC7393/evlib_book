CalibratedLineSensor takes an [[analog|Analog Sensors]] line sensor and turns it into a [[DigitalSensor|Digital Sensors]] by finding a threshold between light and dark. The getValue() method will return true if the sensor sees a line. Note: the act method must be called every loop to update the sensor values for averaging. This class is used in [[DoubleLineSensor]].

[ftc/evlib/hardware/sensors/CalibratedLineSensor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/CalibratedLineSensor.java)

```java
package ftc.evlib.hardware.sensors;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 3/8/16
 * 
 * manages threshold calculation for the line sensor
 *
 * @see AnalogSensor
 */
public class CalibratedLineSensor implements DigitalSensor {
    private boolean ready = true;
    private final AnalogSensor lineSensor;
    private static final int NUM_CALIBRATIONS = 100;
    private int numReadings;
    private double average, deviation, threshold = 80;
    private final double readings[] = new double[NUM_CALIBRATIONS];
    private boolean seeingLine = false;
    private double value;

    /**
     * @param lineSensor the raw line sensor
     */
    public CalibratedLineSensor(AnalogSensor lineSensor) {
        this.lineSensor = lineSensor;
    }

    /**
     * start calibrating the sensor
     */
    public void calibrate() {
        ready = false;
        average = 0;
        numReadings = 0;
        deviation = 0;
    }

    /**
     * @return true when done calibrating
     */
    public boolean isReady() {
        return ready;
    }

    /**
     * @return true if the sensor is over the line
     */
    @Override
    public Boolean getValue() {
        return seeingLine;
    }

    /**
     * read the sensor value and use it to calibrate/update the average
     */
    public void act() {
        value = lineSensor.getValue();
        if (!ready) {
            if (numReadings < NUM_CALIBRATIONS) {
                //add a value
                readings[numReadings] = value;
                numReadings++;

                //update the average
                average = (average * (numReadings - 1) + value) / (numReadings);

            } else {
                //find deviation
                for (double reading : readings) {
                    deviation += (reading - average) * (reading - average);
                }
                deviation /= Math.sqrt(numReadings);

                threshold = (average - deviation) * (average - deviation) / average;
                ready = true;
            }
        }
        seeingLine = value < threshold;
    }

    /**
     * @return the raw value of the line sensor
     */
    public double getRawValue() {
        return value;
    }
}
```