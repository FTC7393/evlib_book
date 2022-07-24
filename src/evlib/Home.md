Welcome to the EVLib wiki!

EVLib is designed to assist in the development of very powerful FTC autonomous programs. It provides helper mechanisms for a variety of hardware, like servo speed control.

For starters, check [this page](Importing-Into-Your-Project.md) for instructions on importing into your project.

## Features
EVLib includes the [state-machine-framework](https://github.com/FTC7393/state-machine-framework/wiki), so all the features of the state-machine-framework can be used from EVLib.

* [robot configuration](Robot-Configuration.md)
* [the Motor interfaces](Motor-and-MotorEnc.md)
* [creating motors and continuous servos](Creating-Motors.md)
* [grouping motors](NMotors.md)
* [driving mecanum wheels](Mecanum-Wheels.md)
* [control of mecanum wheels](Mecanum-Control.md)
* [servo configuration](Servo-Config.md)
* [servo speed control](Servo-Speed-Control.md)
* [servo presets](Servo-Presets.md)
* [abstract opmodes](AbstractOp.md)
* [abstract teleop](AbstractTeleOp.md)
* [abstract autonomous](AbstractAutoOp.md)
* [tuning the servo presets](AbstractServoTuneOp.md)
* [selecting options for autonomous](AbstractOptionsOp.md)
* [gamepad button edge detection and joystick scaling](GamepadManager.md)
* [EVStates factory class](EVStates.md)
* [EVEndConditions factory class](EVEndConditions.md)
* [EVStateMachineBuilder](EVStateMachineBuilder.md)
* [analog sensors](Analog-Sensors.md)
* [digital sensors](Digital-Sensors.md)
* [frame grabbing with OpenCV](OpenCV-Frame-Grabbing.md)
* [image processing with OpenCV](OpenCV-Image-Processing.md)
* [line sensor](CalibratedLineSensor.md)
* [double line sensor](DoubleLineSensor.md)
* [color sensor](ColorSensor.md)
* [line finder](LineFinder.md)
* [averaging analog sensors](AveragedSensor.md)
* [global telemetry](Telem.md)
* [step timer for logging](StepTimer.md)
* [file utilities](FileUtil.md)
* [[LineSensorArray]] that uses the i2c [SparkFun Line Follower Array](https://www.sparkfun.com/products/13582)
* [frame grabbing with Vuforia](Vuforia-Frame-Grabbing.md)
* [image processing with Vuforia](Vuforia-and-OpenCV-Beacon-Color-Detection.md)
* [[Joystick Recorder]] for "playback" in autonomous

## Planned Features
* [[TaskWeb]] to manage executing and moving between multiple tasks on the field
* [[Particle Detection]] for autonomous recollection
* [[Position Tracker]] to read encoders for position info and return to a set location after driving

This project depends on our [state-machine-framework](https://github.com/FTC7393/state-machine-framework/) library. Click [here](https://github.com/FTC7393/state-machine-framework/wiki) for the state-machine-framework wiki.