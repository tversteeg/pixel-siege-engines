extern crate siege;
extern crate minifb;
extern crate direct_gui;
extern crate line_drawing;

mod editor;

use minifb::*;
use direct_gui::*;
use direct_gui::controls::*;
use siege::*;
use editor::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DrawTool {
    Beam,
    Wheel
}

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0x00222034; WIDTH * HEIGHT];

    let mut window = Window::new("Siege Editor - ESC to exit", WIDTH, HEIGHT, WindowOptions::default()).expect("Unable to open window");

    let mut gui = Gui::new((WIDTH as i32, HEIGHT as i32));

    let font = gui.default_font();

    let wood_button = gui.register(Button::new((32, 32), Color::from_u32(0x8F563B)).with_pos(4, 4));
    gui.register(Label::new(font).with_pos(40, 4).with_text("Wood"));

    let metal_button = gui.register(Button::new((32, 32), Color::from_u32(0x847E87)).with_pos(4, 40));
    gui.register(Label::new(font).with_pos(40, 40).with_text("Metal"));

    let rope_button = gui.register(Button::new((32, 32), Color::from_u32(0xD9A066)).with_pos(4, 76));
    gui.register(Label::new(font).with_pos(40, 76).with_text("Rope"));

    let wheel_button = gui.register(Button::new((32, 32), Color::from_u32(0xFFFFFF)).with_pos(4, 120));
    gui.register(Label::new(font).with_pos(40, 120).with_text("Wheel"));

    let beam_button = gui.register(Button::new((32, 32), Color::from_u32(0xFFFFFF)).with_pos(4, 156));
    gui.register(Label::new(font).with_pos(40, 156).with_text("Beam"));

    let mut selected_material = Material::Rope;
    let mut selected_tool = DrawTool::Beam;

    let mut editor = Editor::new(12 * SCALE, SCALE);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut cs = ControlState {
            ..ControlState::default()
        };

        window.get_mouse_pos(MouseMode::Pass).map(|mouse| {
            cs.mouse_pos = (mouse.0 as i32, mouse.1 as i32);
            cs.mouse_down = window.get_mouse_down(MouseButton::Left);

            {
                let wood_button: &Button<Flat> = gui.get(wood_button).unwrap();
                if !cs.mouse_down && wood_button.pressed() {
                    selected_material = Material::Wood;
                }

                let metal_button: &Button<Flat> = gui.get(metal_button).unwrap();
                if !cs.mouse_down && metal_button.pressed() {
                    selected_material = Material::Metal;
                }

                let rope_button: &Button<Flat> = gui.get(rope_button).unwrap();
                if !cs.mouse_down && rope_button.pressed() {
                    selected_material = Material::Rope;
                }

                let wheel_button: &Button<Flat> = gui.get(wheel_button).unwrap();
                if !cs.mouse_down && wheel_button.pressed() {
                    selected_tool = DrawTool::Wheel;
                }

                let beam_button: &Button<Flat> = gui.get(beam_button).unwrap();
                if !cs.mouse_down && beam_button.pressed() {
                    selected_tool = DrawTool::Beam;
                }
            }

            let action = match selected_tool {
                DrawTool::Wheel => Action::DrawWheel(selected_material),
                DrawTool::Beam => Action::DrawBeam(selected_material),
            };
            editor.update((cs.mouse_pos.0, cs.mouse_pos.1, cs.mouse_down), action);

            gui.update(&cs);
        });

        editor.draw(&mut buffer, WIDTH);

        gui.draw_to_buffer(&mut buffer);

        window.update_with_buffer(&buffer).unwrap();
    }
}
