# Intro

Building working, successful, and long-lasting code is hard. Moving requirements and new features make every year a surgical process, carefully transplanting what code warks and cutting off what doesn't. Fortunately for FTC programmers, libraries are here to provide some stability and a more reasonable starting point.

This tutorial teaches two libraries together: The Qualcomm framework that all FTC teams are *required* to use, and the ElectronVolts library written by Team 7393, a step above the Qualcomm framework. However, this tutorial leans heavily into the ElectronVolts library. This book is *not* meant to teach how to do text-based programming, so if you don't know any, you should go study up on Java. Alternatively, if you've already know a curly-brace/c-like language, then it might work out for you to just wing it. This also isn't meant to be a primer on how to use the robot apps; that can be found on the FTC [github wiki](https://github.com/FIRST-Tech-Challenge/FtcRobotController/wiki). 

# The ElectronVolts Library

#### What it is:

- A tool for writing FTC code faster
- Classes for reducing boilerplate code
- What this book is trying to get you to use

#### What it isn't:

- Magic
- Required for FTC participation

The ElectronVolts library is some code on top of the FTC framework, assisting you with writing code legibly. However, learning *yet another* framework is a time sink, and if the FTC library suits your purposes, it's what you should stick with. Evlib contains helpful stuff, but it only helps if used correctly. You should weigh the costs and benefits of including this new framework, and weigh these problems:

- Value fudging
- Untraceable logic
- Insanity

## Library Structure

The library is split up into two parts: `electronvolts` and `evlib`. The `electronvolts` code consists of helper classes which support `evlib`, is the part of the library you will be using most often.

In the following tutorials, you'll see the `LinearOpMode`. There are many other items that Qualcomm provides, and can be used along with the ElectronVolts library, but the ElectronVolts library contains its own, preferred Operation Modes, which you will see in the [A Starter Program](../starter/program/README.md).

