Note: This page is old and is no longer accurate. See the [[Simple State Machine]] page for a more up-to-date explanation.

## OpModes: The top level of our state machine design
The following is a simple OpMode that demonstrates the use of the state machine builder. This OpMode is very simple; our actual autonomous program is about 10 times longer.
```java
public class OffenseOp extends OpMode {

   private MecanumRobot robot;
   private MatchTimer matchTimer;
   private StateMachine stateMachine;
   private Integer adjustableWaitMillis;

   public enum s implements StateName {
       INIT_SERVOS,
       CALIBRATE_GYRO,
       MATCH_WAIT,
       ADJUSTABLE_WAIT,
       DRIVE_FORWARD,
       STOP
   }


   @Override
   public void init() {
       final Hardware hardware = Hardware.createInstance(hardwareMap, telemetry, Hardware.defaultServoCfgMap());

       matchTimer = Hardware.getInstance().autoMatchTimer();
       robot = hardware.createMecanumRobot();
       MecanumControl mecanumControl = new MecanumControl(robot, hardware.getGyro(), hardware.getDoubleLineSensor());

       OptionsFile optionsFile = OptionsFile.fromFile("AutoOptions.txt");

       adjustableWaitMillis = optionsFile.getAsInteger("waitTimeMillis");
       if(adjustableWaitMillis == null){
           adjustableWaitMillis = 0;
       }

       StateMachineBuilder b = new StateMachineBuilder(s.INIT_SERVOS, robot, mecanumControl);
       b.addServoInit(s.INIT_SERVOS, s.CALIBRATE_GYRO);
       b.addCalibrateGyro(s.CALIBRATE_GYRO, hardware.getGyro(), s.ADJUSTABLE_WAIT);
       b.addWait(s.ADJUSTABLE_WAIT, matchTimer, adjustableWaitMillis, s.MATCH_WAIT);
       b.addWait(s.MATCH_WAIT, matchTimer, 10000, s.DRIVE_FORWARD); //make sure that 10 seconds have passed
       b.addDrive(s.DRIVE_FORWARD, 8000, s.STOP, 1, 0, 0);
       b.add(s.STOP, States.stop(robot));

       stateMachine = b.build();
   }

   public void init_loop() { }

   public void start() {
       matchTimer.start();
   }

   @Override
   public void loop() {
       Hardware.getInstance().getTelem().addData("wait time", adjustableWaitMillis);

       telemetry.addData("opticalDist (side)", Hardware.getInstance().getOpticalSensor().getValue());

       matchTimer.update();

       if (matchTimer.isMatchJustOver()) stop();
       if (matchTimer.isMatchOver()) return;

       telemetry.addData("Time left", matchTimer.getTimeLeft()/1000.0);

       stateMachine.act();
       Hardware.getInstance().servosAct();
       //allow the line sensor to calibrate if it needs to
       Hardware.getInstance().getDoubleLineSensor().act();
       Hardware.getInstance().getOpticalSensor().act();

       Hardware.getInstance().storeServoPositions("servoPositions.txt");
   }

   public void stop() {
       robot.stopMotors();
   }
}
```
The OpMode is the top level of the state machine design. The OpMode must define the names of the states, then define their behavior and end conditions and link them together, and finally build and start running the state machine.

The first step is initializing the names of all of the independent states that make up the autonomous program.
```java
public enum s implements StateName {
   INIT_SERVOS,
   CALIBRATE_GYRO,
   MATCH_WAIT,
   ADJUSTABLE_WAIT,
   DRIVE_FORWARD,
   STOP
}
```

The next step is to create the StateMachineBuilder, giving it direct access to the robot for use in the convenience methods.
```java
public void init() {
       StateMachineBuilder b = new StateMachineBuilder(s.INIT_SERVOS, robot, mecanumControl);
```
Then comes the main part of autonomous: setting up the actual states. The builder minimizes the amount of typing you have to do for each state. For example, to make a drive state, you call addDrive and give it the state name, the time to drive (milliseconds), the state to transition to next, and the drive parameters (velocity, direction, and orientation).

```java
b.addServoInit(s.INIT_SERVOS, s.CALIBRATE_GYRO);
b.addCalibrateGyro(s.CALIBRATE_GYRO, hardware.getGyro(), s.ADJUSTABLE_WAIT);
b.addWait(s.ADJUSTABLE_WAIT, matchTimer, adjustableWaitMillis, s.MATCH_WAIT);
b.addWait(s.MATCH_WAIT, matchTimer, 10000, s.DRIVE_FORWARD); //make sure that 10 seconds have passed
b.addDrive(s.DRIVE_FORWARD, 8000, s.STOP, 1, 0, 0);
b.add(s.STOP, States.stop(robot));
```
When you are done adding states, you call the build method to get a StateMachine object. During the loop method, you call stateMachine.act() to run the states in the defined order.
```java
       stateMachine = b.build();
}
public void loop() {
   stateMachine.act();
}
```
This state machine setup allows for branching and looping of states.