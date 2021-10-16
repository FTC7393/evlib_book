# TeleOp Program

A TeleOp is Tele-Operation: A period of time when *you*, the human, control the robot. A TeleOp therefore needs take input from a source (in 2021, an xbox controller), and somehow transform it into actions done by the robot. That's the part you make.

Let's get right into it, with the following starter code:
```java
MyTeleOp.java

@TeleOp(name = "My TeleOp")
public class MyTeleOp extends AbstractTeleOp<MyRobotCfg> {
    @Override
    protected MyRobotCfg createRobotCfg() {
        return new MyRobotCfg();
    }

    @Override
    protected Logger createLogger() {
        return null;
    }

    @Override
    protected void setup() {}

    @Override
    protected void setup_act() {}

    @Override
    protected void go() {}

    @Override
    protected void act() {}

    @Override
    protected void end() {}

    @Override
    protected Function getJoystickScalingFunction() {
        return Functions.linear(1.0);
    }
}
```

This is a *lot* of methods, and we'll break it down more in the chapter 3. For now, we'll set the goal of moving the motor counterclockwise with the left trigger of the controller, and clockwise with the right trigger.

To start, we need to figure out how much the motor should move. To do this, we'll need to transform the input from the controller into an output for the motor to be able to use.

```java
protected void act() {
    double power = gamepad1.left_trigger - gamepad1.right_trigger;
}
```

You can see that `gamepad1.left_trigger` is not a reference to hardware, but a value. This is another one of the ElectronVolts library's simplifications: Managing the input values. Next, we want to send this computed value to the DC motor.

```java
protected void act() {
    double power = gamepad1.left_trigger - gamepad1.right_trigger;
    robotCfg.getMyMotor().setPower(power);
}
```

This is relatively straightforward. Now, after the program is initialized and the play button is pressed, the `act` function will run in a loop, so you don't have to worry about the power only being set once.

The completed code should look like this:
```java
import com.qualcomm.robotcore.eventloop.opmode.TeleOp;
import com.qualcomm.robotcore.hardware.DcMotor;

import ftc.electronvolts.util.Function;
import ftc.electronvolts.util.Functions;
import ftc.electronvolts.util.files.Logger;
import ftc.evlib.opmodes.AbstractTeleOp;

@TeleOp(name = "My TeleOp")
public class MyTeleOp extends AbstractTeleOp<MyRobotCfg> {

    DcMotor myMotor;

    @Override
    protected MyRobotCfg createRobotCfg() {
        return new MyRobotCfg(hardwareMap);
    }

    @Override
    protected Logger createLogger() {
        return null;
    }

    @Override
    protected void setup() {}

    @Override
    protected void setup_act() {}

    @Override
    protected void go() {}

    @Override
    protected void act() {
        double power = gamepad1.left_trigger - gamepad1.right_trigger;
        robotCfg.getMyMotor().setPower(power);
    }

    @Override
    protected void end() {}

    @Override
    protected Function getJoystickScalingFunction() {
        return Functions.linear(1.0);
    }
}
```

Don't worry about all of those empty methods, we will go into more detail about those in the next chapter.
