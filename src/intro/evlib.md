# The ElectronVolts Library

## What it is:

- A tool to make writing FTC code faster
- A collection of classes containing boilerplate code
- Stable
- What this book is trying to get you to use

## What it isn't:

- Magic
- Required

The ElectronVolts library contains a lot of helpful stuff, but only if it is used correctly. It's a layer of abstraction on top of the Qualcomm framework, allowing you to write your code in a stable way.

However, learning *yet another* layer of abstraction is pretty difficult, and if the FTC library suits your purposes, it's what you should stick with. But if you struggle with these problems:

- Value fudging
- Untraceable logic
- Not knowing what's going on
- Insanity

TODO: Add concrete example of a problem

Maybe this library is a good fit for you. If it isn't, then you might want to stop here and read the FTC provided documentation instead.

## Library Structure

The library is split up into two parts: `electronvolts` and `evlib`. The `electronvolts` code consists of helper classes which are used to make `evlib`, which is the part of the library you will probably be using most often. Regardless of what you use, you will need to download both and put them inside the same directory.

In the previous tutorial, you saw the `LinearOpMode`. There are many others that Qualcomm provides, and can be used along with the ElectronVolts library, but the ElectronVolts library contains its own, preferred Operation Modes, which you will see in the next chapter.
