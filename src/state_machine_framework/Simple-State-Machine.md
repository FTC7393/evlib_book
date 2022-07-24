Let's say you have a state machine drawing that looks like this:
```
 _______        ____________        ______
|       \      |            |      /      \
| START  \ ==> |  wait for  | ==> |  STOP  |
|        /     |  1 second  |     |        |
|_______/      |____________|      \______/
```

To create a state machine based on this diagram, first create an enum that implements [[StateName|The State Interface]] with your list of states:

```java
enum S implements StateName {
    WAIT,
    STOP
};
```

Then create a [[StateMachineBuilder]], giving it the state to start with:
```java
StateMachineBuilder b = new StateMachineBuilder(S.WAIT);
```

Add all the states to the builder:
```java
//add a wait state named WAIT for 1000 milliseconds, then go to the state named STOP
b.addWait(S.WAIT, S.STOP, 1000);

//add a stop state named STOP
b.addStop(S.STOP);
```

Build the state machine and save it to a variable.
```java
StateMachine stateMachine = b.build();
```

Add the following to a loop method to run the state machine:
```java
stateMachine.act();

//use this for the FTC app:
telemetry.addData("State", stateMachine.getCurrentStateName());

//use this for other usages:
System.out.println("State: " + stateMachine.getCurrentStateName());
```

On the telemetry of the FTC app, or the console in other applications, this will be displayed:

```
State: WAIT
State: WAIT
State: WAIT
State: WAIT
...
State: WAIT
State: WAIT
State: STOP
State: STOP
State: STOP
State: STOP
...
```

Congratulations! You made your first state machine!

The next step is to create [[Custom States]].