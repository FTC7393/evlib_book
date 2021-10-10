## How the robot works

This is all review for those who've already done FTC.

In summary, the FTC robot core communicates over direct wifi with a phone, which controls the robot core. The robot core, in turn, can control connected hardware. You can read about it on the [FTC docs page](https://github.com/FIRST-Tech-Challenge/FtcRobotController/wiki/The-FTC-Control-System).

It is important to note that the program installed on the Robot Controller runs *instantly*. You'll see why this is important in the next section.

## FTC Framework

The software on the phone which communicates with the robot is constant. However, the code on the robot is dependent on you. Even so, there's a certain way that your program has to move motors, read from sensors, and communicate with the app on the phone. This is why, when writing your FTC program, you must do it within a framework.

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
- The `main` method here will not be run at all -- remember, the FTC bot runs the program instantly, so it has to do some steps before it can run your code (aka run its own main function).
- The `@Autonomous` annotation is wrong -- this class is not an "Operation Mode", which is what the framework is expecting. This will probably result in some strange runtime error (TODO: Add a picture of that runtime error).

Here is a correct implementation:

```java
import com.qualcomm.robotcore.eventloop.opmode.Autonomous;
import com.qualcomm.robotcore.eventloop.opmode.LinearOpMode;
import com.qualcomm.robotcore.hardware.Servo;

@Autonomous(name = "FooOp")
public class FooOp extends LinearOpMode {

    @Override
    public void runOpMode() throws InterruptedException {

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
- The `hardwareMap` now exists: It exists in the superclass of `LinearOpMode`
- The `@Autonomous` annotation now makes sense: It is annotating a class which is a Linear Operation Mode. Furthermore, that means when you download this program, you can use your driver station (or phone) to select and run your code.
