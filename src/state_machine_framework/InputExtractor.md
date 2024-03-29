InputExtractor is an interface that can be any type. It has one method, getValue(), that returns a value of that type. It is used where an input must be passed in, but that input needs to be generic.

InputExtractor is used to get inputs for a variety of uses:

* [Logging](Logger.md)
* [Digital edge detection](DigitalInputEdgeDetector.md)
* [Analog scaling](AnalogInputScaler.md)
* Comparing inputs in [EndConditions](EndConditions.md)

[ftc/electronvolts/util/InputExtractor.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/InputExtractor.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * This class is used to extract a static method and store it in an object
 */
public interface InputExtractor<Type> {
    /**
     * @return the value from wherever the InputExtractor got it
     */
    Type getValue();
}
```

The InputExtractors factory class has methods for adding, subtracting, multiplying, dividing, etc. two InputExtractor objects or one InputExtractor object and a constant. It also has methods for boolean operators such as and, or, xor, etc.

[ftc/electronvolts/util/InputExtractors.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/InputExtractors.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 * 
 * This is the factory class for InputExtractor
 */
public class InputExtractors {
    private static final InputExtractor<Double> ZERO = constant(0.0);

    /**
     * @return an InputExtractor<Double> that always returns 0
     */
    public static InputExtractor<Double> zero() {
        return ZERO;
    }

    /**
     * @param value the value to return
     * @return an InputExtractor that always returns the specified value
     */
    public static <T> InputExtractor<T> constant(final T value) {
        return new InputExtractor<T>() {
            @Override
            public T getValue() {
                return value;
            }
        };
    }

    /**
     * Multiply an InputExtractor<Double> by a constant
     * 
     * @param inputExtractor the InputExtractor<Double> to multiply
     * @param value the constant to multiply by
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> multiply(final InputExtractor<Double> inputExtractor, final double value) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor.getValue() * value;
            }
        };
    }

    /**
     * Multiply an InputExtractor<Double> by a constant
     * 
     * @param value the constant to multiply by
     * @param inputExtractor the InputExtractor<Double> to multiply
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> multiply(final double value, final InputExtractor<Double> inputExtractor) {
        return multiply(value, inputExtractor);
    }

    /**
     * Multiply an InputExtractor<Double> by another InputExtractor<Double>
     * 
     * @param inputExtractor1 the first InputExtractor<Double>
     * @param inputExtractor2 the second InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> multiply(final InputExtractor<Double> inputExtractor1, final InputExtractor<Double> inputExtractor2) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor1.getValue() * inputExtractor2.getValue();
            }
        };
    }

    /**
     * Divide an InputExtractor<Double> by a constant. Equivalent to multiplying
     * by the reciprocal of the constant
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @param value the constant
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> divide(final InputExtractor<Double> inputExtractor, final double value) {
        return multiply(inputExtractor, 1 / value);
    }

    /**
     * Divide an InputExtractor<Double> by another InputExtractor<Double>
     * 
     * @param inputExtractor1 the numerator
     * @param inputExtractor2 the denominator
     * @return
     */
    public static InputExtractor<Double> divide(final InputExtractor<Double> inputExtractor1, final InputExtractor<Double> inputExtractor2) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor1.getValue() / inputExtractor2.getValue();
            }
        };
    }

    /**
     * Divide a constant by an InputExtractor<Double>
     * 
     * @param value the constant
     * @param inputExtractor the InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> divide(final double value, final InputExtractor<Double> inputExtractor) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return value / inputExtractor.getValue();
            }
        };
    }

    /**
     * Add a constant to an InputExtractor<Double>
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @param value the constant
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> add(final InputExtractor<Double> inputExtractor, final double value) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor.getValue() + value;
            }
        };
    }

    /**
     * Add a constant to an InputExtractor<Double>
     * 
     * @param value the constant
     * @param inputExtractor the InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> add(final double value, final InputExtractor<Double> inputExtractor) {
        return add(inputExtractor, value);
    }

    /**
     * Add two InputExtractor<Double> objects together
     * 
     * @param inputExtractor1 the first InputExtractor<Double>
     * @param inputExtractor2 the second InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> add(final InputExtractor<Double> inputExtractor1, final InputExtractor<Double> inputExtractor2) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor1.getValue() + inputExtractor2.getValue();
            }
        };
    }

    /**
     * Subtract a constant from an InputExtractor<Double>
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @param value the constant to subtract
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> subtract(final InputExtractor<Double> inputExtractor, final double value) {
        return add(inputExtractor, -value);
    }

    /**
     * Subtract one InputExtractor<Double> from another InputExtractor<Double>
     * 
     * @param inputExtractor1 the first InputExtractor<Double>
     * @param inputExtractor2 the second InputExtractor<Double> (will be
     *            subtracted from the first)
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> subtract(final InputExtractor<Double> inputExtractor1, final InputExtractor<Double> inputExtractor2) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return inputExtractor1.getValue() - inputExtractor2.getValue();
            }
        };
    }

    /**
     * Subtract an InputExtractor<Double> from a constant
     * 
     * @param value the constant
     * @param inputExtractor the InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> subtract(final double value, final InputExtractor<Double> inputExtractor) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return value - inputExtractor.getValue();
            }
        };
    }

    /**
     * Get the absolute value of an InputExtractor<Double>
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> absolute(final InputExtractor<Double> inputExtractor) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return Math.abs(inputExtractor.getValue());
            }
        };
    }

    /**
     * Get the negative of an InputExtractor<Double>
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> negative(final InputExtractor<Double> inputExtractor) {
        return multiply(inputExtractor, -1);
    }

    /**
     * Apply a scaling function to an InputExtractor<Double>
     * 
     * @param inputExtractor the InputExtractor<Double>
     * @param function the Function to scale by
     * @return the created InputExtractor<Double>
     */
    public static InputExtractor<Double> function(final InputExtractor<Double> inputExtractor, final Function function) {
        return new InputExtractor<Double>() {
            @Override
            public Double getValue() {
                return function.f(inputExtractor.getValue());
            }
        };
    }

    /**
     * Get the inverse of an InputExtractor<Boolean>
     * @param inputExtractor the InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> not(final InputExtractor<Boolean> inputExtractor) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return !inputExtractor.getValue();
            }
        };
    }

    /**
     * Apply the "and" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> and(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return inputExtractor1.getValue() && inputExtractor2.getValue();
            }
        };
    }

    /**
     * Apply the "nand" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> nand(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return !(inputExtractor1.getValue() && inputExtractor2.getValue());
            }
        };
    }

    /**
     * Apply the "or" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> or(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return inputExtractor1.getValue() || inputExtractor2.getValue();
            }
        };
    }

    /**
     * Apply the "nor" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> nor(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return !(inputExtractor1.getValue() || inputExtractor2.getValue());
            }
        };
    }

    /**
     * Apply the "xor" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> xor(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return inputExtractor1.getValue() ^ inputExtractor2.getValue();
            }
        };
    }

    /**
     * Apply the "xnor" operator to 2 InputExtractor<Boolean> objects
     * @param inputExtractor1 the first InputExtractor<Boolean>
     * @param inputExtractor2 the second InputExtractor<Boolean>
     * @return the created InputExtractor<Boolean>
     */
    public static InputExtractor<Boolean> xnor(final InputExtractor<Boolean> inputExtractor1, final InputExtractor<Boolean> inputExtractor2) {
        return new InputExtractor<Boolean>() {
            @Override
            public Boolean getValue() {
                return !(inputExtractor1.getValue() ^ inputExtractor2.getValue());
            }
        };
    }
}
```