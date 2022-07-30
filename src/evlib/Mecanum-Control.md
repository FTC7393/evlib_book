> This class has been found to have logic errors, though the source of these has not been pinpointed. We are currently working on an alternative

Using the [MecanumMotors](Mecanum-Wheels.md) class that converts velocity in the X, Y, and R directions, we need a way to add control algorithms, such as gyro stabilization or line following.

The gyro algorithm would look like:
```
Loop:
  get current heading from gyro
  calculate heading error from desired heading
  adjust ROTATION based on heading error
```

And the line following would look like:
(The x axis is the forward/backward movement of the robot)
```
Loop:
  get values from line sensors
  calculate position on the line (LEFT, MIDDLE, or RIGHT)
  if LEFT: y = positive
  if middle: y = 0
  if RIGHT: y = negative
  x is defined by the input
  set the TRANSLATION to (x, y)
```

As you can see, the gyro stabilization controls the rotation, and the line following controls the translation.

Now the challenge is to make a class that can accept different types of translation/rotation controllers.

First, we create interfaces TranslationControl and RotationControl to define what they have output:

[ftc/evlib/hardware/control/TranslationControl.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/TranslationControl.java)
```java
package ftc.evlib.hardware.control;

import ftc.electronvolts.util.Vector2D;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/19/16
 * 
 * Controls the translation of a mecanum robot
 *
 * @see TranslationControls
 * @see Vector2D
 */
public interface TranslationControl {
    /**
     * update the velocity x and velocity y values
     *
     * @return true if there were no problems
     */
    boolean act();

    /**
     * @return the translational velocity
     */
    Vector2D getTranslation();
}
```

As you can see, the translation control returns a Vector2D (x and y).

RotationControl is more complicated, because it not only has to find the angular velocity, but also apply a correction of a certain angle to the translation.

[ftc/evlib/hardware/control/RotationControl.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/RotationControl.java)
```java
package ftc.evlib.hardware.control;

import ftc.electronvolts.util.units.Angle;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/19/16
 * 
 * Controls the rotation of a mecanum robot
 *
 * @see RotationControls
 * @see Angle
 */
public interface RotationControl {
    double DEFAULT_MAX_ANGULAR_SPEED = 0.5;
//    double DEFAULT_MAX_ANGULAR_SPEED = 0.8;

    /**
     * update the rotational velocity
     *
     * @return true if there were no problems
     */
    boolean act();

    /**
     * @return the rotational velocity
     */
    double getVelocityR();

    /**
     * accounts for the robot's rotation being off when translating
     *
     * @return correction to the translation direction
     */
    Angle getPolarDirectionCorrection();
}
```

There are factory classes for the TranslationControl and RotationControl:
[ftc/evlib/hardware/control/TranslationControls.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/TranslationControls.java)
[ftc/evlib/hardware/control/RotationControls.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/RotationControls.java)

They create the controllers such as the gyro stabilization, line following, and also controllers that return constant values for dead reckoning.

For controllers that control the rotation and translation, there is an abstract class called XYRControl that combines both the RotationControl and TranslationControl interfaces.

[ftc/evlib/hardware/control/XYRControl.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/XYRControl.java)
```java
package ftc.evlib.hardware.control;

import ftc.electronvolts.util.units.Angle;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 12/14/16
 *
 * Controls both the translation (x and y) and the rotation (r) of a robot with mecanum wheels
 *
 * @see RotationControl
 * @see TranslationControl
 * @see XYRControls
 */
public abstract class XYRControl implements RotationControl, TranslationControl {
    @Override
    public Angle getPolarDirectionCorrection() {
        return Angle.zero();
    }
}
```

The factory class for XYRControl is [ftc/evlib/hardware/control/XYRControls.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/XYRControls.java)

Now we need to take the value from the TranslationControl and RotationControl and pass them to the MecanumMotors. This is done by MecanumControl:

[ftc/evlib/hardware/control/MecanumControl.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/control/MecanumControl.java)

```java
package ftc.evlib.hardware.control;

import ftc.electronvolts.util.Vector2D;
import ftc.electronvolts.util.units.Velocity;
import ftc.evlib.hardware.motors.MecanumMotors;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/19/16
 * 
 * Manages what algorithms control the rotation and translation of the mecanum wheels
 * This allows you to mix and match rotation and translation (x and y) controllers and change them whenever you want
 *
 * @see MecanumMotors
 * @see RotationControl
 * @see RotationControls
 * @see TranslationControl
 * @see TranslationControls
 */
public class MecanumControl {
    /**
     * the motors to be controlled
     */
    private final MecanumMotors mecanumMotors;

    //the controllers for the rotation and translation
    private RotationControl rotationControl;
    private TranslationControl translationControl;

    /**
     * the velocity in the x, y, and rotation directions
     */
    private double velocityX, velocityY, velocityR;

    /**
     * stores whether or not the translation/rotation worked
     * for example, translationWorked will be false if the line following lost the line
     */
    private boolean translationWorked = true, rotationWorked = false;

    /**
     * create a MecanumControl that is not moving
     *
     * @param mecanumMotors the motors to control
     */
    public MecanumControl(MecanumMotors mecanumMotors) {
        this(mecanumMotors, XYRControls.ZERO);
    }

    /**
     * create a MecanumControl and immediately start using an XYR control
     *
     * @param mecanumMotors the motors to control
     * @param xyrControl    how to command the motors' rotation and translation
     */
    public MecanumControl(MecanumMotors mecanumMotors, XYRControl xyrControl) {
        this(mecanumMotors, xyrControl, xyrControl);
    }

    /**
     * create a MecanumControl and immediately start using TranslationControl and RotationControl
     *
     * @param mecanumMotors      the motors to control
     * @param rotationControl    how to command the motors' rotation
     * @param translationControl how to command the motors' translation
     */
    public MecanumControl(MecanumMotors mecanumMotors, RotationControl rotationControl, TranslationControl translationControl) {
        this.rotationControl = rotationControl;
        this.mecanumMotors = mecanumMotors;
        this.translationControl = translationControl;
    }

    /**
     * @return the robot's speed when the power is set to 100%
     */
    public Velocity getMaxRobotSpeed() {
        return mecanumMotors.getMaxRobotSpeed();
    }

    /**
     * @param rotationControl the controller that determines the robot's rotation
     */
    public void setRotationControl(RotationControl rotationControl) {
        this.rotationControl = rotationControl;
    }

    /**
     * @param translationControl the controller that determines the robot's translation (x and y)
     */
    public void setTranslationControl(TranslationControl translationControl) {
        this.translationControl = translationControl;
    }

    /**
     * set both translation and rotation controls at once
     *
     * @param translationControl the controller that determines the robot's translation (x and y)
     * @param rotationControl    the controller that determines the robot's rotation
     */
    public void setControl(TranslationControl translationControl, RotationControl rotationControl) {
        this.translationControl = translationControl;
        this.rotationControl = rotationControl;
    }

    /**
     * set both translation and rotation controls at once
     *
     * @param rotationControl    the controller that determines the robot's rotation
     * @param translationControl the controller that determines the robot's translation (x and y)
     */
    public void setControl(RotationControl rotationControl, TranslationControl translationControl) {
        this.translationControl = translationControl;
        this.rotationControl = rotationControl;
    }

    /**
     * Set both translation and rotation controls to the same object
     *
     * @param xyrControl the controller that determines both the translation and rotation
     */
    public void setControl(XYRControl xyrControl) {
        this.translationControl = xyrControl;
        this.rotationControl = xyrControl;
    }

    /**
     * stop the motors
     */
    public void stop() {
        translationControl = TranslationControls.ZERO;
        rotationControl = RotationControls.ZERO;
        mecanumMotors.stop();
    }

    /**
     * set the drive mode to turn on or off translation normalizing
     *
     * @param mode the drive mode
     */
    public void setDriveMode(MecanumMotors.MecanumDriveMode mode) {
        mecanumMotors.setDriveMode(mode);
    }

    /**
     * update the motor powers based on the output of the translationControl and rotationControl
     */
    public void act() {
        translationWorked = translationControl.act();

        //in case the same object is passed in that implements both controllers
        if (translationControl == rotationControl) {
            rotationWorked = translationWorked;
        } else {
            rotationWorked = rotationControl.act();
        }

        Vector2D translation = translationControl.getTranslation();

        double velocity = translation.getLength();
        double directionRads = translation.getDirection().radians() +
                rotationControl.getPolarDirectionCorrection().radians();

        velocityX = velocity * Math.cos(directionRads);
        velocityY = velocity * Math.sin(directionRads);

        velocityR = rotationControl.getVelocityR();

        mecanumMotors.setVelocityXYR(
                velocityX,
                velocityY,
                velocityR
        );

        mecanumMotors.mecanumDrive();
        mecanumMotors.update();
    }

    /**
     * @return whether or not the translation worked
     */
    public boolean translationWorked() {
        return translationWorked;
    }

    /**
     * @return whether or not the rotation worked
     */
    public boolean rotationWorked() {
        return rotationWorked;
    }

    /**
     * @return the robot's x velocity
     */
    public double getVelocityX() {
        return velocityX;
    }

    /**
     * @return the robot's y velocity
     */
    public double getVelocityY() {
        return velocityY;
    }

    /**
     * @return the robot's rotational velocity
     */
    public double getVelocityR() {
        return velocityR;
    }

    public RotationControl getRotationControl() {
        return rotationControl;
    }

    public TranslationControl getTranslationControl() {
        return translationControl;
    }
}
```