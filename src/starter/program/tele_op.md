# TeleOp Program

If you haven't done FTC before, a TeleOp is Tele-Operation: A period of time when *you*, the human, control the robot. A TeleOp therefore needs take input from a source (in 2021, an xbox controller), and somehow transform it into actions done by the robot. That's the part you make.

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

This is a *lot* of code, and we'll break it down more in the next chapter. For now, focus on the methods `setup() -> void` and `act() -> void`.

For now, we'll set the goal of moving the motor counterclockwise when hitting the left trigger of the controller, and clockwise on hitting the right trigger.

Let's start with the setup part of the program. You might want to import `myMotor` into this class to be able to use it easily, like this:

```java
public class MyTeleOp extends AbstractTeleOp<MyRobotCfg> {
    DcMotor myMotor;

    @Override
    protected void setup() {
        myMotor = robotCfg.getMyMotor();
    }
}
```

You can do this even if you're not using the getter pattern. This code tells the program to save a reference to the DcMotor on init. This step isn't necessary, and later this code might be bad. It saves some work if you want to write slightly less though.

The next step is to actually do something with the motor. For now, let's compute the power the motor should be moved with.

```java
protected void act() {
    double power = gamepad1.left_trigger - gamepad1.right_trigger;
}
```

You can see that `gamepad1.left_trigger` is not a reference to hardware, but a value. This is another one of the ElectronVolts library's simplifications: Managing the input values. Next, we want to send this computed value to the DC motor.

```java
protected void act() {
    double power = gamepad1.left_trigger - gamepad1.right_trigger;
    myMotor.setPower(power);
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
    protected void setup() {
        myMotor = robotCfg.getMyMotor();
    }

    @Override
    protected void setup_act() {}

    @Override
    protected void go() {}

    @Override
    protected void act() {
        double power = gamepad1.left_trigger - gamepad1.right_trigger;
        myMotor.setPower(power);
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
