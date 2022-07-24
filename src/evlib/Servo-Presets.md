The idea of servo presets is to replace "magic numbers" (numbers whose purpose is not obvious) with named servo positions. For example, instead of ```7.341```, you can write ```ArmServoPresets.LEFT``` -- much more readable.

Another feature of the servo presets is that you can modify the preset values without changing the app and re-uploading to the phone.

The way these two features are implemented is that each servo has an enum of presets:
```java
    /**
     * all the possible values for the arm servo
     */
    public enum ArmServoPresets {
        LEFT,
        MIDDLE,
        RIGHT
    }
```

and each RobotCfg has a list of servos, each of which has a hardware name (such as ```"armServo"```) and a list of presets determined from the enum (such as ```ArmServoPresets.values()```):
```java

    /**
     * defines all the servos on the robot
     */
    public enum SampleServoName implements ServoName {
        //enum name("hardware name", preset enum.values()),
        ARM_SERVO("armServo", ArmServoPresets.values()),
        LEG_SERVO("legServo", LegServoPresets.values());

        private final String hardwareName;
        private final Enum[] presets;

        SampleServoName(String hardwareName, Enum[] presets) {
            this.hardwareName = hardwareName;
            this.presets = presets;
        }

        @Override
        public String getHardwareName() {
            return hardwareName;
        }

        @Override
        public Enum[] getPresets() {
            return presets;
        }

        @Override
        public Class<? extends RobotCfg> getRobotCfg() {
            return SampleRobotCfg.class;
        }


    }
```

To actually enter the preset values, you need to make and run a ServoTuneOp:

[/sample/v3/SampleServoTuneOp.java](https://github.com/FTC7393/EVLib/blob/master/sample/v3/SampleServoTuneOp.java)
```java
package org.firstinspires.ftc.teamcode.sample.v3;

import com.qualcomm.robotcore.eventloop.opmode.TeleOp;

import ftc.evlib.hardware.config.RobotCfg;
import ftc.evlib.opmodes.AbstractServoTuneOp;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * This opmode is very short since the superclass, AbstractServoTuneOp does most of the work. It
 * allows you to change your servo presets without changing the code and re-deploying it to the
 * phone. This means that you can swap out a servo and re-tune it without having to go into the
 * program and fix magic numbers. Note:  It only works if you use presets everywhere instead of
 * hardcoded values.
 *
 * How to use:
 * Select this opmode from the TeleOp menu and run it.
 * Use the dpad up and down to cycle through all the servos
 * Use the dpad left and right to move through the presets for that servo.
 * Press start to save the current preset of the current servo to the current value.
 *
 * The presets are saved in files that are retrieved when you run other opmodes to find the value of each preset.
 */

@TeleOp(name = "SampleServoTuneOp V3")
public class SampleServoTuneOp extends AbstractServoTuneOp {
    @Override
    protected RobotCfg createRobotCfg() {
        //create a new SampleRobotConfig and return it.
        //the superclass will extract the servos and do the rest.
        return new SampleRobotCfg(hardwareMap);
    }
}
```

The details of how the servos are initialized are in the next chapter.