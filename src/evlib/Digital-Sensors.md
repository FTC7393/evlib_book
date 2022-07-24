EVLib has its own interface for Digital sensors. It extends InputExtractor of type Boolean, which means that every DigitalSensor has to have a getValue method that returns a Boolean.

[ftc/evlib/hardware/sensors/DigitalSensor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/DigitalSensor.java)
```java
package ftc.evlib.hardware.sensors;

import ftc.electronvolts.util.InputExtractor;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/12/16
 * 
 * Interface for any type of digital sensor
 * examples: touch sensor, digital line sensor, magnetic reed switch
 *
 * @see InputExtractor
 * @see Sensors
 */
public interface DigitalSensor extends InputExtractor<Boolean> {
}
```

You can create a DigitalSensor from a DigitalChannel or TouchSensor using the [Sensors](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/Sensors.java) factory class:

```java
DigitalInput digitalInput = hardwareMap.touchSensor.get("limitSwitch");
DigitalSensor limitSwitch = Sensors.digitalSensor(digitalInput);

//To combine on one line:
DigitalSensor button = Sensors.digitalSensor(hardwareMap, "button");
```

To get the value of a DigitalSensor, call the getValue() method which returns a Boolean.

```java
teletry.addData("limit switch pressed?", limitSwitch.getValue());
```

See also: [Analog Sensors](Analog-Sensors.md)