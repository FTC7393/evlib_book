In EVLib there are 2 types of motors. Motor, which has no encoder, and MotorEnc, which has an encoder. A MotorEnc extends Motor, which means that you can use a motor with an encoder as a motor without an encoder. To create instances of these, see the [[Creating Motors]] page.

The interface for Motor has only a few methods, and the only way to control it is by setting the power.

[ftc/evlib/hardware/motors/Motor.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/motors/Motor.java)
```java
package ftc.evlib.hardware.motors;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/11/16
 * 
 * Wrapper class for the DcMotor.
 * This represents the functions a motor without an encoder can do.
 *
 * @see MotorEnc
 * @see Motors
 */
public interface Motor {
    /**
     * The different modes the motor can be in
     */
    enum Mode {
        POWER, //directly control the power
        SPEED, //enable a feedback loop to correct speed (requires encoders)
        POSITION //turn the motor to a certain encoder position (requires encoders)
    }

    /**
     * Control the motor's raw voltage
     *
     * @param power value to set the power to
     */
    void setPower(double power);

    /**
     * Tells the mode the motor is in which is determined by the last command to the motor.
     *
     * @return the current mode
     */
    Mode getMode();

    /**
     * Sends motor commands to the motor controller
     */
    void update();
}
```


The MotorEnc interface has all the methods from Motor, plus the ones for the encoder-related functions.

[ftc/evlib/hardware/motors/MotorEnc.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/motors/MotorEnc.java)
```java
package ftc.evlib.hardware.motors;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/12/16
 * 
 * Wrapper class for DcMotor if the motor has an encoder
 * This interface has all the non-encoder methods from the Motor interface plus the ones shown here.
 * It can be passed in where a non-encoder Motor interface is needed.
 *
 * @see Motor
 * @see Motors
 */
public interface MotorEnc extends Motor {
    /**
     * A PID on the motor controller uses the encoder to regulate the speed of the motor.
     *
     * @param speed value to set the speed to
     */
    void setSpeed(double speed);

    /**
     * A PID on the motor controller uses the encoder to turn the motor to any encoder position.
     *
     * @param encoderTarget      position in encoder ticks to rotate to
     * @param maxCorrectionPower the max power to run the motor at when turning to the position
     */
    void setPosition(int encoderTarget, double maxCorrectionPower);

    /**
     * Set the encoder zero point to the current encoder value
     */
    void resetEncoder();

    /**
     * Get the encoder position relative to the zero value
     *
     * @return the encoder position
     */
    int getEncoderPosition();

}
```