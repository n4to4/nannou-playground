use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let sine = app.time.sin();
    let slowersine = (app.time / 2.0).sin();

    let draw = app.draw();
    draw.background().color(PLUM);
    draw.ellipse().color(STEELBLUE).x_y(0.0, 0.0);
    draw.to_frame(app, &frame).unwrap();
}
