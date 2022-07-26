In Java, an interface sets requirements for the classes that implement it. The methods on the interface have no code that executes, but they tell what the method name, parameters, and return type will be.

The State interface says that each implementation of the State interface must have an act() method which returns the name of the state to be run next call of the loop method. It can return its own name or the name of another state if the current state is done. It also has to have a getName() method which returns the State's StateName. The state interface is used for all other states, including [BasicAbstractState](BasicAbstractState.md) and [AbstractState](AbstractState.md).

[ftc/electronvolts/statemachine/State.java](https://github.com/FTC7393/state-machine-framework/blob/master/src/ftc/electronvolts/statemachine/State.java)
```java
package ftc.electronvolts.statemachine;

/**
 * This file was made by the electronVolts, FTC team 7393
 *
 * This is the core of the state machine framework. It is the interface that
 * defines the required behavior of all the classes that call themselves a
 * "State". This way anything that receives a State object knows exactly what
 * methods that object has, regardless of the state's internal structure or
 * function.
 */
public interface State {
    /**
     * Act is run in every single loop
     *
     * @return Returns the name of the next state. Returns null to stay
     *         in the current state.
     */
    StateName act();
}
```

The next step in the guide is to write a [Simple State Machine](Simple-State-Machine.md).