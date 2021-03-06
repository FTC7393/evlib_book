If you are just starting out with FTC, using this library to its fullest extend probably seems intimidating. You can, however, use some of the simple functions of the library to save yourself some headaches. The following is an example of a TeleOp program that uses EVLib for 2 things:

1. [Edge detection on the joystick buttons](GamepadManager.md). It might not seem like a big deal, but we spent too much time with flags and nested if statements before moving the edge detection to a dedicated class.

2. [Management of the motor commands](Creating-Motors.md). The EVLib can "wrap" a DcMotor object into either a Motor object (no encoder) or a MotorEnc object. This wrapper class receives your commands and sends them when you call the update() method.

[/sample/v0/SampleTeleOp.java](https://github.com/FTC7393/EVLib/blob/master/sample/v0/SampleTeleOp.java)
```java
package org.firstinspires.ftc.teamcode.sample.v0;

import com.qualcomm.robotcore.eventloop.opmode.OpMode;
import com.qualcomm.robotcore.eventloop.opmode.TeleOp;
import com.qualcomm.robotcore.hardware.Servo;

import ftc.electronvolts.util.Function;
import ftc.electronvolts.util.Functions;
import ftc.electronvolts.util.Utility;
import ftc.evlib.driverstation.GamepadManager;
import ftc.evlib.hardware.motors.Motor;
import ftc.evlib.hardware.motors.Motors;
import ftc.evlib.hardware.motors.Stoppers;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 10/18/16
 *
 * Sample TeleOp that uses some of the EVLib functionality
 */

@TeleOp(name = "SampleTeleOp V0")
public class SampleTeleOp extends OpMode {
    /**
     * Adds edge detection to the gamepad buttons
     * and analog scaling to the joysticks
     */
    private GamepadManager driver1, driver2;

    /**
     * Wraps the motors to easily keep track of motor direction
     */
    private Motor leftMotor, rightMotor;

    /**
     * Servo object from the FTC libraries
     */
    private Servo servo;

    /**
     * The servo's current position from 0 to 1
     */
    private double servoPosition;

    /**
     * Takes care of stopping both motors
     */
    private Stoppers stoppers = new Stoppers();

    @Override
    public void init() {
        //get the motors from the hardwareMap and wrap them with the Motor interface
        //         motor factory class method      hardware name, reversed, brake
        leftMotor = Motors.withoutEncoder(hardwareMap, "leftMotor", false, true, stoppers);
        rightMotor = Motors.withoutEncoder(hardwareMap, "rightMotor", true, true, stoppers);

        //get the servo from the hardwareMap
        servo = hardwareMap.servo.get("servo");
    }

    @Override
    public void start() {
        //use a squared function for the joystick scaling
        Function scalingFunction = Functions.squared();

        //create the two drivers' gamepads
        driver1 = new GamepadManager(gamepad1, scalingFunction);
        driver2 = new GamepadManager(gamepad2, scalingFunction);

        //set the servo's position
        servoPosition = 0.5;
    }

    @Override
    public void loop() {
        //set the left and right motor powers based on the scaled joystick values
        leftMotor.setPower(driver1.left_stick_y.getValue());
        rightMotor.setPower(driver1.right_stick_y.getValue());

        //making use of the edge detection from the GamepadManager

        //if the dpad up was just pressed, increase the servo position
        if (driver1.dpad_up.justPressed()) {
            servoPosition += 0.1;
        }

        //if the dpad down was just pressed, decrease the servo position
        if (driver1.dpad_down.justPressed()) {
            servoPosition -= 0.1;
        }

        //limit the servo position to be in the valid range for servos (0 to 1)
        servoPosition = Utility.servoLimit(servoPosition);

        //send the command to the servo
        servo.setPosition(servoPosition);

        //send the motor powers to the motor controller
        leftMotor.update();
        rightMotor.update();
    }

    @Override
    public void stop() {
        //stop both motors
        stoppers.stop();
    }
}
```

If you want, you can stop here, you can check out a [Logging Example](Logging-Example.md) for another simple use of EVLib, or you can move on to [Robot Configuration](Robot-Configuration.md) if you want to learn more in depth about the library.