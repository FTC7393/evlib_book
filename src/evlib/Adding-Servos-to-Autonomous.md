Adding servos to the autonomous is a bit more tricky than adding them to TeleOp.
First we have to add convenience methods in the SampleStateMachineBuilder

[/sample/v3/SampleStateMachineBuilder.java](https://github.com/FTC7393/EVLib/blob/master/sample/v3/SampleStateMachineBuilder.java)
```java
package org.firstinspires.ftc.teamcode.sample.v3;

import ftc.electronvolts.statemachine.StateMachineBuilder;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.units.Distance;
import ftc.evlib.hardware.servos.ServoControl;
import ftc.evlib.hardware.servos.ServoName;
import ftc.evlib.statemachine.EVStates;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample extension of the StateMachineBuilder.
 * This subclass adds convenience methods for any opmodes that use the SampleRobotConfig.
 */

public class SampleStateMachineBuilder extends StateMachineBuilder {
    /**
     * The SampleRobotCfg to get the drive motors from
     */
    private final SampleRobotCfg sampleRobotCfg;

    /**
     * Create a SampleStateMachineBuilder, passing it the SampleRobotCfg
     * The SampleRobotCfg's drive motors will be passed to the drive state every time
     *
     * @param firstStateName the state to start with
     * @param sampleRobotCfg the robot's configuration
     */
    public SampleStateMachineBuilder(StateName firstStateName, SampleRobotCfg sampleRobotCfg) {
        super(firstStateName);
        this.sampleRobotCfg = sampleRobotCfg;
    }

    /**
     * convenience method for adding a drive state
     *
     * @param stateName the name of the state
     * @param nextStateName the name of the state to go to after the drive is complete
     * @param distance the distance to drive
     * @param velocity the velocity to drive at
     */
    public void addDrive(StateName stateName, StateName nextStateName, Distance distance, double velocity) {
        //add the drive state with the motors and speed from sampleRobotCfg
        add(EVStates.drive(stateName, nextStateName, distance, SampleRobotCfg.MAX_SPEED, sampleRobotCfg.getTwoMotors(), velocity));
    }

    /**
     * add a servo init state that sets all the servos to their starting positions
     *
     * @param stateName the name of the state
     * @param nextStateName the state to go to after the servos are initialized
     */
    public void addServoInit(StateName stateName, StateName nextStateName) {
        add(EVStates.servoInit(stateName, nextStateName, sampleRobotCfg.getServos()));
    }

    /**
     * turn a servo to a preset
     *
     * @param stateName the name of the state
     * @param nextStateName the state to go to after the servo is done turning
     * @param servoName the name of the servo as defined in SampleRobotCfg.ServoName
     * @param servoPreset the servo preset defined in SampleRobotCfg.ArmServoPresets and SampleRobotCfg.LegServoPresets
     * @param speed the speed to run the servo at
     * @param waitForDone wether or not to wait for the servo to turn to move to the next state
     */
    public void addServo(StateName stateName, StateName nextStateName, ServoName servoName, Enum servoPreset, double speed, boolean waitForDone) {
        //get the servo
        ServoControl servoControl = sampleRobotCfg.getServo(servoName);
        //add the state that will run the command
        add(EVStates.servoTurn(stateName, nextStateName, servoControl, servoPreset, speed, waitForDone));
    }

    /**
     * turn a servo to a preset at max speed
     *
     * @param stateName the name of the state
     * @param nextStateName the state to go to after the servo is done turning
     * @param servoName the name of the servo as defined in SampleRobotCfg.ServoName
     * @param servoPreset the servo preset defined in SampleRobotCfg.ArmServoPresets and SampleRobotCfg.LegServoPresets
     * @param waitForDone wether or not to wait for the servo to turn to move to the next state
     */
    public void addServo(StateName stateName, StateName nextStateName, ServoName servoName, Enum servoPreset, boolean waitForDone) {
        //get the servo
        ServoControl servoControl = sampleRobotCfg.getServo(servoName);
        //add the state that will run the command
        add(EVStates.servoTurn(stateName, nextStateName, servoControl, servoPreset, waitForDone));
    }


}
```



Then comes the easy part -- adding the servo init and turn states to the autonomous:

[/sample/v3/SampleAuto.java](https://github.com/FTC7393/EVLib/blob/master/sample/v3/SampleAuto.java)
```java
package org.firstinspires.ftc.teamcode.sample.v3;

import com.qualcomm.robotcore.eventloop.opmode.Autonomous;

import ftc.electronvolts.statemachine.StateMachine;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.units.Distance;
import ftc.evlib.opmodes.AbstractAutoOp;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample autonomous that:
 * 1. Initializes the servos
 * 2. Drives forward 2 feet
 * 3. Turns a servo arm
 */

@Autonomous(name="SampleAuto V3")
public class SampleAuto extends AbstractAutoOp<SampleRobotCfg> {

    /**
     * defines all the possible states for the state machine
     * modify this to have whatever states you want
     */
    private enum S implements StateName {
        SERVO_INIT,
        DRIVE,
        SERVO_ARM,
        STOP
    }

    @Override
    public StateMachine buildStates() {
        //create a new builder for the states, starting with the SERVO_INIT state
        //we are using the custom builder and passing it the robotCfg
        SampleStateMachineBuilder b = new SampleStateMachineBuilder(S.SERVO_INIT, robotCfg);

        //add the servo initialization state
        b.addServoInit(S.SERVO_INIT, S.DRIVE);

        //define the DRIVE state to drive for 2 feet and move to the STOP state
        b.addDrive(S.DRIVE, S.SERVO_ARM, Distance.fromFeet(2), 0.5);

        //add the servo turn state
        b.addServo(S.SERVO_ARM, S.STOP, SampleRobotCfg.SampleServoName.ARM_SERVO, SampleRobotCfg.ArmServoPresets.RIGHT, true);

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

The next step in the tutorial is [[Vuforia and OpenCV Beacon Color Detection]].