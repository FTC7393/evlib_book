DeadZone is an interface that tells if a given value is inside the dead-zone. It is not very heavily used in the framework.

[ftc/electronvolts/util/DeadZone.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/DeadZone.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A class that defines a deadzone by returning a boolean as a function of a
 * double
 */
public interface DeadZone {
    boolean isInside(double value);
}
```

DeadZones is a factory class for the DeadZone interface. It has methods that return DeadZones for min and max, center and delta, and combinations of other DeadZone objects.

[ftc/electronvolts/util/DeadZones.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/DeadZones.java)
```java
package ftc.electronvolts.util;

import java.util.Collection;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * A factory class for DeadZone
 */
public class DeadZones {
    /**
     * @param deadZones the list of DeadZones to be combined
     * @return the composite DeadZone
     */
    public static DeadZone any(final Collection<DeadZone> deadZones) {
        return new DeadZone() {
            @Override
            public boolean isInside(double value) {
                for (DeadZone deadZone : deadZones) {
                    if (deadZone.isInside(value)) {
                        return true;
                    }
                }
                return false;
            }
        };
    }

    /**
     * @param deadZones the list of DeadZones to be combined
     * @return the composite DeadZone
     */
    public static DeadZone all(final Collection<DeadZone> deadZones) {
        return new DeadZone() {
            @Override
            public boolean isInside(double value) {
                for (DeadZone deadZone : deadZones) {
                    if (!deadZone.isInside(value)) {
                        return false;
                    }
                }
                return true;
            }
        };
    }

    /**
     * inverts a DeadZone
     *
     * @param deadZone the DeadZone to invert
     * @return the resulting DeadZone
     */
    public static DeadZone not(final DeadZone deadZone) {
        return new DeadZone() {
            @Override
            public boolean isInside(double value) {
                return !deadZone.isInside(value);
            }
        };
    }

    /**
     * a DeadZone that is empty
     *
     * @return the DeadZone
     */
    public static DeadZone noDeadZone() {
        return new DeadZone() {
            @Override
            public boolean isInside(double value) {
                return false;
            }
        };
    }

    /**
     * A DeadZone between min and max
     *
     * @param min the lower edge of the DeadZone
     * @param max the upper edge of the DeadZone
     * @return the DeadZone
     */
    public static DeadZone minMaxDeadzone(final double min, final double max) {
        return new DeadZone() {
            @Override
            public boolean isInside(double value) {
                return (value >= min && value <= max);
            }
        };
    }

    /**
     * deadzone that ranges from center-delta to center+delta
     * center is usually zero so that the deadzone is +/-delta
     *
     * @param center the center of the deadzone
     * @param delta the delta on either side of the deadzone
     * @return the DeadZone
     */
    public static DeadZone deltaDeadZone(double center, double delta) {
        double min = center - delta;
        double max = center + delta;
        return minMaxDeadzone(min, max);
    }
}
```