use std::env;

struct Tunes {
    ground_control_speed: f32,
    ground_control_accel: f32,
    ground_friction: f32,
    ground_jump_impulse: f32,
    air_jump_impulse: f32,
    air_control_speed: f32,
    air_control_accel: f32,
    air_friction: f32,
    hook_fire_speed: f32,
    hook_drag_accel: f32,
    hook_drag_speed: f32,
    gravity: f32,
    velramp_start: f32,
    velramp_range: f32,
    gun_speed: f32,
    gun_lifetime: f32,
    grenade_speed: f32,
    grenade_lifetime: f32,
    laser_bounce_delay: f32,
    jetpack_strength: f32,
    shotgun_strength: f32,
    explosion_strength: f32,
    hammer_strength: f32,
    hook_duration: f32,
    hammer_fire_delay: f32,
    gun_fire_delay: f32,
    shotgun_fire_delay: f32,
    grenade_fire_delay: f32,
    laser_fire_delay: f32,
    ninja_fire_delay: f32,
    hammer_hit_fire_delay: f32,
}

fn physics_scaling_linear(value: f32, tick_speed: i32) -> f32 {
    value / { tick_speed as f32 / 50.0 }
}

fn physics_scaling_accel(value: f32, tick_speed: i32) -> f32 {
    value / f32::powi(tick_speed as f32 / 50.0, 2)
}

fn physics_scaling_friction(value: f32, tick_speed: i32) -> f32 {
    return f32::powf(value, 1.0 / (tick_speed as f32 / 50.0));
}

fn calculate_tunes(tick_speed: i32) -> Tunes {
    Tunes {
        ground_control_speed: physics_scaling_linear(10.0, tick_speed),
        ground_control_accel: physics_scaling_accel(2.0, tick_speed),
        ground_friction: physics_scaling_friction(0.5, tick_speed),
        ground_jump_impulse: physics_scaling_linear(13.2, tick_speed),
        air_jump_impulse: physics_scaling_linear(12.0, tick_speed),
        air_control_speed: physics_scaling_linear(5.0, tick_speed),
        air_control_accel: physics_scaling_accel(1.5, tick_speed),
        air_friction: physics_scaling_friction(0.95, tick_speed),
        hook_fire_speed: physics_scaling_linear(80.0, tick_speed),
        hook_drag_accel: physics_scaling_accel(3.0, tick_speed),
        hook_drag_speed: physics_scaling_linear(15.0, tick_speed),
        gravity: physics_scaling_accel(0.5, tick_speed),
        velramp_start: physics_scaling_linear(550.0, tick_speed),
        velramp_range: physics_scaling_linear(2000.0, tick_speed),
        gun_speed: physics_scaling_linear(2200.0, tick_speed),
        gun_lifetime: physics_scaling_linear(2.0, tick_speed),
        grenade_speed: physics_scaling_linear(1000.0, tick_speed),
        grenade_lifetime: physics_scaling_linear(2.0, tick_speed),
        laser_bounce_delay: physics_scaling_linear(150.0, tick_speed),
        jetpack_strength: physics_scaling_linear(400.0, tick_speed),
        shotgun_strength: physics_scaling_linear(10.0, tick_speed),
        explosion_strength: physics_scaling_linear(6.0, tick_speed),
        hammer_strength: physics_scaling_linear(1.0, tick_speed),
        hook_duration: physics_scaling_linear(1.25, tick_speed),
        hammer_fire_delay: physics_scaling_linear(125.0, tick_speed),
        gun_fire_delay: physics_scaling_linear(125.0, tick_speed),
        shotgun_fire_delay: physics_scaling_linear(500.0, tick_speed),
        grenade_fire_delay: physics_scaling_linear(500.0, tick_speed),
        laser_fire_delay: physics_scaling_linear(800.0, tick_speed),
        ninja_fire_delay: physics_scaling_linear(800.0, tick_speed),
        hammer_hit_fire_delay: physics_scaling_linear(320.0, tick_speed),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("usage: tune_calculator <scale>");
        return;
    }
    let scale: f32 = match args[1].parse() {
        Ok(scale) => scale,
        Err(..) => {
            println!("usage: tune_calculator <scale>");
            return;
        }
    };
    let new_tunes = calculate_tunes(((1.0 / scale) * 50.0) as i32);
    println!("tune ground_control_speed {}\ntune ground_control_accel {}\ntune ground_friction {}\ntune ground_jump_impulse {}\ntune air_jump_impulse {}\ntune air_control_speed {}\ntune air_control_accel {}\ntune air_friction {}\ntune hook_fire_speed {}\ntune hook_drag_accel {}\ntune hook_drag_speed {}\ntune gravity {}\ntune velramp_start {}\ntune velramp_range {}\ntune gun_speed {}\ntune gun_lifetime {}\ntune grenade_speed {}\ntune grenade_lifetime {}\ntune laser_bounce_delay {}\ntune jetpack_strength {}\ntune shotgun_strength {}\ntune explosion_strength {}\ntune hammer_strength {}\ntune hook_duration {}\ntune hammer_fire_delay {}\ntune gun_fire_delay {}\ntune shotgun_fire_delay {}\ntune grenade_fire_delay {}\ntune laser_fire_delay {}\ntune ninja_fire_delay {}\ntune hammer_hit_fire_delay {}",
        new_tunes.ground_control_speed, new_tunes.ground_control_accel, new_tunes.ground_friction, new_tunes.ground_jump_impulse, new_tunes.air_jump_impulse, new_tunes.air_control_speed, new_tunes.air_control_accel, new_tunes.air_friction, new_tunes.hook_fire_speed, new_tunes.hook_drag_accel, new_tunes.hook_drag_speed, new_tunes.gravity, new_tunes.velramp_start, new_tunes.velramp_range, new_tunes.gun_speed, new_tunes.gun_lifetime, new_tunes.grenade_speed, new_tunes.grenade_lifetime, new_tunes.laser_bounce_delay, new_tunes.jetpack_strength, new_tunes.shotgun_strength, new_tunes.explosion_strength, new_tunes.hammer_strength, new_tunes.hook_duration, new_tunes.hammer_fire_delay, new_tunes.gun_fire_delay, new_tunes.shotgun_fire_delay, new_tunes.grenade_fire_delay, new_tunes.laser_fire_delay, new_tunes.ninja_fire_delay, new_tunes.hammer_hit_fire_delay);
}
