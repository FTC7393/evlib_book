AveragedSensor takes an [AnalogSensor](Analog-Sensors.md) and averages the last N values to get the current value. The act() method should be called exactly once every time through the loop, but getValue() can be called whenever you need the value. The isReady() method will return true if the sensor is done collecting the first N values. If the sensor is not ready, the value will not be accurate.

[ftc/evlib/hardware/sensors/AveragedSensor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/AveragedSensor.java)
```java
package ftc.evlib.hardware.sensors;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 3/16/16
 * 
 * report the average of the last N values of a sensor as the value
 *
 * @see AnalogSensor
 */
public class AveragedSensor implements AnalogSensor {
    private final AnalogSensor sensor;
    private final double readings[];
    private int index = 0;
    private boolean ready = false;
    private final int numReadings;
    private double average = 0;

    /**
     * @param sensor      the sensor to average
     * @param numReadings the number of readings to average
     */
    public AveragedSensor(AnalogSensor sensor, int numReadings) {
        this.sensor = sensor;
        this.numReadings = numReadings;
        readings = new double[numReadings];
    }

    /**
     * @return true once the first numReadings have been read
     */
    public boolean isReady() {
        return ready;
    }

    @Override
    public Double getValue() {
        return average;
    }

    /**
     * read the sensor and update the average
     */
    public void act() {
        double reading = sensor.getValue();
        readings[index] = reading;
        index++;
        if (index >= numReadings) {
            index = 0;
            ready = true;
        }
        if (ready) {
            average = 0;
            for (int i = 0; i < numReadings; i++) {
                average += readings[i];
            }
            average /= numReadings;
        } else {
            average = reading;
        }
    }
}
```