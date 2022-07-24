Currently in the autonomous we have the following:

```java
        //define the DRIVE state to drive for 2 feet and move to the STOP state
        b.add(EVStates.drive(S.DRIVE, S.STOP, Distance.fromFeet(2), robotCfg.getTwoMotors(), 0.5));
```

It would be nice if we didn't have to specify the robot's max speed and the motors to use for each drive state.
The solution for that is to make a class that extends StateMachineBuilder. This custom builder will store the parameters that appear commonly in the drive state and add a convenience method with fewer arguments.

The line above could be changed to:

```java
        //more readable and less tedious to type
        b.addDrive(S.DRIVE, S.STOP, Distance.fromFeet(2), 0.5);
```

Here is the subclass to StateMachineBuilder with the convenience method:

[/sample/v2/SampleStateMachineBuilder.java](https://github.com/FTC7393/EVLib/blob/master/sample/v2/SampleStateMachineBuilder.java)
```java
package org.firstinspires.ftc.teamcode.sample.v2;

import ftc.electronvolts.statemachine.StateMachineBuilder;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.units.Distance;
import ftc.evlib.hardware.motors.TwoMotors;
import ftc.evlib.statemachine.EVStates;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample extension of the StateMachineBuilder.
 * This subclass adds one convenience method for autonomous opmodes.
 */

public class SampleStateMachineBuilder extends StateMachineBuilder {
    /**
     * The drive motors
     */
    private final TwoMotors twoMotors;

    /**
     * Create a SampleStateMachineBuilder, passing it the drive motors
     * The drive motors will be passed to the drive state every time
     *
     * @param firstStateName the state to start with
     * @param twoMotors      the robot's drive motors
     */
    public SampleStateMachineBuilder(StateName firstStateName, TwoMotors twoMotors) {
        super(firstStateName);
        this.twoMotors = twoMotors;
    }

    /**
     * convenience method for adding a drive state
     *
     * @param stateName     the name of the state
     * @param nextStateName the name of the state to go to after the drive is complete
     * @param distance      the distance to drive
     * @param velocity      the velocity to drive at
     */
    public void addDrive(StateName stateName, StateName nextStateName, Distance distance, double velocity) {
        //add the drive state with the motors and speed from sampleRobotCfg
        add(EVStates.drive(stateName, nextStateName, distance, twoMotors, velocity));
    }
}
```

We then have to change our creation of the StateMachineBuilder to create this new subclass instead, and then we can simplify the drive state.

Here is the autonomous with both changes:

[/sample/v2/SampleAuto.java](https://github.com/FTC7393/EVLib/blob/master/sample/v2/SampleAuto.java)
```java
package org.firstinspires.ftc.teamcode.sample.v2;

import com.google.common.collect.ImmutableList;
import com.qualcomm.robotcore.eventloop.opmode.Autonomous;

import ftc.electronvolts.statemachine.StateMachine;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.InputExtractor;
import ftc.electronvolts.util.files.Logger;
import ftc.electronvolts.util.units.Distance;
import ftc.evlib.opmodes.AbstractAutoOp;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample autonomous that drives forward for 2 feet.
 * This one uses a custom builder
 */

@Autonomous(name = "SampleAuto V2")
public class SampleAuto extends AbstractAutoOp<SampleRobotCfg> {

    /**
     * defines all the possible states for the state machine
     * modify this to have whatever states you want
     */
    private enum S implements StateName {
        DRIVE,
        STOP
    }

    @Override
    public StateMachine buildStates() {
        //create a new builder for the states, starting with the DRIVE state
        //this time we are using the custom builder and passing it the drive motors from the robotCfg
        SampleStateMachineBuilder b = new SampleStateMachineBuilder(S.DRIVE, robotCfg.getTwoMotors());

        //define the DRIVE state to drive for 2 feet and move to the STOP state

        //the old code without the custom builder:
//        b.add(EVStates.drive(S.DRIVE, S.STOP, Distance.fromFeet(2), robotCfg.getTwoMotors(), 0.5));

        //the new code (more readable and less tedious to type):
        b.addDrive(S.DRIVE, S.STOP, Distance.fromFeet(2), 0.5);

        //define the STOP state to be empty (and never exit) so the state machine will stop
        b.addStop(S.STOP);

        //build and return the StateMachine
        return b.build();
    }

    @Override
    protected SampleRobotCfg createRobotCfg() {
        return new SampleRobotCfg(hardwareMap);
    }

    @Override
    protected Logger createLogger() {
        //it will save logs in /FTC/logs/autonomous[....].csv on the robot controller phone
        return new Logger("autonomous", ".csv", ImmutableList.of(
                //each one of these is a column in the log file
                new Logger.Column("state", new InputExtractor<StateName>() {
                    @Override
                    public StateName getValue() {
                        return stateMachine.getCurrentStateName();
                    }
                }),
                new Logger.Column("leftMotor power", new InputExtractor<Double>() {
                    @Override
                    public Double getValue() {
                        return robotCfg.getTwoMotors().getValue(0);
                    }
                }),
                new Logger.Column("rightMotor power", new InputExtractor<Double>() {
                    @Override
                    public Double getValue() {
                        return robotCfg.getTwoMotors().getValue(1);
                    }
                })
        ));
    }

    @Override
    protected void setup_act() {

    }

    @Override
    protected void go() {

    }

    @Override
    protected void act() {

    }

    @Override
    protected void end() {

    }
}
```

I might not seem like a big improvement, but it makes a huge difference when you have upwards of 20 drive states you are trying to debug. It can also be applied to states that take more parameters, such as a gyro turn which needs [TwoMotors](NMotors.md), a GyroSensor, and a rotation speed (which could then be the same for all gyro turns).

The next step is to use [[Servo Presets]] to add some features to the servos.