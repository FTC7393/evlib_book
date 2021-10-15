# Base Operation Mode

The class `evlib.opmodes.AbstractOp` represents the most basic OpMode used by ElectronVolts. It moves through seven runtime stages and three utility stages.

Here are the seven runtime stages, in order:
- `setup`: This method is run once when the init button is pressed.
- `setup_act`: After `setup` has run, this method is run in a loop, which exits when the play button is pressed. This method is guaranteed to not stop before it's gotten to the end, if it started.
- `go`: This method is run once at the time when the play button is pressed.

- `pre_act`: This method is always run before every time `act` runs.
- `act`: After `go` has run, this method is run in a loop until the OpMode is stopped by the Driver Station or by the game time.
- `post_act`: This method is always run after every time `act` runs.

- `end`: This method is run once after the OpMode is stopped by the Driver Station or by the game timer.

You usually don't need to deal with all of these directly, since those such as `pre_act` and `post_act` are meant for the the creation of custom OpModes. Others, such as `setup` and `act`, are usually for you to use.
