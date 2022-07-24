Before you read this, it might be helpful to read about the [NMotors](NMotors.md) class first.

Mecanum wheels use diagonal forces to move in any direction. Here is a diagram of the direction of the force of each wheel when moving forward:

![mecanum force diagram](https://github.com/FTC7393/EVLib/blob/master/images/mecanum.png?raw=true)

To drive forward/backward, you just power all the wheels forward/backward.
To turn right, you power the right wheels backward and the left wheels forward, just like normal wheels.
(Turning left is the same idea.)

To move sideways right, move the front-left and back-right wheels forward, and the others backward.

Now we have this table:
```
            FORWARD(+x)   SIDEWAYS RIGHT(+y)   TURN RIGHT(+r)
front left      +                 +                  +
front right     +                 -                  -
back left       +                 -                  +
back right      +                 +                  -
```

And we can convert this table to an algorithm:

```
inputs: x, y, and r

flPower = + x + y + r
frPower = + x - y - r
blPower = + x - y + r
brPower = + x + y - r
```

This converts the desired motion of the robot into the power for the wheels.

The next thing to worry about is scaling to make sure the motors do not get sent a power that is greater than 1 or less than -1. Fortunately, the scaling is already implemented in the [NMotors](NMotors.md) class.

This is the MecanumMotors class, which extends FourMotors, which in turn extends NMotors. So all MecanumMotors has to do is convert the velocity in X, Y, and R to motor powers using the algorithm we found above. It also has various setters for X, y, and R, as well as functions to convert polar (direction and distance) coordinates to cartesian (x and y) coordinates.

> This example uses guava to initialize lists, which we have left for alternative methods.

[ftc/evlib/hardware/motors/MecanumMotors.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/motors/MecanumMotors.java)
```java
package ftc.evlib.hardware.motors;

import com.google.common.collect.ImmutableList;

import java.util.ArrayList;
import java.util.List;

import ftc.electronvolts.util.Utility;
import ftc.electronvolts.util.units.Angle;
import ftc.electronvolts.util.units.Velocity;

import static ftc.evlib.driverstation.Telem.telemetry;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 12/27/15
 *
 * A subclass of FourMotors that contains algorithms for controlling mecanum wheels.
 * It stores the X, Y, and R velocities and sends them to the motors when it is updated.
 *
 * @see FourMotors
 * @see ftc.evlib.hardware.control.MecanumControl
 */
public class MecanumMotors extends FourMotors {

    public enum MecanumDriveMode {
        NORMALIZED, TRANSLATION_NORMALIZED
    }

    //the stored velocities
    private double velocityX = 0;
    private double velocityY = 0;
    private double velocityR = 0;

    //the drive mode
    private MecanumDriveMode driveMode = MecanumDriveMode.NORMALIZED;

    /**
     * @param frontLeftMotor  the front left motor
     * @param frontRightMotor the front right motor
     * @param backLeftMotor   the back left motor
     * @param backRightMotor  the back right motor
     * @param useSpeedMode    whether or not to use speed control on the motors
     * @param maxRobotSpeed   the measured speed of the robot at 100% power
     */
    public MecanumMotors(Motor frontLeftMotor, Motor frontRightMotor, Motor backLeftMotor, Motor backRightMotor, boolean useSpeedMode, Velocity maxRobotSpeed) {
        super(frontLeftMotor, frontRightMotor, backLeftMotor, backRightMotor, useSpeedMode, maxRobotSpeed);
    }

    /**
     * @param mode the drive mode
     */
    public void setDriveMode(MecanumDriveMode mode) {
        driveMode = mode;
    }

    /**
     * Update the motor powers
     */
    public void mecanumDrive() {
        switch (driveMode) {
            case NORMALIZED:
                mecanumDriveNormalized();
                break;
            case TRANSLATION_NORMALIZED:
                mecanumDriveTranslationNormalized();
                break;
        }
    }

    /**
     * run the motors based on the xyr velocities
     * normalize if any motor power is too large
     */
    private void mecanumDriveNormalized() {
        //calculate motor powers
        runMotorsNormalized(
                velocityX + velocityY - velocityR,
                velocityX - velocityY + velocityR,
                velocityX - velocityY - velocityR,
                velocityX + velocityY + velocityR
        );
    }

    /**
     * Calculate rotational velocity first, and use remaining headway for translation.
     */
    private void mecanumDriveTranslationNormalized() {
        //calculate motor powers
        List<Double> translationValues = ImmutableList.of(
                velocityX + velocityY,
                velocityX - velocityY,
                velocityX - velocityY,
                velocityX + velocityY);

        List<Double> rotationValues = ImmutableList.of(
                -velocityR,
                velocityR,
                -velocityR,
                velocityR);

        double scaleFactor = 1;
        double tmpScale = 1;

        // Solve this equation backwards:
        // MotorX = TranslationX * scaleFactor + RotationX
        // to find scaleFactor that ensures -1 <= MotorX <= 1 and 0 < scaleFactor <= 1

        for (int i = 0; i < 4; i++) {
            if (Math.abs(translationValues.get(i) + rotationValues.get(i)) > 1) {
                tmpScale = (1 - rotationValues.get(i)) / translationValues.get(i);
            } else if (translationValues.get(i) + rotationValues.get(i) < -1) {
                tmpScale = (rotationValues.get(i) - 1) / translationValues.get(i);
            }
            if (tmpScale < scaleFactor) {
                scaleFactor = tmpScale;
            }
        }

        telemetry.addData("driveMode", driveMode.toString());
        telemetry.addData("scaleFactor", scaleFactor);

        List<Double> valuesScaled = new ArrayList<>();
        for (int i = 0; i < 4; i++) {
            valuesScaled.add(translationValues.get(i) * scaleFactor + rotationValues.get(i));
            telemetry.addData("valuesScaled(" + i + ")", valuesScaled.get(i));
        }

        run(valuesScaled);
    }

    /**
     * @param velocityX the x velocity
     */
    public void setVelocityX(double velocityX) {
        this.velocityX = Utility.motorLimit(velocityX);
    }

    /**
     * @param velocityY the y velocity
     */
    public void setVelocityY(double velocityY) {
        this.velocityY = Utility.motorLimit(velocityY);
    }

    /**
     * @param velocityR the rotational velocity
     */
    public void setVelocityR(double velocityR) {
        this.velocityR = Utility.motorLimit(velocityR);
    }

    /**
     * set the x and y velocities at the same time
     *
     * @param velocityX the x velocity
     * @param velocityY the y velocity
     */
    public void setVelocityXY(double velocityX, double velocityY) {
        setVelocityX(velocityX);
        setVelocityY(velocityY);
    }

    /**
     * set the x, y, and rotational velocities at the same time
     *
     * @param velocityX the x velocity
     * @param velocityY the y velocity
     * @param velocityR the rotational velocity
     */
    public void setVelocityXYR(double velocityX, double velocityY, double velocityR) {
        setVelocityX(velocityX);
        setVelocityY(velocityY);
        setVelocityR(velocityR);
    }

    /**
     * set the x and y velocities using polar coordinates
     *
     * @param velocity  the velocity (r of the polar coordinate)
     * @param direction the direction (theta of the polar coordinate)
     */
    public void setVelocityPolar(double velocity, Angle direction) {
        double directionRadians = direction.radians();
        setVelocityX(velocity * Math.cos(directionRadians));
        setVelocityY(velocity * Math.sin(directionRadians));
    }

    /**
     * set the x and y velocities using polar coordinates, and also set the rotational velocity
     *
     * @param velocity  the velocity (r of the polar coordinate)
     * @param direction the direction (theta of the polar coordinate)
     * @param velocityR the rotational velocity
     */
    public void setVelocityPolarR(double velocity, Angle direction, double velocityR) {
        setVelocityPolar(velocity, direction);
        setVelocityR(velocityR);
    }
}
```