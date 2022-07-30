# Base Operation Mode

The class `ftc.evlib.opmodes.AbstractOp` represents the most basic conventional OpMode. It allows you to make an opmode that is linked to a [Robot Configuration](Robot-Configuration.md). This class can be used directly by extending it, but it is recommended that you extend [AbstractTeleOp](AbstractTeleOp.md) or [AbstractAutoOp](AbstractAutoOp.md) instead to take advantage of their features as well as these.

`AbstractOp` takes care of the following:

* Timing the match
* Logging values to a file
* Updating the servos
* Giving global access to the telemetry
* Storing the robotCfg

### Runtime stages

In order to use the `AbstractOp`, you must extend and implement the following methods, representing stages that the robot goes through during its operation. The OpMode moves through seven runtime stages, three of which are utility stages. Here they are in order:

1. `setup`: This method is run once when the init button is pressed.
2. `setup_act`: After `setup` has run, this method is run in a loop, which exits when the play button is pressed. This method is guaranteed to not stop before it completes.
3. `go`: This method is run once at the time when the play button is pressed.

4. `pre_act`: This method is always run before every time `act` runs.
5. `act`: After `go` has run, this method is run in a loop until the OpMode is stopped by the Driver Station or by the game time.
6. `post_act`: This method is always run after every time `act` runs.

7. `end`: This method is run once after the OpMode is stopped by the Driver Station or by the game timer.

You usually don't deal with all of these directly, since some such as `pre_act` and `post_act` are used for supporting more specific OpModes. Others, such as `setup` and `act`, are usually meant for you to implement.

You can find the definition of this class at [ftc/evlib/opmodes/AbstractOp.java](https://github.com/FTC7393/EVLib/blob/master/EVLib/src/main/java/ftc/evlib/opmodes/AbstractOp.java)
