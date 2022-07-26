First, create a class that extends the [States](States.md) factory class. It will inherit all the methods from [States](States.md).
```java
public class MyStates extends States {

}
```

You can add methods to it such as a gyro calibration state or a telemetry state:
```java
public class MyStates extends States {
    public static State calibrateGyro(StateName stateName, final GyroSensor gyro, final StateName nextStateName){
        gyro.calibrate();
        return new BasicAbstractState(stateName) {
            @Override
            public void init() {}

            @Override
            public boolean isDone() {
                return !gyro.isCalibrating();
            }

            @Override
            public StateName getNextStateName() {
                return nextStateName;
            }
        };
    }
    
    
    public static State telemetry(Map<StateName, EndCondition> transitions, final String message, final double value) {
        return new AbstractState(transitions) {
            @Override
            public void init() {
            }

            @Override
            public void run() {
                telemetry.addData(message, value);
            }

            @Override
            public void dispose() {
            }
        };
    }
```

The next step is to create a [Custom StateMachineBuilder](Custom-StateMachineBuilder.md).