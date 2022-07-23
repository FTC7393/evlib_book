The following is an example of how the Logger and [[FileUtil]] classes are used together in an OpMode to log values. The logs will appear in the phone storage in the FTC/logs folder.

```java
package org.firstinspires.ftc.teamcode.yr2016.opmodes;

import com.google.common.collect.ImmutableList;
import com.qualcomm.robotcore.eventloop.opmode.OpMode;
import com.qualcomm.robotcore.eventloop.opmode.TeleOp;

import ftc.electronvolts.util.InputExtractor;
import ftc.electronvolts.util.files.Logger;
import ftc.evlib.util.FileUtil;

/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 1/13/17
 */

@TeleOp(name = "LogTestOp")
public class LogTestOp extends OpMode {
    Logger logger = new Logger("log", ".csv", ImmutableList.of(
            new Logger.Column("gamepad1.left_stick_y", new InputExtractor<Float>() {
                @Override
                public Float getValue() {
                    return gamepad1.left_stick_y;
                }
            }),
            new Logger.Column("gamepad1.right_stick_y", new InputExtractor<Float>() {
                @Override
                public Float getValue() {
                    return gamepad1.right_stick_y;
                }
            })
    ));

    @Override
    public void init() {

    }

    @Override
    public void start() {
        logger.start(FileUtil.getLogsDir());
    }

    @Override
    public void loop() {
        logger.act();
    }

    @Override
    public void stop() {
        logger.stop();
    }
}
```

If you want to learn more about the library, check out [[Robot Configuration]].