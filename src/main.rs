use druid::kurbo::Circle;
use druid::widget::{Button, Flex, Label, Slider, ViewSwitcher};
use druid::*;

use druid::im::{vector, Vector};

fn main() {
    let boxes = vector![
        TestBox::new("red", Color::RED),
        TestBox::new("green", Color::GREEN),
        TestBox::new("blue", Color::BLUE),
        TestBox::new("yellow", Color::YELLOW),
        TestBox::new("purple", Color::PURPLE),
    ];

    let names: Vec<&str> = boxes.iter().map(|b| b.name).collect();
    let main_window = WindowDesc::new(|| main_widget(names));

    let _ = AppLauncher::with_window(main_window).launch(AppData::init(boxes));
}

fn main_widget(names: Vec<&str>) -> impl Widget<AppData> {
    let mut controls = Flex::column();
    for (i, name) in names.iter().enumerate() {
        controls.add_child(
            Button::new(name.to_string())
                .on_click(move |_event, data: &mut usize, _env| {
                    *data = i;
                })
                .lens(AppData::current_view),
        );
    }
    controls.add_spacer(40.0);
    controls.add_child(Label::new("Scale"));
    controls.add_child(ViewSwitcher::new(
        |data: &AppData, _env| data.current_view,
        |selector, _data, _env| {
            Box::new(
                Slider::new()
                    .with_range(0.0, 1.0)
                    .lens(AppData::boxes.index(*selector).then(TestBox::scale)),
            )
        },
    ));

    Flex::row().with_child(controls).with_child(BoxMaker::new())
}

#[derive(Clone, Data, Lens)]
struct AppData {
    current_view: usize,
    scale_slider: f64,
    boxes: Vector<TestBox>,
}

impl AppData {
    fn init(boxes: Vector<TestBox>) -> AppData {
        AppData {
            current_view: 0,
            scale_slider: 0.2,
            boxes,
        }
    }
}

#[derive(Clone, Data, Lens)]
struct TestBox {
    current_view: usize,
    name: &'static str,
    color: Color,
    scale: f64,
}

impl TestBox {
    fn new(name: &'static str, color: Color) -> Self {
        TestBox {
            current_view: 0,
            name,
            color,
            scale: 0.2,
        }
    }
}

struct BoxMaker {}

impl BoxMaker {
    fn new() -> Self {
        BoxMaker {}
    }
}

impl Widget<AppData> for BoxMaker {
    fn event(&mut self, _: &mut EventCtx, _: &Event, _: &mut AppData, _: &Env) {}

    fn lifecycle(&mut self, _: &mut LifeCycleCtx, _: &LifeCycle, _: &AppData, _: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &AppData, _data: &AppData, _e: &Env) {
        // if data.current_view == old_data.current_view {
        //     let this_box = &mut BOXES.lock().unwrap()[data.current_view];
        //     this_box.scale = data.scale_slider;
        // }
        ctx.request_paint()
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, _: &BoxConstraints, _: &AppData, _: &Env) -> Size {
        ctx.window().get_size()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppData, _env: &druid::Env) {
        // paint corners
        // ad hoc correction to get to actual pane_size
        let bad_pane_size = ctx.size();
        let pane_size = Size::new(bad_pane_size.width - 100.0, bad_pane_size.height - 25.0);
        let corrected_corner = Point::new(pane_size.width, pane_size.height);

        ctx.clip(pane_size.to_rect());
        ctx.clear(Color::rgb(0.90, 0.90, 0.90));

        // upper left corner
        ctx.stroke(Circle::new(Point::ORIGIN, 20.0), &Color::LIME, 5.0);
        let bad_corner = Point::new(bad_pane_size.width, bad_pane_size.height);

        // lower right corner - outside of widget
        ctx.stroke(Circle::new(bad_corner, 20.0), &Color::LIME, 5.0);
        // actual lower right corner for widget
        ctx.stroke(Circle::new(corrected_corner, 20.0), &Color::PURPLE, 5.0);

        let this_box = &data.boxes[data.current_view]; //&BOXES.lock().unwrap()[data.current_view];
        let max_size = pane_size.width.min(pane_size.height);
        let this_box_size = Size::new(this_box.scale * max_size, this_box.scale * max_size);
        let rect = Rect::from_center_size(pane_size.to_rect().center(), this_box_size);
        ctx.stroke(rect, &this_box.color, 10.0);
    }
}
