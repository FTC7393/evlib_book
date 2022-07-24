TeamColor is an enum that represents the alliance a robot is on. It has 3 possible values: `RED`, `BLUE`, and `UNKNOWN`. To convert a string to a TeamColor, use the `TeamColor.fromString()` method. This method converts the input to capital letters. Then it searches in the string for "RED" and "BLUE". If it doesn't find either one, it then searches for "R" and "B".

TeamColor also has the opposite() method, which can be used to get your opponent's color.

[ftc/electronvolts/util/TeamColor.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/TeamColor.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * This is a class that stores which team you are on.
 */
public enum TeamColor {
    RED,
    BLUE,
    UNKNOWN;

    /**
     * This method converts a String to a TeamColor. Neither color has
     * precedence over the other,
     * which means that when it finds both, it will return UNKNOWN.
     * 
     * @param s the input String
     * @return the TeamColor corresponding to that string
     */
    public static TeamColor fromString(String s) {
    	if (s==null)return UNKNOWN;
        s = s.toUpperCase(); // convert the input to upper case

        boolean foundRed = s.contains("RED"); // look for RED in the input
        boolean foundBlue = s.contains("BLUE"); // look for BLUE in the input

        if (foundRed && !foundBlue) return RED; // if it found RED but not BLUE
        if (foundBlue && !foundRed) return BLUE; // if it found BLUE but not RED
        if (foundBlue && foundRed) return UNKNOWN; // if it found both

        // only case remaining is that neither were found

        boolean foundR = s.contains("R"); // look for R in the input
        boolean foundB = s.contains("B"); // look for B in the input

        if (foundR && !foundB) return RED; // if it found R but not B
        if (foundB && !foundR) return BLUE; // if it found B but not R
        return UNKNOWN; // if it found both or neither.
    }
    
    /**
     * @return the opposite color
     */
    public TeamColor opposite() {
        switch (this) {
            case RED:
                return BLUE;
            case BLUE:
                return RED;
            default:
                return UNKNOWN;
        }
    }
}
```