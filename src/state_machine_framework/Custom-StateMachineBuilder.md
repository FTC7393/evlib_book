Make a class that extends [[StateMachineBuilder]] and calls the superconstructor.
```java
public class MyStateMachineBuilder extends StateMachineBuilder {
    public MyStateMachineBuilder(StateName firstStateName) {
        super(firstStateName);
    }
}

```

Then add a convenience method to it that uses your method you made in [[MyStates|Custom States]].
```java
public class MyStateMachineBuilder extends StateMachineBuilder {
    public MyStateMachineBuilder(StateName firstStateName) {
        super(firstStateName);
    }
    
    public void addCalibrateGyro(StateName stateName, GyroSensor gyro, StateName nextStateName){
        add(stateName, MyStates.calibrateGyro(gyro, nextStateName));
    }
    
    public void addTelem(StateName stateName, String message, double value, Map<StateName, EndCondition> transitions) {
        add(stateName, EVStates.telemetry(transitions, message, value));
    }
}

```

You can even pass in objects in the constructor to be used multiple times, such as the gyro sensor or telemetry.
```java
public class MyStateMachineBuilder extends StateMachineBuilder {
    private final Telemetry telemetry;
    
    public MyStateMachineBuilder(StateName firstStateName, Telemetry telemetry) {
        super(firstStateName);
        this.telemetry = telemetry;
    }
    
    public void addCalibrateGyro(StateName stateName, GyroSensor gyro, StateName nextStateName){
        add(stateName, MyStates.calibrateGyro(gyro, nextStateName));
    }
    
    public void addTelem(StateName stateName, String message, double value, Map<StateName, EndCondition> transitions) {
        add(stateName, EVStates.telemetry(transitions, message, value));
    }
}

```

The next step is to create [[Custom EndConditions]].