> This page is out of date (unsupported feature)

To create your own EndConditions, all you have to do is make a class that extends [EndConditions](EndConditions.md).
```java
public class EVEndConditions extends EndConditions {

}
```

And add some methods to it, for example a gyro end condition.
```java
public class EVEndConditions extends EndConditions {

    public static EndCondition gyroCloseTo(final GyroSensor gyro, double targetDegrees, final double toleranceDegrees){
        final Vector3D targetVector = Vector3D.fromPolar2D(1, Angle.fromDegrees(targetDegrees));
        return new EndCondition() {
            @Override
            public void init() {}

            @Override
            public boolean isDone() {
                Vector3D gyroVector = Vector3D.fromPolar2D(1, Angle.fromDegrees(gyro.getHeading()));
                Angle separation = Vector3D.signedAngularSeparation(targetVector, gyroVector);
                return Math.abs(separation.getValueDegrees()) <= toleranceDegrees;
            }
        };
    }

}
```