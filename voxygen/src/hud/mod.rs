mod chat;

use crate::{
    render::Renderer,
    ui::{ScaleMode, Ui},
    window::Window,
    Error, GlobalState, PlayState, PlayStateResult,
};
use conrod_core::{
    color::TRANSPARENT,
    event::Input,
    image::Id as ImgId,
    text::font::Id as FontId,
    widget::{text_box::Event as TextBoxEvent, Button, Canvas, Image, TextBox, TitleBar},
    widget_ids, Borderable, Color,
    Color::Rgba,
    Colorable, Labelable, Positionable, Sizeable, Widget,
};

widget_ids! {
    struct Ids {
        //Bag and Inventory
        bag,
        bag_contents,
        bag_close,
        bag_map_open,
        //Halp
        halp,
        //ESC-Menu
        esc_bg,
        fireplace,
        menu_button_1,
        menu_button_2,
        menu_button_3,
        menu_button_4,
        menu_button_5,
        //Mini-Map
        mmap_frame,
        mmap_frame_bg,
        mmap_button_0,
        mmap_button_1,
        mmap_button_2,
        mmap_button_3,
        mmap_button_4,
        mmap_button_5,
        mmap_icons,
        mmap_location,
        //Action-Bar
        xp_bar,
        l_click,
        r_click,
        health_bar,
        mana_bar,
        sb_grid_l,
        sb_grid_r,
        sb_grid_bg_l,
        sb_grid_bg_r,
        //Window Frames
        window_frame_0,
        window_frame_1,
        window_frame_2,
        window_frame_3,
        window_frame_4,
        window_frame_5,
        //0 Settings-Window
        settings_bg,
        settings_icon,
        settings_button_mo,
        settings_close,
        settings_title,
            //Contents
            button_help,
            button_help2,
            interface,
            video,
            sound,
            gameplay,
            controls,
        //1 Social
        social_frame,
        social_bg,
        social_icon,
        social_close,
        social_title,
        //2 Map
        map_frame,
        map_bg,
        map_icon,
        map_close,
        map_title,
        //3 Spellbook
        spellbook_frame,
        spellbook_bg,
        spellbook_icon,
        spellbook_close,
        spellbook_title,
        //4 Charwindow
        charwindow_frame,
        charwindow_bg,
        charwindow_icon,
        charwindow_close,
        charwindow_title,
        //5 Quest-Log
        questlog_frame,
        questlog_bg,
        questlog_icon,
        questlog_close,
        questlog_title,

    }
}

// TODO: make macro to mimic widget_ids! for images ids or find another solution to simplify addition of new images.
struct Imgs {
    //Missing: ActionBar, Health/Mana/Energy Bar & Char Window BG/Frame
    // Bag
    bag: ImgId,
    bag_hover: ImgId,
    bag_press: ImgId,
    bag_open: ImgId,
    bag_open_hover: ImgId,
    bag_open_press: ImgId,
    bag_contents: ImgId,

    // Close button
    close_button: ImgId,
    close_button_hover: ImgId,
    close_button_press: ImgId,

    // Menu
    esc_bg: ImgId,
    fireplace: ImgId,
    button_dark: ImgId,
    button_dark_hover: ImgId,
    button_dark_press: ImgId,

    // MiniMap
    mmap_frame: ImgId,
    mmap_frame_bg: ImgId,
    mmap_icons: ImgId,

    // Settings at Mini-Map
    mmap_button: ImgId,
    mmap_button_hover: ImgId,
    mmap_button_press: ImgId,
    mmap_button_open: ImgId,

    // SkillBar Module
    sb_grid: ImgId,
    sb_grid_bg: ImgId,
    l_click: ImgId,
    r_click: ImgId,
    mana_bar: ImgId,
    health_bar: ImgId,
    xp_bar: ImgId,

    //Buff Frame(s)
    //buff_frame: ImgId,
    //buff_frame_bg: ImgId,
    //buff_frame_red: ImgId,
    //buff_frame_green: ImgId,

    //Missing: Buff Frame Animation
    window_frame: ImgId,
    //Settings-Window
    settings_bg: ImgId,
    settings_icon: ImgId,
    settings_button_mo: ImgId,
    check: ImgId,
    check_mo: ImgId,
    check_press: ImgId,
    check_checked: ImgId,
    check_checked_mo: ImgId,
    slider: ImgId,
    slider_indicator: ImgId,
    button_blank: ImgId,
    button_blue_mo: ImgId,
    button_blue_press: ImgId,
    //Social-Window
    social_bg: ImgId,
    social_icon: ImgId,
    //Map-Window
    map_bg: ImgId,
    map_icon: ImgId,
    map_frame: ImgId,
    //Spell Book Window
    spellbook_bg: ImgId,
    spellbook_icon: ImgId,
    //Char Window
    charwindow_bg: ImgId,
    charwindow_icon: ImgId,
    //Quest-Log Window
    questlog_bg: ImgId,
    questlog_icon: ImgId,
    //Halp
    //halp: ImgId,
}
impl Imgs {
    fn new(ui: &mut Ui, renderer: &mut Renderer) -> Imgs {
        let mut load = |filename| {
            let image = image::open(
                &[env!("CARGO_MANIFEST_DIR"), "/test_assets/ui/hud/", filename].concat(),
            )
            .unwrap();
            ui.new_image(renderer, &image).unwrap()
        };
        Imgs {
            // Bag
            bag: load("bag/icon/0_bag.png"),
            bag_hover: load("bag/icon/1_bag_hover.png"),
            bag_press: load("bag/icon/2_bag_press.png"),
            bag_open: load("bag/icon/3_bag_open.png"),
            bag_open_hover: load("bag/icon/4_bag_open_hover.png"),
            bag_open_press: load("bag/icon/5_bag_open_press.png"),
            bag_contents: load("bag/bg.png"),

            // Close button
            close_button: load("x/0_x.png"),
            close_button_hover: load("x/1_x_hover.png"),
            close_button_press: load("x/2_x_press.png"),

            // Esc-Menu
            esc_bg: load("menu/bg.png"),
            fireplace: load("menu/fireplace_1.png"),
            button_dark: load("menu/button_dark.png"),
            button_dark_hover: load("menu/button_dark_hover.png"),
            button_dark_press: load("menu/button_dark_press.png"),

            // MiniMap
            mmap_frame: load("mmap/mmap_frame.png"),
            mmap_frame_bg: load("mmap/mmap_bg.png"),
            mmap_icons: load("mmap/mmap_icons.png"),

            // Settings at Mini-Map
            mmap_button: load("mmap/grid.png"),
            mmap_button_hover: load("mmap/hover.png"),
            mmap_button_press: load("mmap/press.png"),
            mmap_button_open: load("mmap/open.png"),

            // Skillbar Module
            sb_grid: load("skill_bar/sbar_grid.png"),
            sb_grid_bg: load("skill_bar/sbar_grid_bg.png"),
            l_click: load("skill_bar/l.png"),
            r_click: load("skill_bar/r.png"),
            mana_bar: load("skill_bar/mana_bar.png"),
            health_bar: load("skill_bar/health_bar.png"),
            xp_bar: load("skill_bar/xp_bar.png"),

            //Buff Frame(s)
            //buff_frame: load("skill_bar/buff_frame.png"),
            //buff_frame_bg: load("skill_bar/buff_frame_bg.png"),
            //buff_frame_red: load("skill_bar/buff_frame_red.png"),
            //buff_frame_green: load("skill_bar/buff_frame_green.png"),

            //Missing: Buff Frame Animation (.gif ?!)
            window_frame: load("window_frame.png"),

            //Settings Window
            settings_bg: load("settings/bg.png"),
            settings_icon: load("settings/icon.png"),
            settings_button_mo: load("settings/mo.png"),
            check: load("settings/check.png"),
            check_mo: load("settings/check_mo.png"),
            check_press: load("settings/check_press.png"),
            check_checked: load("settings/check_checked.png"),
            check_checked_mo: load("settings/check_checked_mo.png"),
            slider: load("settings/slider.png"),
            slider_indicator: load("settings/slider_indicator.png"),
            button_blank: load("settings/button_blank.png"),
            button_blue_mo: load("settings/mo.png"),
            button_blue_press: load("settings/press.png"),

            //Social Window
            social_bg: load("social/bg.png"),
            social_icon: load("social/icon.png"),

            //Map Window
            map_bg: load("map/bg.png"),
            map_icon: load("map/icon.png"),
            map_frame: load("map/window_frame_map.png"),

            // Spell Book Window
            spellbook_bg: load("spellbook/bg.png"),
            spellbook_icon: load("spellbook/icon.png"),

            //Char Window
            charwindow_bg: load("charwindow/bg.png"),
            charwindow_icon: load("charwindow/icon.png"),

            //Quest-Log Window
            questlog_bg: load("questlog/bg.png"),
            questlog_icon: load("questlog/icon.png"),
        }
    }
}

pub struct Hud {
    ui: Ui,
    ids: Ids,
    imgs: Imgs,
    chat: chat::Chat,
    font_metamorph: FontId,
    font_whitney: FontId,
    show_help: bool,
    bag_open: bool,
    menu_open: bool,
    mmap_button_0: bool,
    mmap_button_1: bool,
    mmap_button_2: bool,
    mmap_button_3: bool,
    mmap_button_4: bool,
    mmap_button_5: bool,
    settings_interface: bool,
    settings_video: bool,
    settings_sound: bool,
    settings_gameplay: bool,
    settings_controls: bool,
}

impl Hud {
    pub fn new(window: &mut Window) -> Self {
        let mut ui = Ui::new(window).unwrap();
        // TODO: adjust/remove this, right now it is used to demonstrate window scaling functionality
        ui.scaling_mode(ScaleMode::RelativeToWindow([1920.0, 1080.0].into()));
        // Generate ids
        let mut ids = Ids::new(ui.id_generator());
        // Load images
        let imgs = Imgs::new(&mut ui, window.renderer_mut());
        // Load fonts
        let font_whitney = ui.new_font(
            conrod_core::text::font::from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/test_assets/font/Whitney-Book.ttf"
            ))
            .unwrap(),
        );
        let font_metamorph = ui.new_font(
            conrod_core::text::font::from_file(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/test_assets/font/Metamorphous-Regular.ttf"
            ))
            .unwrap(),
        );
        // Chat box
        let chat = chat::Chat::new(&mut ui);
        Self {
            ui,
            imgs,
            ids,
            chat,
            settings_interface: false,
            settings_controls: false,
            settings_gameplay: false,
            settings_sound: false,
            settings_video: false,
            show_help: true,
            bag_open: false,
            menu_open: false,
            mmap_button_0: false,
            mmap_button_1: false,
            mmap_button_2: false,
            mmap_button_3: false,
            mmap_button_4: false,
            mmap_button_5: false,

            font_metamorph,
            font_whitney,
        }
    }

    fn update_layout(&mut self) {
        let ref mut ui_widgets = self.ui.set_widgets();
        // Chat box
        self.chat.update_layout(ui_widgets);
        // Check if the bag was clicked
        // (can't use .was_clicked() because we are changing the image and this is after setting the widget which causes flickering as it takes a frame to change after the mouse button is lifted)
        if ui_widgets
            .widget_input(self.ids.bag)
            .clicks()
            .left()
            .count()
            % 2
            == 1
        {
            self.bag_open = !self.bag_open;
        }
        // Bag contents
        // Note that display_contents is set before checking if the bag was clicked
        // this ensures that the contents and open bag img are displayed on the same frame
        //Help Text
        if self.show_help {
            TitleBar::new(
                "
        Tab = Free Cursor
        Esc = Open/Close Menus
        Q = Back to Login

        F1 = Toggle this Window
        F2 = Toggle Interface

        M = Map
        I = Inventory
        L = Quest-Log
        C = Character Window
        O = Social
        P = Spellbook
        N = Settings",
                self.ids.halp,
            )
            .rgba(235.0, 170.0, 114.0, 0.3)
            .border_rgba(235.0, 170.0, 114.0, 1.0)
            .top_left_with_margins_on(ui_widgets.window, -18.0, -30.0)
            .w_h(300.0, 300.0)
            .font_id(self.font_whitney)
            .label_font_size(18)
            .label_rgba(0.0, 0.0, 0.0, 1.0)
            .left_justify_label()
            .set(self.ids.halp, ui_widgets);
            if Button::image(self.imgs.button_dark)
                .w_h(50.0, 30.0)
                .bottom_right_with_margin_on(self.ids.halp, 0.0)
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label("Close")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.button_help2, ui_widgets)
                .was_clicked()
            {
                self.show_help = false;
            };
        }
        if self.bag_open {
            // Contents
            Image::new(self.imgs.bag_contents)
                .w_h(1504.0 / 4.0, 1760.0 / 4.0)
                .bottom_right_with_margins(88.0, 68.0)
                .set(self.ids.bag_contents, ui_widgets);

            // X-button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 3.0, 244.0 * 0.22 / 3.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.bag_contents, 5.0, 17.0)
                .set(self.ids.bag_close, ui_widgets)
                .was_clicked()
            {
                self.bag_open = false;
            }
        }

        // Minimap frame and bg
        Image::new(self.imgs.mmap_frame_bg)
            .w_h(1750.0 / 8.0, 1650.0 / 8.0)
            .top_right_with_margins_on(ui_widgets.window, 20.0, 30.0)
            .set(self.ids.mmap_frame_bg, ui_widgets);

        Image::new(self.imgs.mmap_frame)
            .w_h(1750.0 / 8.0, 1650.0 / 8.0)
            .top_right_with_margins_on(ui_widgets.window, 20.0, 30.0)
            .set(self.ids.mmap_frame, ui_widgets);

        Image::new(self.imgs.mmap_icons)
            .w_h(448.0 / 14.93, 2688.0 / 14.93)
            .right_from(self.ids.mmap_frame, 0.0)
            .align_bottom_of(self.ids.mmap_frame)
            .set(self.ids.mmap_icons, ui_widgets);
        //Title
        //TODO Make it display the actual Location
        TitleBar::new("Unknown Location", self.ids.mmap_frame)
            .color(TRANSPARENT)
            .border_color(TRANSPARENT)
            .top_right_with_margins_on(self.ids.mmap_frame, 5.0, 0.0)
            .w_h(1750.0 / 8.0, 15.0)
            .label_font_size(14)
            .label_rgba(220.0, 220.0, 220.0, 0.8)
            .set(self.ids.mmap_location, ui_widgets);

        // Minimap Buttons

        //0 Settings
        if Button::image(self.imgs.mmap_button)
            .w_h(448.0 / 15.0, 448.0 / 15.0)
            .top_right_with_margins_on(self.ids.mmap_icons, 0.0, 0.0)
            .hover_image(self.imgs.mmap_button_hover)
            .press_image(self.imgs.mmap_button_press)
            .set(self.ids.mmap_button_0, ui_widgets)
            .was_clicked()
        {
            self.mmap_button_0 = !self.mmap_button_0;
            self.mmap_button_1 = false;
            self.mmap_button_2 = false;
            self.mmap_button_3 = false;
            self.mmap_button_4 = false;
            self.mmap_button_5 = false;
            self.bag_open = false;
        };
        //2 Map
        if Button::image(self.imgs.mmap_button)
            .w_h(448.0 / 15.0, 448.0 / 15.0)
            .down_from(self.ids.mmap_button_1, 0.0)
            .hover_image(self.imgs.mmap_button_hover)
            .press_image(self.imgs.mmap_button_press)
            .set(self.ids.mmap_button_2, ui_widgets)
            .was_clicked()
        {
            self.mmap_button_2 = !self.mmap_button_2;
            self.bag_open = false;
        };

        //Other Windows can only be accessed, when Settings are closed. Opening Settings will close all other Windows including the Bag.
        //Opening the Map won't close the windows displayed before.

        if self.mmap_button_0 == false && self.mmap_button_2 == false {
            //1 Social
            if Button::image(self.imgs.mmap_button)
                .w_h(448.0 / 15.0, 448.0 / 15.0)
                .down_from(self.ids.mmap_button_0, 0.0)
                .hover_image(self.imgs.mmap_button_hover)
                .press_image(self.imgs.mmap_button_press)
                .set(self.ids.mmap_button_1, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_1 = !self.mmap_button_1;
                self.mmap_button_2 = false;
                self.mmap_button_3 = false;
                self.mmap_button_5 = false;
            };

            //3 Spellbook
            if Button::image(self.imgs.mmap_button)
                .w_h(448.0 / 15.0, 448.0 / 15.0)
                .down_from(self.ids.mmap_button_2, 0.0)
                .hover_image(self.imgs.mmap_button_hover)
                .press_image(self.imgs.mmap_button_press)
                .set(self.ids.mmap_button_3, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_1 = false;
                self.mmap_button_2 = false;
                self.mmap_button_3 = !self.mmap_button_3;
                self.mmap_button_5 = false;
            };
            //4 Char-Window
            if Button::image(self.imgs.mmap_button)
                .w_h(448.0 / 15.0, 448.0 / 15.0)
                .down_from(self.ids.mmap_button_3, 0.0)
                .hover_image(self.imgs.mmap_button_hover)
                .press_image(self.imgs.mmap_button_press)
                .set(self.ids.mmap_button_4, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_4 = !self.mmap_button_4;
            };
            //5 Quest-Log
            if Button::image(self.imgs.mmap_button)
                .w_h(448.0 / 15.0, 448.0 / 15.0)
                .down_from(self.ids.mmap_button_4, 0.0)
                .hover_image(self.imgs.mmap_button_hover)
                .press_image(self.imgs.mmap_button_press)
                .set(self.ids.mmap_button_5, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_1 = false;
                self.mmap_button_2 = false;
                self.mmap_button_3 = false;
                self.mmap_button_5 = !self.mmap_button_5;
            };
        }

        // Skillbar Module

        //Experience-Bar
        Image::new(self.imgs.xp_bar)
            .w_h(2688.0 / 4.0, 116.0 / 4.0)
            .mid_bottom_of(ui_widgets.window)
            .set(self.ids.xp_bar, ui_widgets);

        //LeftGrid
        Image::new(self.imgs.sb_grid)
            .w_h(2240.0 / 8.0, 448.0 / 8.0)
            .up_from(self.ids.xp_bar, 0.0)
            .align_left_of(self.ids.xp_bar)
            .set(self.ids.sb_grid_l, ui_widgets);

        Image::new(self.imgs.sb_grid_bg)
            .w_h(2240.0 / 8.0, 448.0 / 8.0)
            .middle_of(self.ids.sb_grid_l)
            .set(self.ids.sb_grid_bg_l, ui_widgets);

        //Right Grid
        Image::new(self.imgs.sb_grid)
            .w_h(2240.0 / 8.0, 448.0 / 8.0)
            .up_from(self.ids.xp_bar, 0.0)
            .align_right_of(self.ids.xp_bar)
            .set(self.ids.sb_grid_r, ui_widgets);

        Image::new(self.imgs.sb_grid_bg)
            .w_h(2240.0 / 8.0, 448.0 / 8.0)
            .middle_of(self.ids.sb_grid_r)
            .set(self.ids.sb_grid_bg_r, ui_widgets);

        //Right and Left Click
        Image::new(self.imgs.l_click)
            .w_h(224.0 / 4.0, 320.0 / 4.0)
            .right_from(self.ids.sb_grid_bg_l, 0.0)
            .align_bottom_of(self.ids.sb_grid_bg_l)
            .set(self.ids.l_click, ui_widgets);

        Image::new(self.imgs.r_click)
            .w_h(224.0 / 4.0, 320.0 / 4.0)
            .left_from(self.ids.sb_grid_bg_r, 0.0)
            .align_bottom_of(self.ids.sb_grid_bg_r)
            .set(self.ids.r_click, ui_widgets);

        //Health- and Mana-Bar
        Image::new(self.imgs.health_bar)
            .w_h(1120.0 / 4.0, 96.0 / 4.0)
            .left_from(self.ids.l_click, 0.0)
            .align_top_of(self.ids.l_click)
            .set(self.ids.health_bar, ui_widgets);

        Image::new(self.imgs.mana_bar)
            .w_h(1120.0 / 4.0, 96.0 / 4.0)
            .right_from(self.ids.r_click, 0.0)
            .align_top_of(self.ids.r_click)
            .set(self.ids.mana_bar, ui_widgets);

        //Buffs/Debuffs

        //Buffs

        //Debuffs

        // Bag
        if self.mmap_button_2 == false {
            Button::image(if self.bag_open {
                self.imgs.bag_open
            } else {
                self.imgs.bag
            })
            .bottom_right_with_margin_on(ui_widgets.window, 20.0)
            .hover_image(if self.bag_open {
                self.imgs.bag_open_hover
            } else {
                self.imgs.bag_hover
            })
            .press_image(if self.bag_open {
                self.imgs.bag_open_press
            } else {
                self.imgs.bag_press
            })
            .w_h(420.0 / 6.0, 480.0 / 6.0)
            .set(self.ids.bag, ui_widgets);
        }
        if self.mmap_button_2 {
            Image::new(self.imgs.bag)
                .bottom_right_with_margin_on(ui_widgets.window, 20.0)
                .w_h(420.0 / 4.0, 480.0 / 4.0)
                .set(self.ids.bag_map_open, ui_widgets);
        }

        //Windows

        //Char Window will always appear at the left side. Other Windows either appear at the left side,
        //or when the Char Window is opened they will appear right from it.

        //0 Settings

        if self.mmap_button_0 {
            //BG
            Image::new(self.imgs.settings_bg)
                .middle_of(ui_widgets.window)
                .w_h(1648.0 / 2.5, 1952.0 / 2.5)
                .set(self.ids.settings_bg, ui_widgets);
            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 2.5, 244.0 * 0.22 / 2.5)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.settings_bg, 4.0, 4.0)
                .set(self.ids.settings_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_0 = false;
                self.settings_interface = true;
                self.settings_controls = false;
                self.settings_gameplay = false;
                self.settings_sound = false;
                self.settings_video = false;
            }

            //Title
            TitleBar::new("Settings", self.ids.settings_bg)
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(20.0)
                .w_h(600.0 / 3.0, 8.0)
                .label_font_size(30)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.settings_title, ui_widgets);
            //Icon
            Image::new(self.imgs.settings_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.settings_bg, -10.0, -10.0)
                .set(self.ids.settings_icon, ui_widgets);

            //1 Interface////////////////////////////
            if Button::image(self.imgs.button_blank)
                .w_h(304.0 / 2.5, 80.0 / 2.5)
                .hover_image(self.imgs.button_blue_mo)
                .press_image(self.imgs.button_blue_press)
                .top_left_with_margins_on(self.ids.settings_bg, 10.0, 10.0)
                .label("Interface")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.interface, ui_widgets)
                .was_clicked()
            {
                self.settings_interface = true;
                self.settings_controls = false;
                self.settings_gameplay = false;
                self.settings_sound = false;
                self.settings_video = false;
            }
            //Toggle Help
            if self.settings_interface {
                if Button::image(if self.show_help {
                    self.imgs.check_checked
                } else {
                    self.imgs.check
                })
                .w_h(288.0 / 10.0, 288.0 / 10.0)
                .middle_of(self.ids.settings_bg)
                .hover_image(if self.show_help {
                    self.imgs.check_checked_mo
                } else {
                    self.imgs.check_mo
                })
                .press_image(self.imgs.check_press)
                .label_x(conrod_core::position::Relative::Scalar(55.0))
                .label("Show Help")
                .label_font_size(12)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.button_help, ui_widgets)
                .was_clicked()
                {
                    self.show_help = !self.show_help;
                };
            }
            //2 Gameplay////////////////
            if Button::image(self.imgs.button_blank)
                .w_h(304.0 / 2.5, 80.0 / 2.5)
                .hover_image(self.imgs.button_blue_mo)
                .press_image(self.imgs.button_blue_press)
                .down_from(self.ids.interface, 10.0)
                .label("Gameplay")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.gameplay, ui_widgets)
                .was_clicked()
            {
                self.settings_interface = false;
                self.settings_controls = false;
                self.settings_gameplay = true;
                self.settings_sound = false;
                self.settings_video = false;
            }

            //3 Controls/////////////////////
            if Button::image(self.imgs.button_blank)
                .w_h(304.0 / 2.5, 80.0 / 2.5)
                .hover_image(self.imgs.button_blue_mo)
                .press_image(self.imgs.button_blue_press)
                .down_from(self.ids.interface, 52.0)
                .label("Controls")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.controls, ui_widgets)
                .was_clicked()
            {
                self.settings_interface = false;
                self.settings_controls = true;
                self.settings_gameplay = false;
                self.settings_sound = false;
                self.settings_video = false;
            }

            //4 Video////////////////////////////////
            if Button::image(self.imgs.button_blank)
                .w_h(304.0 / 2.5, 80.0 / 2.5)
                .hover_image(self.imgs.button_blue_mo)
                .press_image(self.imgs.button_blue_press)
                .down_from(self.ids.interface, 94.0)
                .label("Video")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.video, ui_widgets)
                .was_clicked()
            {
                self.settings_interface = false;
                self.settings_controls = false;
                self.settings_gameplay = false;
                self.settings_sound = false;
                self.settings_video = true;
            }

            //5 Sound///////////////////////////////
            if Button::image(self.imgs.button_blank)
                .w_h(304.0 / 2.5, 80.0 / 2.5)
                .hover_image(self.imgs.button_blue_mo)
                .press_image(self.imgs.button_blue_press)
                .down_from(self.ids.interface, 136.0)
                .label("Sound")
                .label_font_size(10)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.sound, ui_widgets)
                .was_clicked()
            {
                self.settings_interface = false;
                self.settings_controls = false;
                self.settings_gameplay = false;
                self.settings_sound = true;
                self.settings_video = false;
            }

            //Toggle Help
            if Button::image(if self.show_help {
                self.imgs.check_checked
            } else {
                self.imgs.check
            })
            .w_h(288.0 / 10.0, 288.0 / 10.0)
            .middle_of(self.ids.settings_bg)
            .hover_image(if self.show_help {
                self.imgs.check_checked_mo
            } else {
                self.imgs.check_mo
            })
            .press_image(self.imgs.check_press)
            .label_x(conrod_core::position::Relative::Scalar(55.0))
            .label("Show Help")
            .label_font_size(12)
            .label_rgba(220.0, 220.0, 220.0, 0.8)
            .set(self.ids.button_help, ui_widgets)
            .was_clicked()
            {
                self.show_help = !self.show_help;
            };
        }
        //1 Social

        if self.mmap_button_1 {
            //Frame
            if self.mmap_button_4 {
                Image::new(self.imgs.window_frame)
                    .right_from(self.ids.charwindow_frame, 20.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.social_frame, ui_widgets);
            } else {
                Image::new(self.imgs.window_frame)
                    .top_left_with_margins_on(ui_widgets.window, 200.0, 90.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.social_frame, ui_widgets);
            }

            //BG
            Image::new(self.imgs.social_bg)
                .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                .middle_of(self.ids.social_frame)
                .set(self.ids.social_bg, ui_widgets);

            //Icon
            Image::new(self.imgs.social_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.social_frame, -10.0, -10.0)
                .set(self.ids.social_icon, ui_widgets);

            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 4.0, 244.0 * 0.22 / 4.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.social_frame, 4.0, 4.0)
                .set(self.ids.social_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_1 = false;
            }
            //Title
            TitleBar::new("Social", self.ids.social_frame)
                .center_justify_label()
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(10.0)
                .w_h(500.0 / 3.0, 8.0)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.social_title, ui_widgets);
        }

        //3 Spell Book
        if self.mmap_button_3 {
            //Frame
            if self.mmap_button_4 {
                Image::new(self.imgs.window_frame)
                    .right_from(self.ids.charwindow_frame, 20.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.spellbook_frame, ui_widgets);
            } else {
                Image::new(self.imgs.window_frame)
                    .top_left_with_margins_on(ui_widgets.window, 200.0, 90.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.spellbook_frame, ui_widgets);
            }

            //BG
            Image::new(self.imgs.spellbook_bg)
                .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                .middle_of(self.ids.spellbook_frame)
                .set(self.ids.spellbook_bg, ui_widgets);

            //Icon
            Image::new(self.imgs.spellbook_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.spellbook_frame, -10.0, -10.0)
                .set(self.ids.spellbook_icon, ui_widgets);

            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 4.0, 244.0 * 0.22 / 4.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.spellbook_frame, 4.0, 4.0)
                .set(self.ids.spellbook_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_3 = false;
            }
            //Title
            TitleBar::new("Spellbook", self.ids.spellbook_frame)
                .center_justify_label()
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(10.0)
                .w_h(500.0 / 3.0, 8.0)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.spellbook_title, ui_widgets);
        }

        //4 Char-Window
        if self.mmap_button_4 {
            //Frame
            Image::new(self.imgs.window_frame)
                .top_left_with_margins_on(ui_widgets.window, 200.0, 90.0)
                .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                .set(self.ids.charwindow_frame, ui_widgets);

            //BG
            Image::new(self.imgs.charwindow_bg)
                .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                .middle_of(self.ids.charwindow_frame)
                .set(self.ids.charwindow_bg, ui_widgets);

            //Icon
            Image::new(self.imgs.charwindow_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.charwindow_frame, -10.0, -10.0)
                .set(self.ids.charwindow_icon, ui_widgets);

            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 4.0, 244.0 * 0.22 / 4.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.charwindow_frame, 4.0, 4.0)
                .set(self.ids.charwindow_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_4 = false;
            }
            //Title
            TitleBar::new("Character Name", self.ids.charwindow_frame) //Add in actual Character Name
                .center_justify_label()
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(10.0)
                .w_h(500.0 / 3.0, 8.0)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.charwindow_title, ui_widgets);
        }

        //5 Quest-Log
        if self.mmap_button_5 {
            //Frame
            if self.mmap_button_4 {
                Image::new(self.imgs.window_frame)
                    .right_from(self.ids.charwindow_frame, 20.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.questlog_frame, ui_widgets);
            } else {
                Image::new(self.imgs.window_frame)
                    .top_left_with_margins_on(ui_widgets.window, 200.0, 90.0)
                    .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                    .set(self.ids.questlog_frame, ui_widgets);
            }

            //BG
            Image::new(self.imgs.questlog_bg)
                .w_h(1648.0 / 4.0, 1952.0 / 4.0)
                .middle_of(self.ids.questlog_frame)
                .set(self.ids.questlog_bg, ui_widgets);

            //Icon
            Image::new(self.imgs.questlog_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.questlog_frame, -10.0, -10.0)
                .set(self.ids.questlog_icon, ui_widgets);

            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 4.0, 244.0 * 0.22 / 4.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.questlog_frame, 4.0, 4.0)
                .set(self.ids.questlog_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_5 = false;
            }
            //Title
            TitleBar::new("Quest-Log", self.ids.questlog_frame)
                .center_justify_label()
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(10.0)
                .w_h(500.0 / 3.0, 8.0)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.questlog_title, ui_widgets);
        }
        //2 Map
        if self.mmap_button_2 {
            //Frame
            Image::new(self.imgs.map_frame)
                .middle_of(ui_widgets.window)
                .w_h(5000.0 / 4.0, 3000.0 / 4.0)
                .set(self.ids.map_frame, ui_widgets);

            //BG
            Image::new(self.imgs.map_bg)
                .w_h(5000.0 / 4.0, 3000.0 / 4.0)
                .middle_of(self.ids.map_frame)
                .set(self.ids.map_bg, ui_widgets);

            //Icon
            Image::new(self.imgs.map_icon)
                .w_h(224.0 / 3.0, 224.0 / 3.0)
                .top_left_with_margins_on(self.ids.map_frame, -10.0, -10.0)
                .set(self.ids.map_icon, ui_widgets);

            //X-Button
            if Button::image(self.imgs.close_button)
                .w_h(244.0 * 0.22 / 1.0, 244.0 * 0.22 / 1.0)
                .hover_image(self.imgs.close_button_hover)
                .press_image(self.imgs.close_button_press)
                .top_right_with_margins_on(self.ids.map_frame, 1.0, 1.0)
                .set(self.ids.map_close, ui_widgets)
                .was_clicked()
            {
                self.mmap_button_2 = false;
            }
            //Title
            TitleBar::new("Map", self.ids.map_frame)
                .center_justify_label()
                .color(TRANSPARENT)
                .border_color(TRANSPARENT)
                .mid_top_with_margin(10.0)
                .w_h(500.0 / 3.0, 8.0)
                .label_font_size(50)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .set(self.ids.map_title, ui_widgets);
        }

        //ESC-MENU
        //Background
        if self.menu_open {
            Image::new(self.imgs.esc_bg)
                .w_h(228.0, 450.0)
                .middle_of(ui_widgets.window)
                .set(self.ids.esc_bg, ui_widgets);

            Image::new(self.imgs.fireplace)
                .w_h(180.0, 60.0)
                .mid_top_with_margin_on(self.ids.esc_bg, 50.0)
                .set(self.ids.fireplace, ui_widgets);

            //Settings
            if Button::image(self.imgs.button_dark)
                .mid_top_with_margin_on(self.ids.esc_bg, 115.0)
                .w_h(170.0, 50.0)
                .label("Settings")
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .label_font_size(20)
                .set(self.ids.menu_button_1, ui_widgets)
                .was_clicked()
            {
                self.menu_open = false;
                self.mmap_button_0 = true;
            };
            //Controls
            if Button::image(self.imgs.button_dark)
                .mid_top_with_margin_on(self.ids.esc_bg, 175.0)
                .w_h(170.0, 50.0)
                .label("Controls")
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .label_font_size(20)
                .set(self.ids.menu_button_2, ui_widgets)
                .was_clicked()
            {
                //self.menu_open = false;

            };
            //Servers

            if Button::image(self.imgs.button_dark)
                .mid_top_with_margin_on(self.ids.esc_bg, 235.0)
                .w_h(170.0, 50.0)
                .label("Servers")
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .label_font_size(20)
                .set(self.ids.menu_button_3, ui_widgets)
                .was_clicked()
            {
                //self.menu_open = false;

            };
            //Logout

            if Button::image(self.imgs.button_dark)
                .mid_top_with_margin_on(self.ids.esc_bg, 295.0)
                .w_h(170.0, 50.0)
                .label("Logout")
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .label_font_size(20)
                .set(self.ids.menu_button_4, ui_widgets)
                .was_clicked()
            {};
            //Quit

            if Button::image(self.imgs.button_dark)
                .mid_top_with_margin_on(self.ids.esc_bg, 355.0)
                .w_h(170.0, 50.0)
                .label("Quit")
                .hover_image(self.imgs.button_dark_hover)
                .press_image(self.imgs.button_dark_press)
                .label_rgba(220.0, 220.0, 220.0, 0.8)
                .label_font_size(20)
                .set(self.ids.menu_button_5, ui_widgets)
                .was_clicked()
            {
                use std::process;

                process::exit(0x0256);
            };
        }
    }

    pub fn toggle_menu(&mut self) {
        self.menu_open = !self.menu_open;
    }

    pub fn handle_event(&mut self, input: Input) {
        self.ui.handle_event(input);
    }

    pub fn maintain(&mut self, renderer: &mut Renderer) {
        self.update_layout();
        self.ui.maintain(renderer);
    }

    pub fn render(&self, renderer: &mut Renderer) {
        self.ui.render(renderer);
    }
    pub fn toggle_windows(&mut self) {
        if self.bag_open == false
            && self.menu_open == false
            && self.mmap_button_0 == false
            && self.mmap_button_1 == false
            && self.mmap_button_2 == false
            && self.mmap_button_3 == false
            && self.mmap_button_4 == false
            && self.mmap_button_5 == false
        {
            self.menu_open = true;
        } else {
            self.bag_open = false;
            self.menu_open = false;
            self.mmap_button_0 = false;
            self.mmap_button_1 = false;
            self.mmap_button_2 = false;
            self.mmap_button_3 = false;
            self.mmap_button_4 = false;
            self.mmap_button_5 = false;
        }
    }
}
