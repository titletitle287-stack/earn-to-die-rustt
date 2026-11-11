use macroquad::prelude::*;

struct Car {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    angle: f32,
    fuel: f32,
    health: f32,
}

struct Zombie {
    x: f32,
    y: f32,
    alive: bool,
}

#[macroquad::main("Earn to Die - Rust")]
async fn main() {
    let mut car = Car {
        x: 100.0,
        y: 300.0,
        vx: 0.0,
        vy: 0.0,
        angle: 0.0,
        fuel: 100.0,
        health: 100.0,
    };

    let mut zombies = vec![
        Zombie { x: 400.0, y: 0.0, alive: true },
        Zombie { x: 700.0, y: 0.0, alive: true },
        Zombie { x: 1100.0, y: 0.0, alive: true },
        Zombie { x: 1500.0, y: 0.0, alive: true },
        Zombie { x: 2000.0, y: 0.0, alive: true },
        Zombie { x: 2500.0, y: 0.0, alive: true },
    ];

    let gravity = 400.0;
    let acceleration = 250.0;

    loop {
        let dt = get_frame_time();

        let get_ground_y = |x: f32| -> f32 {
            400.0 + (x * 0.01).sin() * 50.0 + (x * 0.003).cos() * 30.0
        };

        let ground_y = get_ground_y(car.x);

        let is_touching = touches().iter().any(|t| t.phase == TouchPhase::Started || t.phase == TouchPhase::Moved || t.phase == TouchPhase::Stationary);
        let is_accelerating = (is_touching || is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || is_key_down(KeyCode::W)) && car.fuel > 0.0;

        if is_accelerating {
            car.vx += acceleration * dt;
            car.fuel -= 12.0 * dt;
            if car.fuel < 0.0 { car.fuel = 0.0; }
        } else {
            car.vx *= 0.98;
        }

        if car.y < ground_y - 20.0 {
            car.vy += gravity * dt;
        } else {
            car.y = ground_y - 20.0;
            car.vy = 0.0;
            let next_ground = get_ground_y(car.x + 10.0);
            car.angle = (next_ground - ground_y).atan2(10.0);
        }

        car.x += car.vx * dt;
        car.y += car.vy * dt;

        for z in zombies.iter_mut() {
            z.y = get_ground_y(z.x) - 15.0;
            if z.alive && (car.x - z.x).abs() < 30.0 && (car.y - z.y).abs() < 30.0 {
                z.alive = false;
                car.vx *= 0.85;
                car.health -= 5.0;
            }
        }

        let camera_offset_x = car.x - 150.0;

        clear_background(SKYBLUE);

        for screen_x in 0..(screen_width() as i32 / 5) {
            let world_x = camera_offset_x + (screen_x * 5) as f32;
            let gy = get_ground_y(world_x);
            draw_line(
                (screen_x * 5) as f32,
                gy,
                (screen_x * 5) as f32,
                screen_height(),
                8.0,
                BROWN,
            );
        }

        for z in zombies.iter() {
            if z.alive {
                let draw_x = z.x - camera_offset_x;
                draw_rectangle(draw_x, z.y, 15.0, 25.0, GREEN);
            }
        }

        let car_draw_x = car.x - camera_offset_x;
        draw_rectangle_ex(
            car_draw_x,
            car.y,
            50.0,
            20.0,
            DrawTextureParams {
                rotation: car.angle,
                color: RED,
                ..Default::default()
            },
        );
        draw_circle(car_draw_x + 10.0, car.y + 18.0, 7.0, BLACK);
        draw_circle(car_draw_x + 40.0, car.y + 18.0, 7.0, BLACK);

        draw_text(&format!("Бензин: {:.0}%", car.fuel), 20.0, 40.0, 30.0, WHITE);
        draw_text(&format!("Прочность: {:.0}%", car.health.max(0.0)), 20.0, 80.0, 30.0, WHITE);
        draw_text(&format!("Дистанция: {:.0}m", car.x / 10.0), 20.0, 120.0, 30.0, YELLOW);
        draw_text("Жми на экран для ГАЗА", 20.0, screen_height() - 30.0, 25.0, LIGHTGRAY);

        if car.fuel <= 0.0 && car.vx.abs() < 5.0 {
            draw_text("БЕНЗИН ЗАКАНЧИВАЛСЯ!", screen_width() / 2.0 - 150.0, screen_height() / 2.0, 30.0, RED);
        }

        next_frame().await;
    }
}
