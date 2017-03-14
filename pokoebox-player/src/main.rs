#[macro_use]
extern crate conrod;
extern crate find_folder;

use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{DisplayBuild, Surface};

struct Fonts {
    regular: conrod::text::font::Id,
    italic: conrod::text::font::Id,
    bold: conrod::text::font::Id,
}

// Generate a unique const `WidgetId` for each widget.
widget_ids!{
    struct Ids {
        master,
        left_col,
        middle_col,
        right_col,
        left_text,
        middle_text,
        right_text,
        button,
    }
}

fn main() {
    // Window dimensions
    const WIDTH: u32 = 600;
    const HEIGHT: u32 = 420;

    // Build the window
    let display = glium::glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title("Some title!")
        .build_glium()
        .unwrap();

    // Construct the UI
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // A unique identifier for each widget.
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let noto_sans = assets.join("fonts/NotoSans");

    // Store our `font::Id`s in a struct for easy access in the `set_ui` function.
    let fonts = Fonts {
        regular: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Regular.ttf")).unwrap(),
        italic: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Italic.ttf")).unwrap(),
        bold: ui.fonts.insert_from_file(noto_sans.join("NotoSans-Bold.ttf")).unwrap(),
    };

    // Set the default font
    ui.theme.font_id = Some(fonts.regular);

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut event_loop = EventLoop::new();

    // Main program loop
    'main: loop {
        // Handle events
        for event in event_loop.next(&display) {
            // Use the winit backend feature to convert the winit event to a conrod one
            if let Some(event) = conrod::backend::winit::convert(event.clone(), &display) {
                ui.handle_event(event);
            }

            match event {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed =>
                    break 'main,

                _ => {}
            }
        }

        set_ui(ui.set_widgets(), &ids, &fonts);

        // Render the UI and then display it on the screen
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();

            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();

            target.finish().unwrap();
        }
    }
}

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids, fonts: &Fonts) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Scalar, Sizeable, Widget};

    // Our `Canvas` tree, upon which we will place our text widgets.
    widget::Canvas::new().flow_right(&[
        (ids.left_col, widget::Canvas::new().color(color::BLACK)),
        (ids.middle_col, widget::Canvas::new().color(color::DARK_CHARCOAL)),
        (ids.right_col, widget::Canvas::new().color(color::CHARCOAL)),
    ]).set(ids.master, ui);

    const DEMO_TEXT: &'static str = "Some sample text.\n\nIt is working!";

    const PAD: Scalar = 20.0;

    widget::Text::new(DEMO_TEXT)
        .font_id(fonts.regular)
        .color(color::LIGHT_RED)
        .padded_w_of(ids.left_col, PAD)
        .mid_top_with_margin_on(ids.left_col, PAD)
        .left_justify()
        .line_spacing(10.0)
        .set(ids.left_text, ui);

    widget::Text::new(DEMO_TEXT)
        .font_id(fonts.italic)
        .color(color::LIGHT_GREEN)
        .padded_w_of(ids.middle_col, PAD)
        .middle_of(ids.middle_col)
        .center_justify()
        .line_spacing(2.5)
        .set(ids.middle_text, ui);

    widget::Text::new(DEMO_TEXT)
        .font_id(fonts.bold)
        .color(color::LIGHT_BLUE)
        .padded_w_of(ids.right_col, PAD)
        .mid_bottom_with_margin_on(ids.right_col, PAD)
        .right_justify()
        .line_spacing(5.0)
        .set(ids.right_text, ui);

    let button = widget::Button::new()
        .label("PRESS ME")
        .padded_w_of(ids.right_col, PAD)
        .mid_top_with_margin_on(ids.right_col, PAD)
        .set(ids.button, ui);

    for _press in button {
        println!("PRESSED!");
    }
}

/// A set of reasonable stylistic defaults that works for the `gui` below.
pub fn theme() -> conrod::Theme {
    use conrod::position::{Align, Direction, Padding, Position, Relative};
    conrod::Theme {
        name: "Demo Theme".to_string(),
        padding: Padding::none(),
        x_position: Position::Relative(Relative::Align(Align::Start), None),
        y_position: Position::Relative(Relative::Direction(Direction::Backwards, 20.0), None),
        background_color: conrod::color::DARK_CHARCOAL,
        shape_color: conrod::color::LIGHT_CHARCOAL,
        border_color: conrod::color::BLACK,
        border_width: 0.0,
        label_color: conrod::color::WHITE,
        font_id: None,
        font_size_large: 26,
        font_size_medium: 18,
        font_size_small: 12,
        widget_styling: std::collections::HashMap::new(),
        mouse_drag_threshold: 0.0,
        double_click_threshold: std::time::Duration::from_millis(500),
    }
}
pub struct EventLoop {
    ui_needs_update: bool,
    last_update: std::time::Instant,
}

impl EventLoop {

    pub fn new() -> Self {
        EventLoop {
            last_update: std::time::Instant::now(),
            ui_needs_update: true,
        }
    }

    /// Produce an iterator yielding all available events.
    pub fn next(&mut self, display: &glium::Display) -> Vec<glium::glutin::Event> {
        // We don't want to loop any faster than 60 FPS, so wait until it has been at least 16ms
        // since the last yield.
        let last_update = self.last_update;
        let sixteen_ms = std::time::Duration::from_millis(16);
        let duration_since_last_update = std::time::Instant::now().duration_since(last_update);
        if duration_since_last_update < sixteen_ms {
            std::thread::sleep(sixteen_ms - duration_since_last_update);
        }

        // Collect all pending events.
        let mut events = Vec::new();
        events.extend(display.poll_events());

        // If there are no events and the `Ui` does not need updating, wait for the next event.
        if events.is_empty() && !self.ui_needs_update {
            events.extend(display.wait_events().next());
        }

        self.ui_needs_update = false;
        self.last_update = std::time::Instant::now();

        events
    }

    /// Notifies the event loop that the `Ui` requires another update whether or not there are any
    /// pending events.
    ///
    /// This is primarily used on the occasion that some part of the `Ui` is still animating and
    /// requires further updates to do so.
    pub fn needs_update(&mut self) {
        self.ui_needs_update = true;
    }
}