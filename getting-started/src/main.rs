use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    let start_point = pt2(-30.0, -20.0);
    let end_point = pt2(40.0, 40.0);

    draw.line()
        .start(start_point)
        .end(end_point)
        .weight(4.0)
        .color(STEELBLUE);

    draw.to_frame(app, &frame).unwrap();
}
