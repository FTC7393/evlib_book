Converters are the method that [[OptionsFile]] uses to convert values to and from strings to store in a file.

Converter defines the interface for actually converting something to and from a string.

[ftc/electronvolts/util/files/Converter.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/files/Converter.java)
```java
package ftc.electronvolts.util.files;

/**
 * This file was made by the electronVolts, FTC team 7393
 * 
 * This interface defines a conversion from an object of a certain type to a
 * String and back again
 * 
 * @see Converters
 */
public interface Converter<T> {
    /**
     * Convert an object to a String
     * 
     * @param object the object
     * @return a String representing the object
     */
    String toString(T object);

    /**
     * Convert a String to an object
     * 
     * @param string the String
     * @return the object
     */
    T fromString(String string);
}
```

Converters defines a group of Converter objects for a variety of types of objects. Implementations of Converters are passed to [[OptionsFile]] to allow it to convert different types.

[ftc/electronvolts/util/files/Converters.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/files/Converters.java)
```java
package ftc.electronvolts.util.files;

/**
 * This file was made by the electronVolts, FTC team 7393
 * 
 * Implementations of this interface hold a Map of Converter objects of
 * different types
 * 
 * @see Converter
 */
public interface Converters {
    /**
     * Get a Converter for a certain class
     * 
     * @param clazz the class that the converter converts
     * @return the Converter
     */
    <T> Converter<T> getConverter(Class<T> clazz);
}
```

BasicConverters is the most basic implementation of Converters that has Converter objects for Boolean, Byte, Char, Short, Integer, Long, Float, Double, and String.

[ftc/electronvolts/util/files/BasicConverters.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/files/BasicConverters.java)
```java
package ftc.electronvolts.util.files;

import java.util.HashMap;
import java.util.Map;

/**
 * This file was made by the electronVolts, FTC team 7393
 * 
 * A basic implementation of Converters that includes Converter objects
 * for all the class versions of the primitive types, and Strings
 * 
 * @see Converters
 * @see Converter
 */
public class BasicConverters implements Converters {
    protected static final Map<Class<? extends Object>, Converter<?>> converterMap = new HashMap<>();
    static {
        converterMap.put(Boolean.class, new Converter<Boolean>() {

            @Override
            public String toString(Boolean object) {
                return object.toString();
            }

            @Override
            public Boolean fromString(String string) {
                return Boolean.valueOf(string);
            }
        });
        converterMap.put(Byte.class, new Converter<Byte>() {

            @Override
            public String toString(Byte object) {
                return object.toString();
            }

            @Override
            public Byte fromString(String string) {
                return Byte.valueOf(string);
            }
        });
        converterMap.put(Character.class, new Converter<Character>() {

            @Override
            public String toString(Character object) {
                return object.toString();
            }

            @Override
            public Character fromString(String string) {
                return string.charAt(0);
            }
        });
        converterMap.put(Short.class, new Converter<Short>() {

            @Override
            public String toString(Short object) {
                return object.toString();
            }

            @Override
            public Short fromString(String string) {
                return Short.valueOf(string);
            }
        });
        converterMap.put(Integer.class, new Converter<Integer>() {

            @Override
            public String toString(Integer object) {
                return object.toString();
            }

            @Override
            public Integer fromString(String string) {
                return Integer.valueOf(string);
            }
        });
        converterMap.put(Long.class, new Converter<Long>() {

            @Override
            public String toString(Long object) {
                return object.toString();
            }

            @Override
            public Long fromString(String string) {
                return Long.valueOf(string);
            }
        });
        converterMap.put(Float.class, new Converter<Float>() {

            @Override
            public String toString(Float object) {
                return object.toString();
            }

            @Override
            public Float fromString(String string) {
                return Float.valueOf(string);
            }
        });
        converterMap.put(Double.class, new Converter<Double>() {

            @Override
            public String toString(Double object) {
                return object.toString();
            }

            @Override
            public Double fromString(String string) {
                return Double.valueOf(string);
            }
        });

        converterMap.put(String.class, new Converter<String>() {

            @Override
            public String toString(String object) {
                return object;
            }

            @Override
            public String fromString(String string) {
                return string;
            }
        });

    }

    private static Converters INSTANCE = new BasicConverters();

    protected BasicConverters() {
    }

    public static Converters getInstance() {
        return INSTANCE;
    }

    @Override
    public <T> Converter<T> getConverter(Class<T> clazz) {
        return (Converter<T>) converterMap.get(clazz);
    }

}
```

UtilConverters extends BasicConverters, inheriting all its Converter objects. It adds Converters for TeamColor, Vector2D, and Vector3D.

[ftc/electronvolts/util/files/UtilConverters.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/files/UtilConverters.java)
```java
package ftc.electronvolts.util.files;

import java.util.regex.Matcher;
import java.util.regex.Pattern;

import ftc.electronvolts.util.TeamColor;
import ftc.electronvolts.util.Vector2D;
import ftc.electronvolts.util.Vector3D;

/**
 * This file was made by the electronVolts, FTC team 7393
 * 
 * An implementation of Converters that extends BasicConverters and adds
 * Converter objects for some of the utility classes
 * 
 * @see BasicConverters
 */
public class UtilConverters extends BasicConverters {
    private static final String DECIMAL_NUMBER = "([0-9\\.\\-]+)";
    private static final String COMMA = " *, *";
    static {
        converterMap.put(TeamColor.class, new Converter<TeamColor>() {

            @Override
            public String toString(TeamColor object) {
                return object.name();
            }

            @Override
            public TeamColor fromString(String string) {
                return TeamColor.fromString(string);
            }
        });
        converterMap.put(Vector2D.class, new Converter<Vector2D>() {

            @Override
            public String toString(Vector2D object) {
                return object.toString();
            }

            @Override
            public Vector2D fromString(String string) {
                Pattern pattern = Pattern.compile("\\(" + DECIMAL_NUMBER + COMMA + DECIMAL_NUMBER + "\\)");
                Matcher matcher = pattern.matcher(string);
                if (matcher.find()) {
                    return new Vector2D(Double.valueOf(matcher.group(1)), Double.valueOf(matcher.group(2)));
                } else {
                    return null;
                }
            }
        });
        converterMap.put(Vector3D.class, new Converter<Vector3D>() {

            @Override
            public String toString(Vector3D object) {
                return object.toString();
            }

            @Override
            public Vector3D fromString(String string) {
                Pattern pattern = Pattern.compile("\\(" + DECIMAL_NUMBER + COMMA + DECIMAL_NUMBER + COMMA + DECIMAL_NUMBER + "\\)");
                Matcher matcher = pattern.matcher(string);
                if (matcher.find()) {
                    return new Vector3D(Double.valueOf(matcher.group(1)), Double.valueOf(matcher.group(2)), Double.valueOf(matcher.group(3)));
                } else {
                    return null;
                }
            }
        });
    }

    private static final Converters INSTANCE = new UtilConverters();

    public static Converters getInstance() {
        return INSTANCE;
    }

    protected UtilConverters() {
        super();
    }
}
```