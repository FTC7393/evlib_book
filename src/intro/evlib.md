# The ElectronVolts Library

#### What it is:

- A tool for writing FTC code faster
- Classes containing boilerplate code
- What this book is trying to get you to use

#### What it isn't:

- Magic
- Required

The ElectronVolts library is some code on top of the Qualcomm framework, allowing you to write your code in a stable way. However, learning *yet another* library is pretty difficult, and if the FTC library suits your purposes, it's what you should stick with. Evlib contains helpful stuff, but it only helps if used correctly. But if you struggle with these problems:

- Value fudging
- Untraceable logic
- Not knowing what's going on
- Insanity

TODO: Add concrete example of a problem

Maybe this library is a good fit for you. If you find it isn't, then you should stop and read the FTC provided documentation instead.

## Library Structure

The library is split up into two parts: `electronvolts` and `evlib`. The `electronvolts` code consists of helper classes which are used to make `evlib`, which is the part of the library you will be using most often. Regardless of what you use, you will need to download both and put them inside the same directory.

In the previous tutorial, you saw the `LinearOpMode`. There are many others that Qualcomm provides, and can be used along with the ElectronVolts library, but the ElectronVolts library contains its own, preferred Operation Modes, which you will see in the next chapter.
