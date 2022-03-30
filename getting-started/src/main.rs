use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let win = app.window_rect();
    let win_p = win.pad(25.0);
    let r = Rect::from_w_h(100.0f32, 100.0f32).top_left_of(win_p);
    let draw = app.draw();
    draw.rect()
        .xy(r.xy())
        .wh(r.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));
    draw.to_frame(app, &frame).unwrap();
}
