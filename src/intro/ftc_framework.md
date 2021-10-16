## How the robot works

This part is review for those who've already done FTC.

The REV robotics core communicates over wifi with a phone, which controls the robot core. The robot core, in turn, can control connected hardware. You can read about it on the [FTC docs page](https://github.com/FIRST-Tech-Challenge/FtcRobotController/wiki/The-FTC-Control-System).

## FTC Framework

The software on the phone is always the same; the code on the robot is what you control. Even so, there's a certain way your program must act. This is why, when writing your FTC program, you must do it within a framework.

For example, a wrong way to write FTC code:
```java
import com.qualcomm.robotcore.eventloop.opmode.Autonomous;
import com.qualcomm.robotcore.hardware.Servo;

@Autonomous(name = "FooOp")
public class FooOp {
    public static void main(String[] args) {
        Servo s = (Servo) hardwareMap.get("myServo");
        int i = 0;

        while (true) {
            s.setPosition(i);
            i = (i + 10) % 180;
        }
    }
}
```

There are a few things wrong with this.
- The `hardwareMap` does not even exist, so you can't control any hardware.
- The `main` method here will not be run at all -- Qualcomm has their own `main` code, so yours will not be run.
- The `@Autonomous` annotation is wrong -- this class is not an "Operation Mode", which is what the framework is expecting. This will probably result in some strange runtime error (TODO: Add a picture of that runtime error).

Here is a correct implementation:

```java
import com.qualcomm.robotcore.eventloop.opmode.Autonomous;
import com.qualcomm.robotcore.eventloop.opmode.LinearOpMode;
import com.qualcomm.robotcore.hardware.Servo;

@Autonomous(name = "FooOp")
public class FooOp extends LinearOpMode {
    @Override
    public void runOpMode() {
        Servo s = (Servo) hardwareMap.get("myServo");
        int i = 0;

        waitForStart();

        while (true) {
            s.setPosition(i);
            i = (i + 10) % 180;
        }
    }
}
```

All the problems in the previous code have been corrected here:
- The `hardwareMap` now exists: It exists in the superclasses of `FooOp`.
- The `@Autonomous` annotation now makes sense: It is annotating a class which is a Linear Operation Mode. The Qualcomm framework can now correctly detect and run it.
