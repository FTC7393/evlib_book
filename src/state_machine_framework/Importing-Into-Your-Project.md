Using the state machine framework in the your project is extremely simple!

1. Switch Android Studio into project view (or use a file explorer/finder to complete the following step).
2. Add the latest jar file (`state-machine-framework-<latest version>.jar`) and sources (`state-machine-framework-sources-<latest version>.jar`) from the [/build](https://github.com/FTC7393/state-machine-framework/tree/master/build) folder to the  `<app name>/libs/` folder in your project.
3. Add a line to the gradle build dependencies (inside  `<app name>/FtcRobotController/build.release.gradle` in the 'dependencies' section): `compile files('../libs/state-machine-framework-<latest version>.jar')` This will include the jar file as a dependency for the project.
4. Rebuild the gradle project.

To get started on using the framework, you can see the page for the [[State]] interface.