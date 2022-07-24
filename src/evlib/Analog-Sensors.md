EVLib has its own interface for Analog sensors. It extends [InputExtractor](https://github.com/FTC7393/state-machine-framework/wiki/InputExtractor) (from the state-machine-framework) of type Double, which means that every AnalogSensor has to have a method that returns a Double: `Double getValue()`.

[ftc/evlib/hardware/sensors/AnalogSensor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/AnalogSensor.java)
```java
package ftc.evlib.hardware.sensors;

import ftc.electronvolts.util.InputExtractor;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/11/16
 * 
 * Interface for any type of analog sensor
 * examples: light sensor, distance sensor, potentiometer
 *
 * @see InputExtractor
 * @see Sensors
 */
public interface AnalogSensor extends InputExtractor<Double> {
}
```

To create an AnalogSensor from an AnalogInput (which is part of the FTC libraries), you can use the [ftc/evlib/hardware/sensors/Sensors.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/sensors/Sensors.java) factory class:

```java
AnalogInput analogInput = hardwareMap.analogInput.get("lightSensor");
AnalogSensor lightSensor = Sensors.analog(analogInput);

//To combine on one line:
AnalogSensor lightSensor = Sensors.analog(hardwareMap, "lightSensor");
```

To get the value of a sensor, call the getValue() method:

```java
telemetry.addData("light sensor", lightSensor.getValue());
```

See also: [DistanceSensor](DistanceSensor.md), [Digital Sensors](Digital-Sensors.md)