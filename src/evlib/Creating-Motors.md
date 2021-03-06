Before you read this, it is helpful to read about the [Motor and MotorEnc](Motor-and-MotorEnc.md) interfaces.

To get a DcMotor from the hardwareMap and wrap it with the Motor interface, you can do the following:

```java
Stoppers stoppers = new Stoppers(); //pass this in each time you create a motor
boolean reversed = false;
boolean brake = true;

DcMotor leftDcMotor = hardwareMap.dcMotor.get("leftMotor")
Motor leftMotor = Motors.withoutEncoder(leftDcMotor, reversed, brake, stoppers);
```

or you can combine them into 1 line:

```java
Motor leftMotor = Motors.withoutEncoder(hardwareMap, "leftMotor", false, true, stoppers);
```

You can also create a Motor from a Continuous servo:
```java
boolean reversed = false;
Motor arm = Motors.continuousServo(hardwareMap.crservo.get("servoArm"), reversed);
```

To combine two motors:
```java
Motor combined = Motors.combinedWithoutEncoder(leftMotor, arm);
```

To combine more than 2 motors:
```java
Motor combined = Motors.combinedWithoutEncoder(
        ImmutableList.of(motor1, motor2, motor3, ...)
);
```

Here are all the methods for creating motors:

> This example uses guava to initialize lists, which we have left for alternative methods.

[ftc/evlib/hardware/motors/Motors.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/hardware/motors/Motors.java)
```java
package ftc.evlib.hardware.motors;

import com.google.common.collect.ImmutableList;
import com.qualcomm.robotcore.hardware.CRServo;
import com.qualcomm.robotcore.hardware.DcMotor;
import com.qualcomm.robotcore.hardware.DcMotorSimple;
import com.qualcomm.robotcore.hardware.HardwareMap;

import java.util.List;

import ftc.electronvolts.util.Function;
import ftc.electronvolts.util.Utility;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 9/11/16
 *
 * Factory class for creating Motor wrapper classes.
 * Has methods for all the combinations of with/without encoders and forward/reversed.
 *
 * @see Motor
 * @see MotorEnc
 */
public class Motors {
    /**
     * combine two motors with encoders into one motor
     *
     * @param motorEnc1 the first motor (with encoder)
     * @param motorEnc2 the second motor (with encoder)
     * @return the motor that controls both (with encoder support)
     */
    public static MotorEnc combinedWithEncoder(MotorEnc motorEnc1, MotorEnc motorEnc2) {
        return combinedWithEncoder(ImmutableList.of(motorEnc1, motorEnc2));
    }

    /**
     * combines any number of motors with encoders into one
     *
     * @param motorEncs the list of motors to combine (all must have encoders)
     * @return the motor that controls all of them (with encoder support)
     */
    public static MotorEnc combinedWithEncoder(final List<MotorEnc> motorEncs) {
        return new MotorEnc() {
            @Override
            public void setSpeed(double speed) {
                for (MotorEnc motorEnc : motorEncs)
                    motorEnc.setSpeed(speed);
            }

            @Override
            public void setPosition(int encoderTarget, double maxCorrectionPower) {
                for (MotorEnc motorEnc : motorEncs)
                    motorEnc.setPosition(encoderTarget, maxCorrectionPower);
            }

            @Override
            public void resetEncoder() {
                for (MotorEnc motorEnc : motorEncs) motorEnc.resetEncoder();
            }

            @Override
            public int getEncoderPosition() {
                int total = 0;
                for (MotorEnc motorEnc : motorEncs) total += motorEnc.getEncoderPosition();
                if (motorEncs.size() == 0) {
                    return 0;
                } else {
                    return total / motorEncs.size();
                }
            }

            @Override
            public void setPower(double power) {
                for (MotorEnc motorEnc : motorEncs) motorEnc.setPower(power);
            }

            @Override
            public Mode getMode() {
                if (motorEncs.size() == 0) {
                    return Mode.POWER;
                } else {
                    return motorEncs.get(0).getMode();
                }
            }

            @Override
            public void update() {
                for (MotorEnc motorEnc : motorEncs) motorEnc.update();
            }
        };
    }

    /**
     * combine two motors with or without encoders into one motor
     *
     * @param motor1 the first motor
     * @param motor2 the second motor
     * @return the motor that controls both (without encoder support)
     */
    public static Motor combinedWithoutEncoder(Motor motor1, Motor motor2) {
        return combinedWithoutEncoder(ImmutableList.of(motor1, motor2));
    }

    /**
     * combines any number of motors with or without encoders into one
     *
     * @param motors the list of motors to combine
     * @return the motor that controls all of them (without encoder support)
     */
    public static Motor combinedWithoutEncoder(final List<Motor> motors) {
        return new Motor() {
            @Override
            public void setPower(double power) {
                for (Motor motor : motors) motor.setPower(power);
            }

            @Override
            public Mode getMode() {
                return Mode.POWER;
            }

            @Override
            public void update() {
                for (Motor motor : motors) motor.update();
            }
        };
    }

    /**
     * Initialize a dcMotor by setting parameters and checking that they were set properly
     *
     * @param dcMotor  the motor to initialize
     * @param reversed whether or not the motor direction should be reversed
     * @param brake    whether to brake or float when stopping
     * @param runMode  what mode to start the motor in
     */
    private static void dcMotorInit(DcMotor dcMotor, boolean reversed, boolean brake, DcMotor.RunMode runMode) {
        //reset the encoder position to zero
        do {
            dcMotor.setMode(DcMotor.RunMode.STOP_AND_RESET_ENCODER);
        } while (dcMotor.getMode() != DcMotor.RunMode.STOP_AND_RESET_ENCODER);

        //determine the motor's direction as a Direction object
        DcMotor.Direction direction;
        if (reversed) {
            direction = DcMotorSimple.Direction.REVERSE;
        } else {
            direction = DcMotorSimple.Direction.FORWARD;
        }

        //set the motor's direction
        do {
            dcMotor.setDirection(direction);
        } while (dcMotor.getDirection() != direction);

        //set the motor's mode
        do {
            dcMotor.setMode(runMode);
        } while (dcMotor.getMode() != runMode);

        //determine the ZeroPowerBehavior
        DcMotor.ZeroPowerBehavior zeroPowerBehavior;
        if (brake) {
            zeroPowerBehavior = DcMotor.ZeroPowerBehavior.BRAKE;
        } else {
            zeroPowerBehavior = DcMotor.ZeroPowerBehavior.FLOAT;
        }

        //set the motor's ZeroPowerBehavior
        do {
            dcMotor.setZeroPowerBehavior(zeroPowerBehavior);
        } while (dcMotor.getZeroPowerBehavior() != zeroPowerBehavior);
    }

    /**
     * Create a Motor from the hardware map
     *
     * @param hardwareMap              the hardwareMap from the opmode
     * @param dcMotorName              the name of the DcMotor in the hardwareMap
     * @param reversed                 true if the motor's direction should be reversed
     * @param brake                    true if the motor should brake when stopped
     * @param stoppers                 the Stoppers object to add the motor to
     * @return the created MotorEnc
     */
    public static Motor withoutEncoder(HardwareMap hardwareMap, String dcMotorName, boolean reversed, boolean brake, Stoppers stoppers) {
        return withoutEncoder(hardwareMap.dcMotor.get(dcMotorName), reversed, brake, stoppers);
    }

    /**
     * Create a Motor from a DcMotor
     *
     * @param dcMotor                  the DcMotor to be wrapped
     * @param reversed                 true if the motor's direction should be reversed
     * @param brake                    true if the motor should brake when stopped
     * @param stoppers                 the Stoppers object to add the motor to
     * @return the created MotorEnc
     */
    public static Motor withoutEncoder(final DcMotor dcMotor, boolean reversed, boolean brake, Stoppers stoppers) {
        //initialize the motor with no encoder
        dcMotorInit(dcMotor, reversed, brake, DcMotor.RunMode.RUN_WITHOUT_ENCODER);

        stoppers.add(new Stopper() {
            @Override
            public void stop() {
                do {
                    dcMotor.setPower(0);
                } while (dcMotor.getPower() != 0);
            }
        });

        return new Motor() {
            private double power = 0;

            @Override
            public void setPower(double power) {
                this.power = power;
            }

            @Override
            public Mode getMode() {
                return Mode.POWER;
            }

            @Override
            public void update() {
                dcMotor.setPower(Utility.motorLimit(power));
            }
        };
    }

    /**
     * Convert a Motor.MotorMode to a DcMotor.RunMode
     *
     * @param mode the mode to convert
     * @return the corresponding DcMotor.RunMode
     */
    public static DcMotor.RunMode motorModeToDcMotorRunMode(Motor.Mode mode) {
        switch (mode) {
            case POWER:
                return DcMotor.RunMode.RUN_WITHOUT_ENCODER;
            case SPEED:
                return DcMotor.RunMode.RUN_USING_ENCODER;
            case POSITION:
                return DcMotor.RunMode.RUN_TO_POSITION;
            default:
                return null;
        }
    }

    /**
     * Convert a DcMotor.RunMode to a Motor.Mode
     *
     * @param runMode the mode to convert
     * @return the corresponding Motor.Mode
     */
    public static Motor.Mode dcMotorRunModeToMotorMode(DcMotor.RunMode runMode) {
        switch (runMode) {
            case RUN_WITHOUT_ENCODER:
                return Motor.Mode.POWER;
            case RUN_USING_ENCODER:
                return Motor.Mode.SPEED;
            case RUN_TO_POSITION:
                return Motor.Mode.POSITION;
            default:
                return null;
        }
    }

    /**
     * Create a MotorEnc from the hardware map
     *
     * @param hardwareMap              the hardwareMap from the opmode
     * @param dcMotorName              the name of the DcMotor in the hardwareMap
     * @param maxEncoderTicksPerSecond the encoder ticks per second at max power
     * @param reversed                 true if the motor's direction should be reversed
     * @param brake                    true if the motor should brake when stopped
     * @param stoppers                 the Stoppers object to add the motor to
     * @return the created MotorEnc
     */
    public static MotorEnc withEncoder(HardwareMap hardwareMap, String dcMotorName, int maxEncoderTicksPerSecond, boolean reversed, boolean brake, Stoppers stoppers) {
        return withEncoder(hardwareMap.dcMotor.get(dcMotorName), maxEncoderTicksPerSecond, reversed, brake, stoppers);
    }

    /**
     * Create a MotorEnc from a DcMotor
     *
     * @param dcMotor                  the DcMotor to be wrapped
     * @param maxEncoderTicksPerSecond the encoder ticks per second at max power
     * @param reversed                 true if the motor's direction should be reversed
     * @param brake                    true if the motor should brake when stopped
     * @param stoppers                 the Stoppers object to add the motor to
     * @return the created MotorEnc
     */
    public static MotorEnc withEncoder(final DcMotor dcMotor, int maxEncoderTicksPerSecond, boolean reversed, boolean brake, Stoppers stoppers) {
        final Motor.Mode initMode = Motor.Mode.SPEED;
        dcMotorInit(dcMotor, reversed, brake, motorModeToDcMotorRunMode(initMode)); //start with speed mode

        do {
            dcMotor.setMaxSpeed(maxEncoderTicksPerSecond);
        } while (dcMotor.getMaxSpeed() != maxEncoderTicksPerSecond);

        stoppers.add(new Stopper() {
            @Override
            public void stop() {
                do {
                    dcMotor.setMode(DcMotor.RunMode.RUN_WITHOUT_ENCODER);
                } while (dcMotor.getMode() != DcMotor.RunMode.RUN_WITHOUT_ENCODER);
                do {
                    dcMotor.setPower(0);
                } while (dcMotor.getPower() != 0);
            }
        });

        return new MotorEnc() {
            private int encoderZero = 0, encoderPosition = 0;
            private Mode mode = initMode, lastMode = initMode;
            private double power = 0;
            private int encoderTarget = 0;

            @Override
            public void setPower(double power) {
                mode = Mode.POWER;
                this.power = power;
            }

            @Override
            public void setSpeed(double speed) {
                mode = Mode.SPEED;
                power = speed;
            }

            @Override
            public void setPosition(int encoderTarget, double maxCorrectionPower) {
                mode = Mode.POSITION;
                this.encoderTarget = encoderTarget;
                power = maxCorrectionPower;
            }

            @Override
            public void resetEncoder() {
                encoderZero = encoderPosition;
            }

            @Override
            public int getEncoderPosition() {
                return encoderPosition - encoderZero;
            }

            @Override
            public Mode getMode() {
                return mode;
            }

            @Override
            public void update() {
                encoderPosition = dcMotor.getCurrentPosition();

                if (mode != lastMode) {
                    dcMotor.setMode(motorModeToDcMotorRunMode(mode));
                    lastMode = dcMotorRunModeToMotorMode(dcMotor.getMode());
                }

                switch (mode) {
                    case POWER:
                        break;
                    case SPEED:
                        break;
                    case POSITION:
                        dcMotor.setTargetPosition(encoderTarget);
                        break;
                }
                dcMotor.setPower(Utility.motorLimit(power));
            }
        };
    }

    /**
     * Wraps a continuous rotation servo as a normal motor
     *
     * @param crServo  the servo to be wrapped as a motor
     * @param reversed true if the servo should be reversed
     * @return the Motor wrapper class
     */
    public static Motor continuousServo(final CRServo crServo, boolean reversed) {
        DcMotorSimple.Direction direction;
        if (reversed) {
            direction = DcMotorSimple.Direction.REVERSE;
        } else {
            direction = DcMotorSimple.Direction.FORWARD;
        }

        do {
            crServo.setDirection(direction);
        } while (crServo.getDirection() != direction);

        return new Motor() {
            private double power = 0;

            @Override
            public void setPower(double power) {
                this.power = power;
            }

            @Override
            public Mode getMode() {
                return Mode.POWER;
            }

            @Override
            public void update() {
                crServo.setPower(power);
            }
        };
    }

    /**
     * Scale a motor's power by a constant
     *
     * @param motor the motor to scale
     * @param scale the scaling factor
     * @return the created MotorEnc
     */
    public static Motor scale(final Motor motor, final double scale) {
        return new Motor() {
            @Override
            public void setPower(double power) {
                motor.setPower(power * scale);
            }

            @Override
            public Mode getMode() {
                return motor.getMode();
            }

            @Override
            public void update() {
                motor.update();
            }

        };
    }

    /**
     * Scale a motor's power and speed by a constant
     *
     * @param motorEnc the motor to scale
     * @param scale    the scaling factor
     * @return the created MotorEnc
     */
    public static Motor scale(final MotorEnc motorEnc, final double scale) {
        return new MotorEnc() {
            @Override
            public void setSpeed(double speed) {
                motorEnc.setSpeed(speed * scale);
            }

            @Override
            public void setPosition(int encoderTarget, double maxCorrectionPower) {
                motorEnc.setPosition(encoderTarget, maxCorrectionPower);
            }

            @Override
            public void resetEncoder() {
                motorEnc.resetEncoder();
            }

            @Override
            public int getEncoderPosition() {
                return motorEnc.getEncoderPosition();
            }

            @Override
            public void setPower(double power) {
                motorEnc.setPower(power * scale);
            }

            @Override
            public Mode getMode() {
                return motorEnc.getMode();
            }

            @Override
            public void update() {
                motorEnc.update();
            }

        };
    }


    /**
     * Scale a motor's power by a function
     *
     * @param motor    the motor to scale
     * @param function the function to scale by
     * @return the created MotorEnc
     */
    public static Motor scale(final Motor motor, final Function function) {
        return new Motor() {
            @Override
            public void setPower(double power) {
                motor.setPower(function.f(power));
            }

            @Override
            public Mode getMode() {
                return motor.getMode();
            }

            @Override
            public void update() {
                motor.update();
            }

        };
    }

    /**
     * Scale a motor's power and speed by a function
     *
     * @param motorEnc the motor to scale
     * @param function the function to scale by
     * @return the created MotorEnc
     */
    public static Motor scale(final MotorEnc motorEnc, final Function function) {
        return new MotorEnc() {
            @Override
            public void setSpeed(double speed) {
                motorEnc.setSpeed(function.f(speed));
            }

            @Override
            public void setPosition(int encoderTarget, double maxCorrectionPower) {
                motorEnc.setPosition(encoderTarget, maxCorrectionPower);
            }

            @Override
            public void resetEncoder() {
                motorEnc.resetEncoder();
            }

            @Override
            public int getEncoderPosition() {
                return motorEnc.getEncoderPosition();
            }

            @Override
            public void setPower(double power) {
                motorEnc.setPower(function.f(power));
            }

            @Override
            public Mode getMode() {
                return motorEnc.getMode();
            }

            @Override
            public void update() {
                motorEnc.update();
            }

        };
    }

    /**
     * @return a motor with an encoder that does nothing
     */
    public static MotorEnc dummyWithEncoder() {
        return new MotorEnc() {
            private Mode mode = Mode.POWER;

            @Override
            public void setSpeed(double speed) {
                mode = Mode.SPEED;
            }

            @Override
            public void setPosition(int encoderTarget, double maxCorrectionPower) {
                mode = Mode.POSITION;
            }

            @Override
            public void resetEncoder() {

            }

            @Override
            public int getEncoderPosition() {
                return 0;
            }

            @Override
            public void setPower(double power) {
                mode = Mode.POWER;
            }

            @Override
            public Mode getMode() {
                return mode;
            }

            @Override
            public void update() {

            }
        };
    }

    /**
     * @return a motor that does nothing
     */
    public static Motor dummyWithoutEncoder() {
        return new Motor() {
            @Override
            public void setPower(double power) {

            }

            @Override
            public Mode getMode() {
                return Mode.POWER;
            }

            @Override
            public void update() {

            }
        };
    }

}
```