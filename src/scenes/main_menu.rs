// The MIT License (MIT)
// Copyright © 2014-2018 Miguel Peláez <kernelfreeze@outlook.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation
// files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy,
// modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
// OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use gfx::canvas::Canvas;
use gfx::pencil::Pencil;
use gfx::scene::{Scene, SceneAction};

use core::camera::Camera;
use core::constants::*;

use core::resource_manager::resource::Resource;
use core::resource_manager::resource_type::ResourceType;
use core::resource_manager::ResourceManager;

use glium::framebuffer::SimpleFrameBuffer;

use conrod::position::rect::Rect;

/// How many time we should wait before changing our wallpaper
const WALLPAPER_DELAY: u32 = 15;

widget_ids! {
    struct Ids {
        master,

        header,

        header_left_column,
        header_right_column,

        title,
        logo_left,
        logo_right,

        body,

        body_left_column,
        body_middle_column,
        body_right_column,

        singleplayer,
        multiplayer,

        footer,

        version,
        copyright,
    }
}

/// Show Litecraft logo and start resource loading
pub struct MainMenu {
    ids: Ids,
    camera: Camera,
}

impl MainMenu {
    pub fn new(canvas: &mut Canvas) -> MainMenu {
        MainMenu {
            ids: Ids::new(canvas.ui_mut().widget_id_generator()),
            camera: Camera::new(),
        }
    }

    /// Main menu's background
    fn draw_wallpaper(&mut self, canvas: &mut Canvas, frame: &mut SimpleFrameBuffer) {
        let i = ResourceManager::time() as u32 / WALLPAPER_DELAY % 12 + 1;

        let wallpaper = canvas.resources().textures().get(&Resource::litecraft_path(
            format!("menu_{}", i),
            "wallpapers",
            ResourceType::Texture,
        ));

        if let Some(wallpaper) = wallpaper {
            Pencil::new(frame, "quad", &canvas)
                .texture(wallpaper)
                .camera(&self.camera)
                .vertices(canvas.resources().shapes().rectangle())
                .linear(true)
                .draw();
        }
    }
}

impl Scene for MainMenu {
    /// Do resource load
    fn load(&mut self, canvas: &mut Canvas) {
        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::minecraft_path(
                "minecraft",
                "gui/title",
                ResourceType::Texture,
            ));

        canvas
            .resources_mut()
            .textures_mut()
            .load_ui(Resource::minecraft_path("widgets", "gui", ResourceType::Texture));
    }

    /// Draw scene
    fn draw(&mut self, canvas: &mut Canvas, frame: &mut SimpleFrameBuffer) -> SceneAction {
        use conrod::{color, widget, Borderable, Colorable, Labelable, Positionable, Sizeable, Widget};

        let logo = canvas.resources().textures().get_ui(&Resource::minecraft_path(
            "minecraft",
            "gui/title",
            ResourceType::Texture,
        ));

        let widgets = canvas.resources().textures().get_ui(&Resource::minecraft_path(
            "widgets",
            "gui",
            ResourceType::Texture,
        ));

        self.draw_wallpaper(canvas, frame);

        let mut ui = canvas.ui_mut().set_widgets();

        // Construct our main `Canvas` tree.
        widget::Canvas::new()
            .color(color::TRANSPARENT)
            .flow_down(&[
                (
                    self.ids.header,
                    widget::Canvas::new()
                        .color(color::TRANSPARENT)
                        .border(0.0)
                        .pad(85.0)
                        .flow_right(&[
                            (
                                self.ids.header_left_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.header_right_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                        ]),
                ),
                (
                    self.ids.body,
                    widget::Canvas::new()
                        .color(color::TRANSPARENT)
                        .border(0.0)
                        .length(300.0)
                        .flow_right(&[
                            (
                                self.ids.body_left_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.body_middle_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                            (
                                self.ids.body_right_column,
                                widget::Canvas::new().color(color::TRANSPARENT).border(0.0),
                            ),
                        ]),
                ),
                (
                    self.ids.footer,
                    widget::Canvas::new()
                        .pad(20.0)
                        .scroll_kids_vertically()
                        .border(0.0)
                        .color(color::TRANSPARENT),
                ),
            ])
            .set(self.ids.master, &mut ui);

        // Header //

        // Draw the beloved Minecraft logo
        if let Some(logo) = logo {
            let base = 256.0;
            let size = [280.0, 85.0];
            let (w, h) = logo.1;

            // Draw logo first part
            widget::Image::new(logo.0)
                .bottom_right_of(self.ids.header_left_column)
                .wh(size)
                .source_rectangle(Rect::from_corners(
                    [0.0, 212.0 * h / base],
                    [156.0 * w / base, 256.0 * h / base],
                ))
                .set(self.ids.logo_left, &mut ui);

            // Draw logo second part
            widget::Image::new(logo.0)
                .bottom_left_of(self.ids.header_right_column)
                .wh(size)
                .source_rectangle(Rect::from_corners(
                    [0.0, 168.0 * h / base],
                    [156.0 * w / base, 211.0 * h / base],
                ))
                .set(self.ids.logo_right, &mut ui);
        }

        // Body //

        if let Some(widgets) = widgets {
            let base = 256.0;
            let (w, h) = widgets.1;

            let base_rect =
                Rect::from_corners([0.0, 170.0 * h / base], [200.0 * w / base, 190.0 * h / base]);
            let hover_rect =
                Rect::from_corners([0.0, 150.0 * h / base], [200.0 * w / base, 170.0 * h / base]);
            let press_rect =
                Rect::from_corners([0.0, 190.0 * h / base], [200.0 * w / base, 210.0 * h / base]);

            widget::Button::image(widgets.0)
                .h(45.0)
                .up_from(self.ids.multiplayer, 20.0)
                .label("Singleplayer")
                .label_color(color::WHITE)
                .center_justify_label()
                .padded_w_of(self.ids.body_middle_column, 40.0)
                .source_rectangle(base_rect)
                .hover_source_rectangle(hover_rect)
                .press_source_rectangle(press_rect)
                .set(self.ids.singleplayer, &mut ui);

            widget::Button::image(widgets.0)
                .h(45.0)
                .middle_of(self.ids.body_middle_column)
                .label("Multiplayer")
                .label_color(color::WHITE)
                .center_justify_label()
                .padded_w_of(self.ids.body_middle_column, 40.0)
                .source_rectangle(base_rect)
                .hover_source_rectangle(hover_rect)
                .press_source_rectangle(press_rect)
                .set(self.ids.multiplayer, &mut ui);
        }

        // Footer //

        // Litecraft and Minecraft version
        widget::Text::new(&format!(
            "Litecraft {}\nMinecraft {}",
            LITECRAFT_VERSION, MINECRAFT_VERSION
        ))
        .color(color::WHITE)
        .font_size(16)
        .bottom_left_of(self.ids.footer)
        .set(self.ids.version, &mut ui);

        // Credits
        widget::Text::new("Litecraft Team")
            .color(color::WHITE)
            .font_size(16)
            .bottom_right_of(self.ids.footer)
            .set(self.ids.copyright, &mut ui);

        SceneAction::None
    }
}