# A Starter Program

To start an ElectronVolts `TeleOp` project, make a new copy of the Qualcomm framework and add this into your TeamCode directory:

```
 java
 ├── MyRobotCfg.java
 └── MyTeleOp.java
```

Then add the ElectronVolts library into the directory like so:

```
 java
 ├── MyRobotCfg.java
 ├── MyTeleOp.java
 └── ftc
     ├── electronvolts
     │   └┄┄
     ├── evlib
     ┆   └┄┄
```

Once you've set up those files, you can start writing FTC code.

The end product should look something like below. This is just for later reference (i.e. when your code stops working), keep reading for the rest of the walkthrough.
```java
MyRobotCfg.java

import com.qualcomm.robotcore.hardware.DcMotor;
import com.qualcomm.robotcore.hardware.DcMotorEx;
import com.qualcomm.robotcore.hardware.HardwareMap;
import ftc.evlib.hardware.config.RobotCfg;

public class MyRobotCfg extends RobotCfg {
    private final DcMotor myMotor;

    public DcMotor getMyMotor() {
        return myMotor;
    }

    public MyRobotCfg(HardwareMap hardwareMap) {
        super(hardwareMap);
        myMotor = hardwareMap.get(DcMotorEx.class, "bruh");
    }

    @Override
    public void start() {}

    @Override
    public void act() {}

    @Override
    public void stop() {}
}

MyTeleOp.java

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
