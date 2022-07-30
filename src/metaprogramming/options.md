# Options

The unfortunate part about programming for the FTC robot is that to test code, you must build, download, then run; for large codebases this is often a slow process. Sometimes, there will be numbers which are not necessarily fixed, and rebuilding just before every competition and match is infeasible.

This collection of classes gives you a dynamic solution to this problem:
* `ftc.evlib.opmodes.AbstractOptionsOp`
* `ftc.evlib.util.FileUtil`
* `ftc.electronvolts.util.OptionsFile`
* `ftc.electronvolts.util.OptionEntry`

These classes allow you to set variables by running an `AbstractOptionsOp`, then access them later in an unrelated Opmode.

>NOTE: Option setting is an **opt-in** feature. An opmode for setting options does not show up by default.

## Interface

You'll probably want to orient your driver hub to be landscape to use the interface

>TODO: Add picture of driver hub

The table on the right shows a list of option names and their current values.

* To select an option, press dpad-up or dpad-down (navigate up or down) until you reach it (the arrows are on either side of the option).
* For discrete variables (e.g. integers), press the left/right bumpers to modify their values (this change will be reflected on the telemetry log).
* For approximate continuous variables (e.g. floating-points), press the left/right triggers.
    * The depth of the trigger press will be reflected in the rate at which the option is modified (the heavier the touch, the more the variable goes up).
* To save the options, press the start button
    * **The options will not save the opmode finishes but the start button is not pressed**
* To clear changes to the last save, press the reset button
    * **This will irrevocably remove all changes that were not saved**

## Defining Variables

You'll first want to set up two classes: An OpMode extending `AbstractOptionsOp` and an enum class implementing `OptionEntry`. Here is an example:

```java
// ExampleOptions.java

import ftc.electronvolts.util.OptionEntry;

public enum ExampleOptions implements OptionEntry {

    TypeData<?> data;

    @Override
    public TypeData<?> getData {
        return data;
    }

    ExampleOptions(TypeDataBuilder<?> t) {
        data = t.build();
    }

}

// ExampleOptionsOp.java

import com.qualcomm.robotcore.eventloop.opmode.TeleOp;
import ftc.evlib.opmodes.AbstractOptionsOp;

@TeleOp(name = "Example Options Op")
public class ExampleOptionsOp extends AbstractOptionsOp {
    @Override
    protected void setImplementationSpecificDetails() {

        filename = "example_options.json";
        options = ExampleOptions.class;

    }
}
```

These classes would run fine by themselves, but if you try running "Example Options Op" you would see nothing. To create options, you would have to define an entry in `ExampleOptions`. Let's define one that is heavily suited for this task: The alliance color. It's one you'll only know on the field, and will not necessarily have the time to set.

Let's say that you represent your alliance color like so:

```java
// AllianceColor.java

public enum AllianceColor {
    RED,
    BLUE,
}
```

We'll define an option with these properties:
* It is called `ALLIANCE_COLOR`
* It stores the `AllianceColor` enum
* We somehow tipped off the judge, so we're expecting it to mostly be red

Here is the full definition:
```java
ALLIANCE_COLOR(TypeDataBuilder
    .enumType(AllianceColor.class)
    .withFallback(AllianceColor.RED)
)
```

Here it is inserted into `ExampleOptions`:
```java
public enum ExampleOptions implements OptionEntry {

    ALLIANCE_COLOR(TypeDataBuilder
        .enumType(AllianceColor.class)
        .withFallback(AllianceColor.RED)
    )

    TypeData<?> data;

    @Override
    public TypeData<?> getData {
        return data;
    }

    ExampleOptions(TypeDataBuilder<?> t) {
        data = t.build();
    }

}
```

Now if you run "Example Options Op" again, it will show this one option, which you can now change using the bumpers on your controller. Adding more options is as easy as adding more enum variants.

>TODO: add an image

## Fetching Options

After you have set these options, you can programatically access them using the `OptionsFile` class. Admittedly this is not an easy thing to do, and will be the next thing to improve about the option framework.

To get the object containing the options, use:
```java
OptionsFile file = new OptionsFile(FileUtil.getOptionsFile("example_options.json"));
```
The string `"example_options.json"` comes from the `ExampleOptionsOp` defined before.

To get out the `AllianceColor`, access the object like so:
```java
AllianceColor color = (AllianceColor) file.load(ExampleOptions.ALLIANCE_COLOR);
```
The cast is necessary, without it you will have a compile error.
