use egui::{self, CentralPanel, Context, TextureId, Ui, Color32};
use std::collections::HashMap;

pub struct Gui {
    sprite_editor: SpriteEditor,
    tile_editor: TileEditor,
    command_input: String,
    command_output: String,
    console_commands: HashMap<String, Box<dyn Fn(&[&str]) -> String>>,
}

impl Gui {
    pub fn new() -> Self {
        let mut gui = Self {
            sprite_editor: SpriteEditor::new(),
            tile_editor: TileEditor::new(),
            command_input: String::new(),
            command_output: String::new(),
            console_commands: HashMap::new(),
        };
        gui.register_commands();
        gui
    }

    fn register_commands(&mut self) {
        self.console_commands.insert("help".to_string(), Box::new(|_args| {
            "Available commands: help, exit, clear, load [file], save [file], reset, set [property] [value]".to_string()
        }));

        self.console_commands.insert("exit".to_string(), Box::new(|_args| {
            std::process::exit(0);
        }));

        self.console_commands.insert("clear".to_string(), Box::new(|_args| {
            self.command_output.clear();
            "Console cleared.".to_string()
        }));

        self.console_commands.insert("load".to_string(), Box::new(|args| {
            if args.is_empty() {
                return "Usage: load [file]".to_string();
            }
            let file = args[0];
            // Add file loading logic here
            format!("Loaded file: {}", file)
        }));

        self.console_commands.insert("save".to_string(), Box::new(|args| {
            if args.is_empty() {
                return "Usage: save [file]".to_string();
            }
            let file = args[0];
            // Add file saving logic here
            format!("Saved to file: {}", file)
        }));

        self.console_commands.insert("reset".to_string(), Box::new(|_args| {
            // Add reset logic here
            "Game state reset.".to_string()
        }));

        self.console_commands.insert("set".to_string(), Box::new(|args| {
            if args.len() < 2 {
                return "Usage: set [property] [value]".to_string();
            }
            let property = args[0];
            let value = args[1];
            // Add property setting logic here
            format!("Set {} to {}", property, value)
        }));
    }

    fn execute_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if let Some(cmd) = self.console_commands.get(parts[0]) {
            let result = cmd(&parts[1..]);
            self.command_output.push_str(&format!("> {}\n{}\n", command, result));
        } else {
            self.command_output.push_str(&format!("> {}\nUnknown command: {}\n", command, parts[0]));
        }
    }

    pub fn draw(&mut self, egui_ctx: &Context, game_texture: TextureId) {
        CentralPanel::default().show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                self.sprite_editor.draw(ui);
                self.tile_editor.draw(ui);
            });

            ui.image(game_texture, ui.available_size());

            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut self.command_input).lost_focus()
                    && ui.input().key_pressed(egui::Key::Enter)
                {
                    self.execute_command(&self.command_input);
                    self.command_input.clear();
                }

                if ui.button("Execute").clicked() {
                    self.execute_command(&self.command_input);
                    self.command_input.clear();
                }
            });

            ui.add(egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(&self.command_output);
            }));
        });
    }
}

struct SpriteEditor {
    canvas: Vec<Color32>,
    canvas_size: usize,
    selected_color: Color32,
    sprite_size: (usize, usize),
}

impl SpriteEditor {
    pub fn new() -> Self {
        let sprite_size = (16, 16);
        Self {
            canvas: vec![Color32::BLACK; sprite_size.0 * sprite_size.1],
            canvas_size: sprite_size.0 * sprite_size.1,
            selected_color: Color32::WHITE,
            sprite_size,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Sprite Editor");

            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.selected_color);
                if ui.button("Clear").clicked() {
                    self.canvas.fill(Color32::BLACK);
                }
            });

            let (sprite_width, sprite_height) = self.sprite_size;
            let canvas_width = 320.0;
            let pixel_size = canvas_width / sprite_width as f32;

            ui.allocate_ui_with_layout(
                egui::vec2(canvas_width, canvas_width),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    let painter = ui.painter();
                    for y in 0..sprite_height {
                        for x in 0..sprite_width {
                            let rect = egui::Rect::from_min_size(
                                ui.min_rect().min
                                    + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                                egui::vec2(pixel_size, pixel_size),
                            );

                            let color = self.canvas[y * sprite_width + x];
                            painter.rect_filled(rect, 0.0, color);

                            if ui.input().pointer.any_pressed() && rect.contains(ui.input().pointer.interact_pos().unwrap_or_default()) {
                                self.canvas[y * sprite_width + x] = self.selected_color;
                            }
                        }
                    }
                },
            );
        });
    }
}

struct TileEditor {
    tiles: Vec<Vec<Color32>>,
    selected_tile: usize,
    selected_color: Color32,
    tile_size: (usize, usize),
}

impl TileEditor {
    pub fn new() -> Self {
        let tile_size = (8, 8);
        Self {
            tiles: vec![vec![Color32::BLACK; tile_size.0 * tile_size.1]; 256],
            selected_tile: 0,
            selected_color: Color32::WHITE,
            tile_size,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.label("Tile Editor");

            ui.horizontal(|ui| {
                ui.color_edit_button_srgba(&mut self.selected_color);
                if ui.button("Clear").clicked() {
                    self.tiles[self.selected_tile].fill(Color32::BLACK);
                }
            });

            let (tile_width, tile_height) = self.tile_size;
            let canvas_width = 128.0;
            let pixel_size = canvas_width / tile_width as f32;

            ui.allocate_ui_with_layout(
                egui::vec2(canvas_width, canvas_width),
                egui::Layout::top_down(egui::Align::Center),
                |ui| {
                    let painter = ui.painter();
                    for y in 0..tile_height {
                        for x in 0..tile_width {
                            let rect = egui::Rect::from_min_size(
                                ui.min_rect().min
                                    + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                                egui::vec2(pixel_size, pixel_size),
                            );

                            let color = self.tiles[self.selected_tile][y * tile_width + x];
                            painter.rect_filled(rect, 0.0, color);

                            if ui.input().pointer.any_pressed() && rect.contains(ui.input().pointer.interact_pos().unwrap_or_default()) {
                                self.tiles[self.selected_tile][y * tile_width + x] = self.selected_color;
                            }
                        }
                    }
                },
            );

            ui.add(egui::Slider::new(&mut self.selected_tile, 0..=255).text("Tile"));
        });
    }
}
