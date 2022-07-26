[Angle](Angle.md) | [Distance](Distance.md) | [Time](Time.md) | [Velocity](Velocity.md) | **AngularVelocity**

***

**Units: radians, degrees, rotations _PER_ second, (nano|micro|milli)second, minute, hour, day, week, month, year**

The AngularVelocity class stores an amount of angular velocity. It can be created from an [Angle](Angle.md) class and a [Time](Time.md) class, and you can get the value from it in any of the units above. Once the AngularVelocity is created, the value cannot be changed.

AngularVelocity has a few math functions: abs (absolute value), signum (returns the sign), add, subtract, multiply, and divide.

[ftc/electronvolts/util/units/AngularVelocity.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/units/AngularVelocity.java)
```java
package ftc.electronvolts.util.units;


/**
 * This file was made by the electronVolts, FTC team 7393
 * Date Created: 11/16/16
 */

public class AngularVelocity {
    private static final AngularVelocity zero = new AngularVelocity(Angle.zero());
    
    private final Angle angle;
    private final Time time;

    /**
     * Angle per Time
     * 
     * @param angle the Angle object
     * @param time the Time object
     */
    public AngularVelocity(Angle angle, Time time) {
        this.angle = angle;
        this.time = time;
    }

    /**
     * Angle per 1 second
     * 
     * @param angle the Angle object
     */
    private AngularVelocity(Angle angle) {
        this(angle, Time.fromSeconds(1));
    }

    public static AngularVelocity zero() {
        return zero;
    }

    public Angle getAngle(Time time) {
        // angle = angular velocity * time
        return Angle.fromRadians(radiansPerSecond() * time.seconds());
    }

    public Time getTime(Angle angle) {
        // time = angle / angular velocity
        return Time.fromSeconds(angle.radians() / radiansPerSecond());
    }

    public Velocity getVelocity(Distance radius) {
        //velocity = radius * angular velocity
        return new Velocity(Distance.fromMeters(radius.meters() * radiansPerSecond()), Time.fromSeconds(1));
    }

    public AngularVelocity abs() {
        return new AngularVelocity(angle.abs(), time.abs());
    }

    public double signum() {
        return Math.signum(radiansPerSecond());
    }

    /**
     * Adds two AngularVelocity objects together
     * 
     * @param angularVelocity1 the first AngularVelocity
     * @param angularVelocity2 the second AngularVelocity
     * @return the resulting AngularVelocity
     */
    public static AngularVelocity add(AngularVelocity angularVelocity1, AngularVelocity angularVelocity2) {
        return new AngularVelocity(Angle.fromRadians(angularVelocity1.radiansPerSecond() + angularVelocity2.radiansPerSecond()));
    }

    /**
     * Subtracts velocity2 from velocity1
     * 
     * @param angularVelocity1 the first AngularVelocity
     * @param angularVelocity2 the second AngularVelocity
     * @return the resulting AngularVelocity
     */
    public static AngularVelocity subtract(AngularVelocity angularVelocity1, AngularVelocity angularVelocity2) {
        return new AngularVelocity(Angle.fromRadians(angularVelocity1.radiansPerSecond() - angularVelocity2.radiansPerSecond()));
    }

    /**
     * Multiplies an AngularVelocity by a number
     * 
     * @param angularVelocity the AngularVelocity
     * @param number the number to multiply by
     * @return the resulting AngularVelocity
     */
    public static AngularVelocity multiply(AngularVelocity angularVelocity, double number) {
        return new AngularVelocity(Angle.fromRadians(angularVelocity.radiansPerSecond() * number));
    }

    /**
     * Divides an AngularVelocity by a number
     * 
     * @param angularVelocity the AngularVelocity
     * @param number the number to divide by
     * @return the resulting AngularVelocity
     */
    public static AngularVelocity divide(AngularVelocity angularVelocity, double number) {
        return new AngularVelocity(Angle.fromRadians(angularVelocity.radiansPerSecond() * number));
    }

    @Override
    public int hashCode() {
        final int prime = 31;
        int result = 1;
        long temp;
        temp = Double.doubleToLongBits(radiansPerSecond());
        result = prime * result + (int) (temp ^ (temp >>> 32));
        return result;
    }

    @Override
    public boolean equals(Object obj) {
        if (this == obj) return true;
        if (obj == null) return false;
        if (getClass() != obj.getClass()) return false;
        AngularVelocity other = (AngularVelocity) obj;
        if (Double.doubleToLongBits(radiansPerSecond()) != Double.doubleToLongBits(other.radiansPerSecond())) return false;
        return true;
    }

    // get angular velocity in various units
    public double radiansPerNanosecond() {
        return angle.radians() / time.nanoseconds();
    }

    public double radiansPerMicrosecond() {
        return angle.radians() / time.microseconds();
    }

    public double radiansPerMillisecond() {
        return angle.radians() / time.milliseconds();
    }

    public double radiansPerSecond() {
        return angle.radians() / time.seconds();
    }

    public double radiansPerMinute() {
        return angle.radians() / time.minutes();
    }

    public double radiansPerHour() {
        return angle.radians() / time.hours();
    }

    public double radiansPerDay() {
        return angle.radians() / time.days();
    }

    public double radiansPerWeek() {
        return angle.radians() / time.weeks();
    }

    public double radiansPerMonth() {
        return angle.radians() / time.months();
    }

    public double radiansPerYear() {
        return angle.radians() / time.years();
    }

    public double degreesPerNanosecond() {
        return angle.degrees() / time.nanoseconds();
    }

    public double degreesPerMicrosecond() {
        return angle.degrees() / time.microseconds();
    }

    public double degreesPerMillisecond() {
        return angle.degrees() / time.milliseconds();
    }

    public double degreesPerSecond() {
        return angle.degrees() / time.seconds();
    }

    public double degreesPerMinute() {
        return angle.degrees() / time.minutes();
    }

    public double degreesPerHour() {
        return angle.degrees() / time.hours();
    }

    public double degreesPerDay() {
        return angle.degrees() / time.days();
    }

    public double degreesPerWeek() {
        return angle.degrees() / time.weeks();
    }

    public double degreesPerMonth() {
        return angle.degrees() / time.months();
    }

    public double degreesPerYear() {
        return angle.degrees() / time.years();
    }

    public double rotationsPerNanosecond() {
        return angle.rotations() / time.nanoseconds();
    }

    public double rotationsPerMicrosecond() {
        return angle.rotations() / time.microseconds();
    }

    public double rotationsPerMillisecond() {
        return angle.rotations() / time.milliseconds();
    }

    public double rotationsPerSecond() {
        return angle.rotations() / time.seconds();
    }

    public double rotationsPerMinute() {
        return angle.rotations() / time.minutes();
    }

    public double rotationsPerHour() {
        return angle.rotations() / time.hours();
    }

    public double rotationsPerDay() {
        return angle.rotations() / time.days();
    }

    public double rotationsPerWeek() {
        return angle.rotations() / time.weeks();
    }

    public double rotationsPerMonth() {
        return angle.rotations() / time.months();
    }

    public double rotationsPerYear() {
        return angle.rotations() / time.years();
    }

}
```