ResultReceiver is an interface that allows you to pass results between threads.

[ftc/electronvolts/util/ResultReceiver.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/ResultReceiver.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * Used to transmit information between threads
 */
public interface ResultReceiver<T> {
    /**
     * @return whether or not the value been stored yet
     */
    boolean isReady();

    /**
     * @return the value
     */
    T getValue();

    /**
     * @param value the value to be set
     */
    void setValue(T value);

    /**
     * reset the value and mark as not ready
     */
    void clear();
}
```

BasicResultReceiver is the simplest implementation of the ResultReceiver interface.

[ftc/electronvolts/util/BasicResultReceiver.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/util/BasicResultReceiver.java)
```java
package ftc.electronvolts.util;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * The most basic implementation of ResultReceiver
 */
public class BasicResultReceiver<T> implements ResultReceiver<T> {
    private T value = null;
    private boolean ready = false;

    @Override
    public boolean isReady() {
        return ready;
    }

    @Override
    public T getValue() {
        return value;
    }

    @Override
    public void setValue(T value) {
        this.value = value;
        ready = true;
    }

    @Override
    public void clear() {
        ready = false;
        value = null;
    }
}
```