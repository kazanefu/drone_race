# drone race

this is a 3D drone racing game. main mode is time attack mode.

# control

- W: Up
- S: Down
- A: Yaw Left
- D: Yaw Right
- Mouse X: Roll Left/Right
- Mouse Y: Pitch Up/Down
- Mouse Wheel: Throttle Up/Down
- Left Shift: Throttle Up
- Left Ctrl: Throttle Down
- Space: Throttle Boost
- First person camera

# physics

- drone speed is defined by total acceleration.
- drone has mass.
- drone acceleration is defined by total force & mass.
- drone force is defined by motor force, gravity, and drag.
- drone motor force is defined by throttle.
- drone gravity is defined by gravity constant.
- drone drag is defined by drag coefficient & current speed.
- all forces, acceleration, speed, position are vectors.
- all forces, acceleration, speed, position are 3D vectors.
- drone has battery.
- drone battery capacity is defined by PC memory usage rate.
- drone battery consumption is defined by throttle & GPU clock & CPU clock

# game rule

0. home scene
1. get PC status when Enter key is pressed.
2. calculate drone battery capacity.
3. calculate drone battery consumption.
4. set drone start transform(position, rotation, scale).
5. count down 3, 2, 1, 0.
6. start drone.
7. go through the gates in order.
8. finish the race.
9. calculate drone score.
10. show result scene.
11. back to home scene.

# drone

- drone has 4 motors as design but physics simulation uses 1 motor.
- drone has Transform component.
- drone has Velocity component.
- drone has Acceleration component.
- drone has Gravity component.
- drone has Drag component.
- drone has DroneStatus component which contains battery, throttle, is_boost, is_crash .
- drone has DroneControl component which contains roll, pitch, yaw, throttle, boost, crash.
- drone has DronePhysics component which contains Velocity, Acceleration, Gravity, Drag.

# pc status

- pc status is obtained from sysinfo crate.
- pc status contains memory usage rate, cpu clock, gpu clock.

# DroneStatus component

- battery is f64
- throttle is f64
- is_boost is bool
- is_crash is bool

# DroneControl component

- roll is f64
- pitch is f64
- yaw is f64
- throttle is f64
- boost is bool
- crash is bool

# DronePhysics component

- velocity is Vec3
- acceleration is Vec3
- gravity is Vec3
- drag is Vec3

# DroneStatus Calculation

- battery capacity = 100 * (1 - memory usage rate)
- battery = battery capacity - battery consumption * throttle * time
- battery consumption = (cpu clock + gpu clock) / 2
