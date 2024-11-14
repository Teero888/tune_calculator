A simple calculator for physics tunes in ddnet to make the game seem slower or faster.
Even though it works very well most of the time, due to ddnet rounding physics values this won't work well if the scale is too extreme.
Currently there is no good way to apply these tunes to your map except putting them in manually.

`usage: tune_calculator <scale>`
scale 0.5 would be equivalent to a 2x slow down.

example output:
```
> tune_calculator 0.5
tune ground_control_speed 5
tune ground_control_accel 0.5
tune ground_friction 0.70710677
tune ground_jump_impulse 6.6
tune air_jump_impulse 6
tune air_control_speed 2.5
tune air_control_accel 0.375
tune air_friction 0.9746794
tune hook_fire_speed 40
tune hook_drag_accel 0.75
tune hook_drag_speed 7.5
tune gravity 0.125
tune velramp_start 275
tune velramp_range 1000
tune gun_speed 1100
tune gun_lifetime 1
tune grenade_speed 500
tune grenade_lifetime 1
tune laser_bounce_delay 75
tune jetpack_strength 200
tune shotgun_strength 5
tune explosion_strength 3
tune hammer_strength 0.5
tune hook_duration 0.625
tune hammer_fire_delay 62.5
tune gun_fire_delay 62.5
tune shotgun_fire_delay 250
tune grenade_fire_delay 250
tune laser_fire_delay 400
tune ninja_fire_delay 400
tune hammer_hit_fire_delay 160
```
