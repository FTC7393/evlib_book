EVLib includes the [state-machine-framework](https://github.com/FTC7393/state-machine-framework/wiki), so all the features of the state-machine-framework can be used from EVLib.

## Features
* [[robot configuration|Robot Configuration]]
* [[the Motor interfaces|Motor and MotorEnc]]
* [[creating motors and continuous servos|Creating Motors]]
* [[grouping motors|NMotors]]
* [[driving mecanum wheels|Mecanum Wheels]]
* [[control of mecanum wheels|Mecanum Control]]
* [[servo configuration|Servo Config]]
* [[servo speed control|Servo Speed Control]]
* [[servo presets|Servo Presets]]
* [[abstract opmodes|AbstractOp]]
* [[abstract teleop|AbstractTeleOp]]
* [[abstract autonomous|AbstractAutoOp]]
* [[tuning the servo presets|AbstractServoTuneOp]]
* [[selecting options for autonomous|AbstractOptionsOp]]
* [[gamepad button edge detection and joystick scaling|GamepadManager]]
* [[EVStates factory class|EVStates]]
* [[EVEndConditions factory class|EVEndConditions]]
* [[EVStateMachineBuilder|EVStateMachineBuilder]]
* [[analog sensors|Analog Sensors]]
* [[digital sensors|Digital Sensors]]
* [[frame grabbing with OpenCV|OpenCV Frame Grabbing]]
* [[image processing with OpenCV|OpenCV Image Processing]]
* [[line sensor|CalibratedLineSensor]]
* [[double line sensor|DoubleLineSensor]]
* [[color sensor|ColorSensor]]
* [[line finder|LineFinder]]
* [[averaging analog sensors|AveragedSensor]]
* [[global telemetry|Telem]]
* [[step timer for logging|StepTimer]]
* [[file utilities|FileUtil]]
* [[LineSensorArray]] that uses the i2c [SparkFun Line Follower Array](https://www.sparkfun.com/products/13582)
* [[frame grabbing with Vuforia|Vuforia Frame Grabbing]]
* [[image processing with Vuforia|Vuforia and OpenCV Beacon Color Detection]]
* [[Joystick Recorder]] for "playback" in autonomous

## Planned Features
* [[TaskWeb]] to manage executing and moving between multiple tasks on the field
* [[Particle Detection]] for autonomous recollection
* [[Position Tracker]] to read encoders for position info and return to a set location after driving