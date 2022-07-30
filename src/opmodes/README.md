# Operation Modes

Operation Modes are the entry point for your code. Each of them contain "runtime stages", which are called at a certain point during the OpMode's lifetime.

For example, as soon as you press the init button on the driver station, the `setup` method (which is in almost all of our opmodes) is run. If you want to run code during an opmode, you should do it in one of those stages.
