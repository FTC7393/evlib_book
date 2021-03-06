After you have [set up a RobotCfg](Robot-Configuration.md), you can now write TeleOp and Autonomous OpModes.

The following is a sample TeleOp program that uses the sample RobotCfg.

[/sample/v1/SampleTeleOp.java](https://github.com/FTC7393/EVLib/blob/master/sample/v1/SampleTeleOp.java)
```java
package org.firstinspires.ftc.teamcode.sample.v1;

import com.qualcomm.robotcore.eventloop.opmode.TeleOp;

import ftc.electronvolts.util.Function;
import ftc.electronvolts.util.Functions;
import ftc.electronvolts.util.files.Logger;
import ftc.evlib.opmodes.AbstractTeleOp;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * A sample TeleOp program that will allow you to control 2 motors with the left and right joysticks
 */

@TeleOp(name = "SampleTeleOp V1")
public class SampleTeleOp extends AbstractTeleOp<SampleRobotCfg> {
    @Override
    protected Function getJoystickScalingFunction() {
        //use an exponentially based function for the joystick scaling to allow fine control
        return Functions.eBased(5);
//        return Functions.squared();
//        return Functions.cubed();
//        return Functions.none();
    }

    @Override
    protected SampleRobotCfg createRobotCfg() {
        //create and return a SampleRobotCfg for the library to use
        return new SampleRobotCfg(hardwareMap);
    }

    @Override
    protected Logger createLogger() {
        return null;
    }

    @Override
    protected void setup() {

    }

    @Override
    protected void setup_act() {

    }

    @Override
    protected void go() {

    }

    @Override
    protected void act() {
        //set the motor powers to the joystick values
        robotCfg.getTwoMotors().runMotors(
                driver1.left_stick_y.getValue(),
                driver1.right_stick_y.getValue()
        );
    }

    @Override
    protected void end() {

    }
}
```

The next step is to make a Basic Autonomous Program (next chapter).