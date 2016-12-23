use mapper::{Mapper, APPLE_II_TEXT_WIDTH, APPLE_II_TEXT_HEIGHT};

use std::path::Path;

use sdl2::VideoSubsystem;
use sdl2::render::{Renderer, Texture, TextureAccess};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, INIT_PNG};

const FONT_PATH: &'static str = "resources/font.png";

const APPLE_II_SCREEN_WIDTH: usize = 280;
const APPLE_II_SCREEN_HEIGHT: usize = 192;

pub struct Monitor<'a>
{
    pub renderer: Renderer<'a>,
    pub font: Texture,
}

impl<'a> Monitor<'a>
{
    pub fn new(sdl_video: VideoSubsystem) -> Monitor<'a>
    {
        /* Perhaps have AppleII init SDL_Image? */
        let _sdl_img_ctx = image::init(INIT_PNG)
                                      .expect("Could not init SDL2 image.");

        let mut window = sdl_video.window("APPLE ][",
                                          APPLE_II_SCREEN_WIDTH as u32,
                                          APPLE_II_SCREEN_HEIGHT as u32)
                                  .position_centered()
                                  .build()
                                  .expect("Could not make window.");
        window.set_minimum_size(APPLE_II_SCREEN_WIDTH as u32,
                                APPLE_II_SCREEN_HEIGHT as u32)
              .expect("Could not set min size.");

        let mut renderer = window.renderer()
                                 .accelerated()
                                 .target_texture()
                                 .build()
                                 .expect("Could not make renderer");

        let window_format = renderer.window()
                                    .expect("Could not get window.")
                                    .window_pixel_format();
        let vbuf = renderer.create_texture(window_format,
                                           TextureAccess::Target,
                                           APPLE_II_SCREEN_WIDTH as u32,
                                           APPLE_II_SCREEN_HEIGHT as u32)
                           .expect("Could not create texture.");

        let font = renderer.load_texture(Path::new(FONT_PATH))
                           .expect("Could not load font file.");

        /* render_target() should be guarenteed to return Some(RenderTarget)
         * since we specified TextureAccess::Target.
         */
        renderer.render_target()
                .unwrap()
                .set(vbuf)
                .expect("Could not set render target");

        Monitor{
            renderer: renderer,
            font: font,
        }
    }

    fn draw_text_row(&mut self, memory: &mut Mapper, scr_base: usize, y: usize, cycles: u64)
    {
        static ROW_MAP: [usize; APPLE_II_TEXT_HEIGHT] = [
            0x000, 0x080, 0x100, 0x180, 0x200, 0x280, 0x300, 0x380,
            0x028, 0x0A8, 0x128, 0x1A8, 0x228, 0x2A8, 0x328, 0x3A8,
            0x050, 0x0D0, 0x150, 0x1D0, 0x250, 0x2D0, 0x350, 0x3D0,
        ];

        let base = scr_base + ROW_MAP[y];
        for x in 0..APPLE_II_TEXT_WIDTH
        {
            let mut character = memory.ram[base + x];
            let char_type = character >> 6;
            match char_type
            {
                0 =>
                    character |= 0x40,
                1 =>
                {
                    if cycles % 1000000 < 500000
                    {
                        character |= 0x40;
                    }
                    else
                    {
                        character &= 0x3F;
                    }
                },
                _ =>
                    character &= 0x3F,
            }
            let font_y = (character & 0x7) as i32 * 8;
            let font_x = ((character & 0x78) >> 3) as i32 * 7;
            let src = Some(Rect::new(font_x, font_y, 7, 8));
            let dst = Some(Rect::new(x as i32 * 7, y as i32 * 8, 7, 8));
            self.renderer.copy(&self.font,
                               src,
                               dst)
                         .expect("Could not copy texture.");
        }
    }

    fn draw_low_res_row(&mut self, memory: &mut Mapper, scr_base: usize, y: usize)
    {
        static ROW_MAP: [usize; APPLE_II_TEXT_HEIGHT] = [
            0x000, 0x080, 0x100, 0x180, 0x200, 0x280, 0x300, 0x380,
            0x028, 0x0A8, 0x128, 0x1A8, 0x228, 0x2A8, 0x328, 0x3A8,
            0x050, 0x0D0, 0x150, 0x1D0, 0x250, 0x2D0, 0x350, 0x3D0,
        ];
        static COLOR_MAP: [Color; 0x10] = [
            Color::RGB(0x00, 0x00, 0x00),
            Color::RGB(0xD0, 0x00, 0x30),
            Color::RGB(0x00, 0x00, 0x80),
            Color::RGB(0xFF, 0x00, 0xFF),
            Color::RGB(0x00, 0x80, 0x00),
            Color::RGB(0x80, 0x80, 0x80),
            Color::RGB(0x00, 0x00, 0xFF),
            Color::RGB(0x60, 0xA0, 0xFF),
            Color::RGB(0x80, 0x50, 0x00),
            Color::RGB(0xFF, 0x80, 0x00),
            Color::RGB(0xC0, 0xC0, 0xC0),
            Color::RGB(0xFF, 0x90, 0x80),
            Color::RGB(0x00, 0xFF, 0x00),
            Color::RGB(0xFF, 0xFF, 0x00),
            Color::RGB(0x40, 0xFF, 0x90),
            Color::RGB(0xFF, 0xFF, 0xFF),
        ];

        let base = scr_base + ROW_MAP[y];
        for x in 0..APPLE_II_TEXT_WIDTH
        {
            let colors = memory.ram[base + x];
            self.renderer.set_draw_color(COLOR_MAP[(colors & 0xF) as usize]);
            self.renderer.fill_rect(Rect::new(x as i32 * 7,
                                              y as i32 * 8,
                                              7,
                                              4))
                         .expect("Could not draw to screen.");
            self.renderer.set_draw_color(COLOR_MAP[(colors >> 4) as usize]);
            self.renderer.fill_rect(Rect::new(x as i32 * 7,
                                              y as i32 * 8 + 4,
                                              7,
                                              4))
                         .expect("Could not draw to screen.");
        }
    }

    fn draw_high_res_row(&mut self, memory: &mut Mapper, scr_base: usize, y: usize)
    {
        static ROW_MAP: [usize; APPLE_II_SCREEN_HEIGHT] = [
            0x0, 0x400, 0x800, 0xc00, 0x1000, 0x1400, 0x1800, 0x1c00, 
            0x80, 0x480, 0x880, 0xc80, 0x1080, 0x1480, 0x1880, 0x1c80, 
            0x100, 0x500, 0x900, 0xd00, 0x1100, 0x1500, 0x1900, 0x1d00, 
            0x180, 0x580, 0x980, 0xd80, 0x1180, 0x1580, 0x1980, 0x1d80, 
            0x200, 0x600, 0xa00, 0xe00, 0x1200, 0x1600, 0x1a00, 0x1e00, 
            0x280, 0x680, 0xa80, 0xe80, 0x1280, 0x1680, 0x1a80, 0x1e80, 
            0x300, 0x700, 0xb00, 0xf00, 0x1300, 0x1700, 0x1b00, 0x1f00, 
            0x380, 0x780, 0xb80, 0xf80, 0x1380, 0x1780, 0x1b80, 0x1f80, 
            0x28, 0x428, 0x828, 0xc28, 0x1028, 0x1428, 0x1828, 0x1c28, 
            0xa8, 0x4a8, 0x8a8, 0xca8, 0x10a8, 0x14a8, 0x18a8, 0x1ca8, 
            0x128, 0x528, 0x928, 0xd28, 0x1128, 0x1528, 0x1928, 0x1d28, 
            0x1a8, 0x5a8, 0x9a8, 0xda8, 0x11a8, 0x15a8, 0x19a8, 0x1da8, 
            0x228, 0x628, 0xa28, 0xe28, 0x1228, 0x1628, 0x1a28, 0x1e28, 
            0x2a8, 0x6a8, 0xaa8, 0xea8, 0x12a8, 0x16a8, 0x1aa8, 0x1ea8, 
            0x328, 0x728, 0xb28, 0xf28, 0x1328, 0x1728, 0x1b28, 0x1f28, 
            0x3a8, 0x7a8, 0xba8, 0xfa8, 0x13a8, 0x17a8, 0x1ba8, 0x1fa8, 
            0x50, 0x450, 0x850, 0xc50, 0x1050, 0x1450, 0x1850, 0x1c50, 
            0xd0, 0x4d0, 0x8d0, 0xcd0, 0x10d0, 0x14d0, 0x18d0, 0x1cd0, 
            0x150, 0x550, 0x950, 0xd50, 0x1150, 0x1550, 0x1950, 0x1d50, 
            0x1d0, 0x5d0, 0x9d0, 0xdd0, 0x11d0, 0x15d0, 0x19d0, 0x1dd0, 
            0x250, 0x650, 0xa50, 0xe50, 0x1250, 0x1650, 0x1a50, 0x1e50, 
            0x2d0, 0x6d0, 0xad0, 0xed0, 0x12d0, 0x16d0, 0x1ad0, 0x1ed0, 
            0x350, 0x750, 0xb50, 0xf50, 0x1350, 0x1750, 0x1b50, 0x1f50, 
            0x3d0, 0x7d0, 0xbd0, 0xfd0, 0x13d0, 0x17d0, 0x1bd0, 0x1fd0,
        ];

        let base = scr_base + ROW_MAP[y];
        let mut prev;
        let mut curr = 0;
        /* prevents the very first bit from not being seen */
        let mut next = memory.ram[base] & 0x1;
        let mut x = 0;
        for byte in 0..APPLE_II_SCREEN_WIDTH/7
        {
            let data = memory.ram[base + byte];
            let colorset = data & 0x80;
            for bit in 0..7
            {
                prev = curr;
                curr = next;
                next = data & (1 << bit);

                if curr != 0
                {
                    if (prev != 0) || (next != 0)
                    {
                        /* white */
                        self.renderer.set_draw_color(
                            Color::RGB(0xFF, 0xFF, 0xFF));
                    }
                    else if colorset != 0
                    {
                        if (x & 1) != 0
                        {
                            /* blue */
                            self.renderer.set_draw_color(
                                Color::RGB(0x00, 0x80, 0xFF));
                        }
                        else
                        {
                            /* red */
                            self.renderer.set_draw_color(
                                Color::RGB(0xF0, 0x50, 0x00));
                        }
                    }
                    else
                    {
                        if (x & 1) != 0
                        {
                            /* violet */
                            self.renderer.set_draw_color(
                                Color::RGB(0xA0, 0x00, 0xFF));
                        }
                        else
                        {
                            /* green */
                            self.renderer.set_draw_color(
                                Color::RGB(0x20, 0xC0, 0x00));
                        }
                    }
                }
                else if (prev != 0) && (next != 0)
                {
                    if colorset != 0
                    {
                        if (x & 1) != 0
                        {
                            /* red */
                            self.renderer.set_draw_color(
                                Color::RGB(0xF0, 0x50, 0x00));
                        }
                        else
                        {
                            /* blue */
                            self.renderer.set_draw_color(
                                Color::RGB(0x00, 0x80, 0xFF));
                        }
                    }
                    else
                    {
                        if (x & 1) != 0
                        {
                            /* green */
                            self.renderer.set_draw_color(
                                Color::RGB(0x20, 0xC0, 0x00));
                        }
                        else
                        {
                            /* violet */
                            self.renderer.set_draw_color(
                                Color::RGB(0xA0, 0x00, 0xFF));
                        }
                    }
                }
                else
                {
                    /* black */
                    self.renderer.set_draw_color(
                        Color::RGB(0x00, 0x00, 0x00));
                }
                self.renderer.draw_point(Point::new(x, y as i32))
                             .expect("Could not render point.");
                x += 1;
            }
        }
    }

    pub fn update_window(&mut self, memory: &mut Mapper, cycles: u64)
    {
        if memory.screen.graphics
        {
            if memory.screen.low_res
            {
                if memory.screen.primary
                {
                    for y in 0..APPLE_II_TEXT_HEIGHT
                    {
                        self.draw_low_res_row(memory, 0x400, y);
                    }
                }
                else
                {
                    for y in 0..APPLE_II_TEXT_HEIGHT
                    {
                        self.draw_low_res_row(memory, 0x800, y);
                    }
                }
            }
            else
            {
                if memory.screen.primary
                {
                    for y in 0..APPLE_II_SCREEN_HEIGHT
                    {
                        self.draw_high_res_row(memory, 0x2000, y);
                    }
                }
                else
                {
                    for y in 0..APPLE_II_SCREEN_HEIGHT
                    {
                        self.draw_high_res_row(memory, 0x4000, y);
                    }
                }
            }

            if !memory.screen.all
            {
                for y in APPLE_II_TEXT_HEIGHT - 4..APPLE_II_TEXT_HEIGHT
                {
                    self.draw_text_row(memory, 0x400, y, cycles);
                }
            }
        }
        else
        {
            if memory.screen.primary
            {
                for y in 0..APPLE_II_TEXT_HEIGHT
                {
                    self.draw_text_row(memory, 0x400, y, cycles);
                }
            }
            else
            {
                for y in 0..APPLE_II_TEXT_HEIGHT
                {
                    self.draw_text_row(memory, 0x800, y, cycles);
                }
            }
        }

        self.renderer.present();
        let vbuf = self.renderer.render_target()
                                .unwrap()
                                .reset()
                                .expect("Could not reset render target.")
                                .unwrap();
        let (w, h) = self.renderer.window().unwrap().size();
        self.renderer.set_logical_size(w, h)
                     .expect("Could not set logical size");
        self.renderer.copy(&vbuf, None, Some(Rect::new(0, 0, w, h)))
                     .expect("Could not copy texture.");
        self.renderer.present();
        self.renderer.render_target()
                     .unwrap()
                     .set(vbuf)
                     .expect("Could not set render target.");
    }
}
