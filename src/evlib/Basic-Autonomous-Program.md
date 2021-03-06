Before you can make an autonomous program, add the following field to your RobotCfg:

```java
    //the speed of the robot at 100% power
    private static final Velocity MAX_SPEED = new Velocity(Distance.fromInches(50), Time.fromSeconds(5));
```

This represents your robot's speed at 100% power. You can calculate it or determine it by measuring the distance your robot drives in a certain amount of time. The Distance and Time classes support a variety of units (you could _technically_ express you robot's speed in nautical miles per week).

The following is an Autonomous program that drives forward for 2 feet (using the speed defined in the robot config):

[/sample/v1/SampleAuto.java](https://github.com/FTC7393/EVLib/blob/master/sample/v1/SampleAuto.java)
```java
package org.firstinspires.ftc.teamcode.sample.v1;

import com.qualcomm.robotcore.eventloop.opmode.Autonomous;

import ftc.electronvolts.statemachine.StateMachine;
import ftc.electronvolts.statemachine.StateMachineBuilder;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.files.Logger;
import ftc.electronvolts.util.units.Distance;
import ftc.evlib.opmodes.AbstractAutoOp;
import ftc.evlib.statemachine.EVStates;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample autonomous that drives forward for 2 feet.
 */

@Autonomous(name = "SampleAuto V1")
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
        StateMachineBuilder b = new StateMachineBuilder(S.DRIVE);

        //define the DRIVE state to drive for 2 feet and move to the STOP state
        b.add(EVStates.drive(S.DRIVE, S.STOP, Distance.fromFeet(2), robotCfg.getTwoMotors(), 0.5));

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
        return null;
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

You can simplify this autonomous a bit by customizing the StateMachineBuilder (next chapter).