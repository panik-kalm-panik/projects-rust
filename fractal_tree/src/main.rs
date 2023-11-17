use nannou::prelude::*;

static WINDOW_WIDTH: u32 = 600;
static WINDOW_HEIGHT: u32 = 400;

static FIXED_ANGLE: f32 = 1.5707963267948966;

fn main() {
    nannou::app(model).run();
}

struct Model;

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Fractal Tree")
        .resizable(false)  // Set the window to not resizable
        .view(view)
        .build()
        .unwrap();
    Model
}

fn view(app: &App, _model: &Model, frame: Frame) {
    draw(app, frame);
}

fn draw(app: &App, frame: Frame) {
    let draw = app.draw();
    let window: Rect = app.window_rect();

    let start = pt2(0.0, 0.0 - (window.h() / 2.0));
    let len = 200.0;
    let end = pt2(start.x, start.y + len);
    let angle: f32 = 1.5707963267948966;

    draw.background().color(BLACK);
    
    branch(&draw, len, start, end, angle);
    
    draw.to_frame(app, &frame).unwrap();
}

fn branch(draw: &Draw, mut _len: f32, mut _start: Vec2, mut _end: Vec2, mut _angle: f32) {

    draw.line()
    .start(_start)
    .end(_end)
    .weight(1.0)
    .color(WHITE);

    let new_len = _len * 0.65;
    let new_start = _end;
    let mut new_angle = _angle - FIXED_ANGLE;
    let mut end_offset = pt2(new_len * f32::cos(new_angle), new_len * f32::sin(new_angle));
    let mut new_end = _end + end_offset;

    if new_len > 3.0 {
        branch(&draw, new_len, new_start, new_end, new_angle);
    }

    new_angle = _angle + FIXED_ANGLE;
    end_offset = pt2(new_len * f32::cos(new_angle), new_len * f32::sin(new_angle));
    new_end = _end + end_offset;

    if new_len > 3.0 {
        branch(&draw, new_len, new_start, new_end, new_angle);
    }
}
