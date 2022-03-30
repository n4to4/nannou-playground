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
    let pad = 25.0;
    let win = app.window_rect();
    let win_p = win.pad(pad);
    let draw = app.draw();

    let square = Rect::from_w_h(100.0, 100.0).top_left_of(win_p);
    draw.rect().xy(square.xy()).wh(square.wh()).color(PLUM);

    let circle = square.below(square).shift_y(-pad);
    draw.ellipse().xy(circle.xy()).wh(circle.wh()).color(SALMON);

    draw.to_frame(app, &frame).unwrap();
}
