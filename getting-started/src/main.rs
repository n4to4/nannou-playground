use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    let point1 = pt2(-10.0, -20.0);
    let point2 = pt2(10.0, -30.0);
    let point3 = pt2(15.0, 40.0);

    draw.tri()
        .color(STEELBLUE)
        .w(300.0)
        .h(200.0)
        .points(point1, point2, point3);

    draw.to_frame(app, &frame).unwrap();
}
