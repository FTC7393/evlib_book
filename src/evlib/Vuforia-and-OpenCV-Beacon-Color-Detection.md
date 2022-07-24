Vuforia and OpenCV are image processing libraries. [Vuforia](https://www.vuforia.com/) is by [PTC](http://www.ptc.com/) and is built in to the latest version of the FTC app. It has high-level functions for tracking objects and augmented reality. It is very useful for finding the position of the beacon target images.

[OpenCV](http://opencv.org/) is an open source library that is more low-level, and because of this it is possible to write the procedures yourself and optimize them for better performance.

Before you can use this example, you have to:

* Set up OpenCV. A guide is available on our website at ~~http://www.gearbox4h.org/opencv/~~ (no longer exists)
* Create a Vuforia developer account and get a license key from https://developer.vuforia.com/license-manager

This example uses EVLib functionality to:

* Initialize Vuforia
* Find the "Gears" beacon target image (this is controlled by the TeamColor.RED input)
* Crop the beacon out of the frame based on the target image location
* Find the beacon's color

> This example uses guava to initialize lists, which we have left for alternative methods.

[sample/v3/SampleVuforiaOp.java](https://github.com/FTC7393/EVLib/blob/master/sample/v3/SampleVuforiaOp.java)
```java
package org.firstinspires.ftc.teamcode.sample.v3;

import com.google.common.collect.ImmutableList;
import com.qualcomm.robotcore.eventloop.opmode.Autonomous;

import org.firstinspires.ftc.teamcode.R;

import ftc.electronvolts.statemachine.EndConditions;
import ftc.electronvolts.statemachine.StateMachine;
import ftc.electronvolts.statemachine.StateMachineBuilder;
import ftc.electronvolts.statemachine.StateName;
import ftc.electronvolts.util.BasicResultReceiver;
import ftc.electronvolts.util.InputExtractor;
import ftc.electronvolts.util.ResultReceiver;
import ftc.electronvolts.util.TeamColor;
import ftc.electronvolts.util.files.Logger;
import ftc.electronvolts.util.units.Time;
import ftc.evlib.driverstation.Telem;
import ftc.evlib.hardware.config.RobotCfg;
import ftc.evlib.opmodes.AbstractAutoOp;
import ftc.evlib.statemachine.EVStates;
import ftc.evlib.vision.framegrabber.VuforiaFrameFeeder;
import ftc.evlib.vision.processors.BeaconColorResult;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 12/15/16
 */
@Autonomous(name = "SampleVuforiaOp V3")
public class SampleVuforiaOp extends AbstractAutoOp<RobotCfg> {
    //get a key (for free) and paste it below by creating an account at https://developer.vuforia.com/license-manager
    private static final String LICENSE_KEY = "";

    private static final int FRAME_SIZE = 16;
//    private static final int FRAME_SIZE = 8;
//    private static final int FRAME_SIZE = 64;

    private static final int FRAME_WIDTH = 3 * FRAME_SIZE;
    private static final int FRAME_HEIGHT = 4 * FRAME_SIZE;
//    private static final int FRAME_WIDTH = 11 * FRAME_SIZE;
//    private static final int FRAME_HEIGHT = 9 * FRAME_SIZE;

    private enum S implements StateName {
        WAIT_FOR_VUFORIA_INIT,
        FIND_BEACON_COLOR,
        DISPLAY,
        UNKNOWN_BEACON_COLOR,
        BEACON_COLOR_TIMEOUT
    }

    private ResultReceiver<VuforiaFrameFeeder> vuforiaReceiver;
    private ResultReceiver<BeaconColorResult> beaconColorResult;

    @Override
    public StateMachine buildStates() {
        vuforiaReceiver = VuforiaFrameFeeder.initInNewThread(LICENSE_KEY, R.id.cameraMonitorViewId, FRAME_WIDTH, FRAME_HEIGHT);

        StateMachineBuilder b = new StateMachineBuilder(S.WAIT_FOR_VUFORIA_INIT);
        beaconColorResult = new BasicResultReceiver<>();

        b.addEmpty(S.WAIT_FOR_VUFORIA_INIT, b.ts(EndConditions.receiverReady(vuforiaReceiver), S.FIND_BEACON_COLOR));
        b.add(EVStates.findBeaconColorState(S.FIND_BEACON_COLOR, S.DISPLAY, S.UNKNOWN_BEACON_COLOR, S.BEACON_COLOR_TIMEOUT, Time.fromSeconds(10), vuforiaReceiver, beaconColorResult, TeamColor.RED, 4, true));

        b.add(EVStates.displayBeaconColorResult(S.DISPLAY, beaconColorResult));
        b.add(EVStates.displayBeaconColorResult(S.UNKNOWN_BEACON_COLOR, beaconColorResult));
        b.add(EVStates.displayBeaconColorResult(S.BEACON_COLOR_TIMEOUT, beaconColorResult));

        return b.build();
    }

    @Override
    protected RobotCfg createRobotCfg() {
        return new RobotCfg(hardwareMap) {
            @Override
            public void act() {

            }

            @Override
            public void stop() {

            }
        };
    }

    @Override
    protected Logger createLogger() {
        //it will save logs in /FTC/logs/vuforiaOp[....].csv on the robot controller phone
        return new Logger("vuforiaOp", ".csv", ImmutableList.of(
                //each one of these is a column in the log file
                new Logger.Column("state", new InputExtractor<StateName>() {
                    @Override
                    public StateName getValue() {
                        return stateMachine.getCurrentStateName();
                    }
                }),
                new Logger.Column("beaconColor", new InputExtractor<BeaconColorResult>() {
                    @Override
                    public BeaconColorResult getValue() {
                        return beaconColorResult.getValue();
                    }
                })
        ));
    }

    @Override
    protected void setup_act() {
        Telem.displayVuforiaReadiness(vuforiaReceiver);
    }

    @Override
    protected void go() {

    }

    @Override
    protected void act() {
        telemetry.addData("state", stateMachine.getCurrentStateName());
        Telem.displayVuforiaReadiness(vuforiaReceiver);
    }

    @Override
    protected void end() {

    }
}
```

This concludes the tutorial for now. Check back later for more updates and sample code, or go on to
the [Features](Features.md) page for an index of other things you can do with EVLib.