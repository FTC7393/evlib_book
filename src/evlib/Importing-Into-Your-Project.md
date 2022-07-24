~~Importing the library into the ftc-app is extremely simple!~~

Unfortunately right now importing Evlib is not very simple, since the library has been migrated and
releases aren't up to date. We're working on it though!

1. Download the latest release `EVLib-release.aar`. You can also click ~~[here](https://github.com/FTC7393/EVLib/raw/master/EVLib-release.aar)~~.

2. Copy the downloaded library to the `(your project root)/libs/` directory.

3. Now you need to tell gradle that EVLib is a dependency. In android studio, edit the `TeamCode/build.gradle` file and add the following lines:
```groovy
dependencies {
    implementation 'EVlib-release.aar`
}
```

Attaching the source (Optional, out of date):

1. ~~[Download the repository as a zip file](https://github.com/FTC7393/EVLib/archive/master.zip)~~ and extract it.

2. When a yellow bar appears at the top of the screen:

![Decompiled .class file](https://github.com/FTC7393/EVLib/blob/master/images/attach.png?raw=true)

3. Click on the blue link that says "Choose Sources..." and a window will pop up.

![attach sources window](https://github.com/FTC7393/EVLib/blob/master/images/attach2.png?raw=true)

4. Choose the "EVLib-master/EVLib/src/main/java directory" from the file you downloaded.

You are done! You can look at the next chapter to get you started on using the library.