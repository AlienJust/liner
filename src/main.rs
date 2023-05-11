use std::collections::VecDeque;

use macroquad::prelude::*;

struct LineCoord {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    /*
    let mut x = screen_width() / 2.0;
    let mut y = screen_height() / 2.0;

    loop {
        clear_background(LIGHTGRAY);

        if is_key_down(KeyCode::Right) {
            x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            x -= 1.0;
        }
        if is_key_down(KeyCode::Down) {
            y += 1.0;
        }
        if is_key_down(KeyCode::Up) {
            y -= 1.0;
        }

        draw_circle(x, y, 15.0, YELLOW);
        draw_text("move the ball with arrow keys", 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
    */
    /*
    let minimum_frame_time = 1. / 60.; // 60 FPS
    loop {
        clear_background(LIGHTGRAY);

        // Render some primitives in camera space

        set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        });
        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("HELLO", 30.0, 200.0, 30.0, BLACK);


        let frame_time = get_frame_time();
        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
        next_frame().await
    }*/

    /*loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }*/

    let minimum_frame_time = 1. / 240.; // 60 FPS

    let mut coords: VecDeque<LineCoord> = VecDeque::new();

    let lc = LineCoord {
        x1: 0.0,
        y1: 0.0,
        x2: 100.0,
        y2: 50.0,
    };

    coords.push_back(lc);

    /*
    let mut x1 = 0.0;
    let mut y1 = 0.0;

    let mut x2 = 100.0;
    let mut y2 = 50.0;
     */

    let mut dx1 = 2.0;
    let mut dy1 = 2.0;

    let mut dx2 = 4.0;
    let mut dy2 = 4.0;

    clear_background(LIGHTGRAY);
    loop {
        for cc in coords.iter_mut() {
            draw_line(cc.x1, cc.y1, cc.x2, cc.y2, 1.0, BLUE);
        }

        let ccx = coords.get(coords.len() - 1).unwrap();

        let mut x1 = ccx.x1;
        let mut y1 = ccx.y1;

        let mut x2 = ccx.x2;
        let mut y2 = ccx.y2;

        /*draw_circle(x1, y1, 10.0, YELLOW);
        draw_circle(x2, y2, 10.0, RED);
        draw_circle(x1, y2, 10.0, BLUE);
        draw_circle(x2, y1, 10.0, GREEN);*/

        x1 += dx1;
        y1 += dy1;

        x2 += dx2;
        y2 += dy2;

        if x1 < 0.0 {
            dx1 = 2.0;
        } else if x1 > screen_width() {
            dx1 = -2.0;
        }

        if y1 < 0.0 {
            dy1 = 2.0;
        } else if y1 > screen_height() {
            dy1 = -2.0;
        }

        if x2 < 0.0 {
            dx2 = 4.0;
        } else if x2 > screen_width() {
            dx2 = -4.0;
        }

        if y2 < 0.0 {
            dy2 = 4.0;
        } else if y2 > screen_height() {
            dy2 = -4.0;
        }

        let ccn = LineCoord {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        };

        if coords.len() > 10 {
            coords.pop_front();
        }
        coords.push_back(ccn);

        /*
        let frame_time = get_frame_time();
        println!("Frame time: {}ms", frame_time * 1000.);
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            println!("Sleep for {}ms", time_to_sleep);
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }
         */

        next_frame().await
    }
}
