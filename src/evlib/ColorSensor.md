ColorSensor uses a red LED, a blue LED, and a photoresistor to detect the color of a line on the mat. It has 4 methods for setting the color: red, blue, magenta, and off. It implements [DigitalSensor](Digital-Sensors.md), returning whether or not it is seeing the color specified. It is used in [LineFinder](LineFinder.md) along with [DoubleLineSensor](DoubleLineSensor.md) to more accurately find colored lines.

Fun fact: last year when connecting LEDs, we guessed on the resistor values and it burnt out 2 of our digital outputs. Always calculate the current before you plug LEDs in to the Core Device Interface Module.

[ftc/evlib/hardware/sensors/ColorSensor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/ColorSensor.java)

```java
package ftc.evlib.hardware.sensors;

import com.qualcomm.robotcore.hardware.LED;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 3/22/16
 * 
 * Uses LEDs and a photoresistor to detect the color of a line on the mat
 * 
 * --- light red blue
 * mat    35 250 250
 * red
 * blue
 * white 145 640 550
 *
 * @see DigitalSensor
 */
public class ColorSensor implements DigitalSensor {
    private static final int THRESHOLD = 500;
    private final LED redLight, blueLight;
    private final AveragedSensor lightSensor;
    private String ledColorString = "unknown";

    /**
     * @param redLight    the red LED
     * @param blueLight   the blue LED
     * @param lightSensor the photoresistor
     */
    public ColorSensor(LED redLight, LED blueLight, AveragedSensor lightSensor) {
        this.redLight = redLight;
        this.blueLight = blueLight;
        this.lightSensor = lightSensor;
    }

    /**
     * turn off both LEDs
     */
    public void turnOff() {
        redLight.enable(true);
        blueLight.enable(true);
        ledColorString = "blank";
    }

    /**
     * turn on the red LED and turn off the blue LED
     */
    public void setColorRed() {
        redLight.enable(false);
        blueLight.enable(true);
        ledColorString = "red";
    }

    /**
     * turn on the blue LED and turn off the red LED
     */
    public void setColorBlue() {
        redLight.enable(true);
        blueLight.enable(false);
        ledColorString = "blue";
    }

    /**
     * turn on both LEDs
     */
    public void setColorMagenta() {
        redLight.enable(false);
        blueLight.enable(false);
        ledColorString = "magenta";
    }

    /**
     * @return the name of the current color for telemetry purposes
     */
    public String getLedColorString() {
        return ledColorString;
    }

    /**
     * update the light sensor
     */
    public void act() {
        lightSensor.act();
    }

    /**
     * @return true if it is seeing the color requested
     */
    @Override
    public Boolean getValue() {
        return lightSensor.getValue() > THRESHOLD;
    }

    /**
     * @return the raw value of the light sensor
     */
    public double getRawValue() {
        return lightSensor.getValue();
    }
}
```