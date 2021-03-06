MatchTimer keeps track of the current time relative to the start of a match. When you create it, you pass in the length of the match in milliseconds. Then you call the start() method when the match starts. You have to call the update() method every loop.

There are several useful methods available:

* getDeltaTime() returns the time since the last update()
* getElapsedTime() returns the time since the start()
* getTimeLeft() returns the time until the match is over
* isMatchOver() returns true if the match is over
* isMatchJustOver() returns true the first time the match is over when it is called
* getMatchLengthMillis() returns the length of the match in milliseconds
* getStartTime() returns the time the start() method was called
* getNow() returns the last time update() was called

[ftc/electronvolts/util/MatchTimer.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/MatchTimer.java)
```java
package ftc.electronvolts.util;

import ftc.electronvolts.util.units.Time;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A simple class for keeping track of elapsed time
 */
public class MatchTimer {
    private final long matchLengthMillis;
    private long startTime, deltaTime, now;
    private boolean hasStopped = false;

    /**
     * The match-timer needs to know how long the match will last.
     *
     * @param matchLengthMillis the length of the match. If negative, the match
     *            continues indefinitely
     */
    public MatchTimer(long matchLengthMillis) {
        this.matchLengthMillis = matchLengthMillis;
    }

    public MatchTimer(Time matchLength) {
        if (matchLength == null) {
            this.matchLengthMillis = -1;
        } else {
            this.matchLengthMillis = (long) matchLength.milliseconds();
        }
    }

    /**
     * Start the match timer. Should be called in init method of project.
     */
    public void start() {
        startTime = System.currentTimeMillis();
        now = startTime;
        deltaTime = 0;

        hasStopped = false;
    }

    /**
     * Update the match timer. Must be called every loop.
     *
     * @return the time between the last call of update() and now
     */
    public long update() {
        long previousTime = now;
        now = System.currentTimeMillis();
        deltaTime = now - previousTime;
        previousTime = now;
        return deltaTime;
    }

    /**
     * Get the time since the last update.
     *
     * @return time since last update
     */
    public long getDeltaTime() {
        return deltaTime;
    }

    /**
     * Get the total time that the match has been going
     *
     * @return total time that the match has been going
     */
    public long getElapsedTime() {
        return now - startTime;
    }

    /**
     * Get the time left in a match
     *
     * @return the time left in a match
     */
    public long getTimeLeft() {
        return matchLengthMillis - getElapsedTime();
    }

    /**
     * Return if the match is over
     *
     * @return returns true if the match is over
     */
    public boolean isMatchOver() {
        return matchLengthMillis >= 0 && now - startTime >= matchLengthMillis;
    }

    /**
     * @return if this is the first loop that the match has been over
     */
    public boolean isMatchJustOver() {
        if (isMatchOver() && !hasStopped) {
            hasStopped = true;
            return true;
        }
        return false;
    }

    /**
     * @return the length of the match in milliseconds
     */
    public long getMatchLengthMillis() {
        return matchLengthMillis;
    }

    /**
     * @return the time the match started in milliseconds
     */
    public long getStartTime() {
        return startTime;
    }

    /**
     * @return the time of the last call to update()
     */
    public long getNow() {
        return now;
    }
}
```