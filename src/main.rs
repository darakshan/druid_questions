/*
Questions:

- When ViewSwitcher changes views, how can I reset the Scale slider?

- ctx.size() seems to be size of whole window.  Maybe because that's what layout() returns.
  What is the appropriate definition of Layout for a simple, flexible canvas?
  How do I get size of my widget's actual drawing region?

  (In this demo I make ad hoc corrections in paint based on what seems to work on my Mac.)

- BoxMaker needs to access state (BOXES) that persists between BoxMaker instances.
  But I see no way to pass it in except by making TestBox copy,
  which is not acceptable in a real application.
  Is there any alternative to lazy_static?

  Maybe I could put a slider for each box in AppData, but I can't figure out how to
  make a Lens index into my array.  A single example would be nice.

- Is there a way to set the minimum window size? I would like to get smaller.

*/

use druid::kurbo::Circle;
use druid::widget::{Button, Flex, Label, Slider, ViewSwitcher};
use druid::*;

fn main() {
    let main_window = WindowDesc::new(main_widget);
    let _ = AppLauncher::with_window(main_window).launch(AppData::init());
}

fn main_widget() -> impl Widget<AppData> {
    let mut controls = Flex::column();
    for i in 0..3 {
        let name = &BOXES.lock().unwrap()[i].name;
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
    controls.add_child(
        Slider::new()
            .with_range(0.0, 1.0)
            .lens(AppData::scale_slider),
    );

    Flex::row()
        .with_child(controls)
        .with_child(ViewSwitcher::new(
            |data: &AppData, _env| data.current_view,
            |_selector, _data, _env| BoxMaker::new().boxed(),
        ))
}

#[derive(Clone, Data, Lens)]
struct AppData {
    current_view: usize,
    scale_slider: f64,
}

impl AppData {
    fn init() -> AppData {
        AppData {
            current_view: 0,
            scale_slider: 0.2,
        }
    }
}

#[derive(Clone)]
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

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref BOXES: Mutex<Vec<TestBox>> = Mutex::new(init_boxes());
}

fn init_boxes() -> Vec<TestBox> {
    vec![
        TestBox::new("red", Color::RED),
        TestBox::new("green", Color::GREEN),
        TestBox::new("blue", Color::BLUE),
    ]
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

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppData, data: &AppData, _e: &Env) {
        if data.current_view == old_data.current_view {
            let this_box = &mut BOXES.lock().unwrap()[data.current_view];
            this_box.scale = data.scale_slider;
        }
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

        let this_box = &BOXES.lock().unwrap()[data.current_view];
        let max_size = pane_size.width.min(pane_size.height);
        let this_box_size = Size::new(this_box.scale * max_size, this_box.scale * max_size);
        let rect = Rect::from_center_size(pane_size.to_rect().center(), this_box_size);
        ctx.stroke(rect, &this_box.color, 10.0);
    }
}
