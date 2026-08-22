use crate::{sandbox::*, tile::*};
use macroquack::prelude::*;
use macroquad::ui::{
    Drag, Ui, hash, root_ui,
    widgets::{self, Group},
};

pub struct GameUI {
    pub hot_bar: [(&'static str, Slot<Tile>); 10],
    pub tile_palette: Vec<Tile>,
    pub palette_visible: bool,
    pub item_dragging: bool,
    pub selected: usize,
    pub fit_command: Option<FittingCommand>,
    pub tiles_atlas: Option<Image>,
    pub tile_textures: Vec<Texture2D>,
}

pub enum FittingCommand {
    Unfit { target_slot: u64 },
    Fit { target_slot: u64, item: Tile },
    Refit { target_slot: u64, origin_slot: u64 },
}

pub struct Slot<T> {
    pub id: u64,
    pub item: Option<T>,
}

impl<T> Slot<T> {
    pub fn new(id: u64) -> Slot<T> {
        Slot { id, item: None }
    }

    pub fn new_with(id: u64, item: T) -> Slot<T> {
        Slot {
            id,
            item: Some(item),
        }
    }
}

impl GameUI {
    pub fn new() -> GameUI {
        GameUI {
            hot_bar: {
                [
                    ("1", Slot::new_with(hash!(), Tile::Sand)),
                    ("2", Slot::new_with(hash!(), Tile::Water)),
                    ("3", Slot::new_with(hash!(), Tile::Fire)),
                    ("4", Slot::new_with(hash!(), Tile::GunPowder)),
                    ("5", Slot::new(hash!())),
                    ("6", Slot::new(hash!())),
                    ("7", Slot::new(hash!())),
                    ("8", Slot::new(hash!())),
                    ("9", Slot::new(hash!())),
                    ("0", Slot::new(hash!())),
                ]
            },
            tile_palette: {
                let mut vec = Vec::<Tile>::with_capacity(Tile::Size as usize - 1);
                for i in 1..Tile::Size as u8 {
                    vec.push(i.into());
                }
                vec
            },
            palette_visible: true,
            item_dragging: false,
            selected: 0,
            fit_command: None,
            tiles_atlas: None,
            tile_textures: vec![],
        }
    }
    pub fn init_textures(&mut self) {
        if let Some(img) = &self.tiles_atlas {
            for i in 1..Tile::Size as i32 {
                self.tile_textures
                    .push(Texture2D::from_image(&img.sub_image(Rect {
                        x: (i % 16) as f32 * ATLAS_TEXTURES_SIZE as f32,
                        y: (i / 16) as f32 * ATLAS_TEXTURES_SIZE as f32,
                        w: ATLAS_TEXTURES_SIZE as f32,
                        h: ATLAS_TEXTURES_SIZE as f32,
                    })));
            }
        }
    }
    pub fn get_selected_tile(&self) -> Option<Tile> {
        self.hot_bar[self.selected].1.item
    }
    pub fn hot_bar(&mut self, ui: &mut Ui) {
        let fit_command = &mut self.fit_command;
        let item_dragging = &mut self.item_dragging;
        let slot_size = 52.0;

        for i in 0..self.hot_bar.len() {
            let (label, slot) = &self.hot_bar[i];
            let drag = Group::new(slot.id, vec2(slot_size + 4.0, slot_size + 4.0))
                .draggable(slot.item.is_some())
                .hoverable(*item_dragging)
                .highlight(*item_dragging || self.selected == i)
                .ui(ui, |ui| {
                    if let Some(ref item) = slot.item {
                        ui.texture(
                            self.tile_textures[*item as usize - 1].clone(),
                            slot_size,
                            slot_size,
                        );
                    } else {
                        ui.label(vec2(24.0, 24.0), label);
                    }
                });

            match drag {
                Drag::Dropped(_, Some(id)) if slot.item.is_some() => {
                    *fit_command = Some(FittingCommand::Refit {
                        target_slot: (id),
                        origin_slot: (slot.id),
                    });
                    *item_dragging = false;
                }
                Drag::Dropped(_, None) if slot.item.is_some() => {
                    *fit_command = Some(FittingCommand::Unfit {
                        target_slot: (slot.id),
                    });
                    *item_dragging = false;
                }
                Drag::Dropped(_, _) => unreachable!(),
                Drag::Dragging(pos, id) => {
                    debug!("slots pos: {:?}, id: {:?}", pos, id);

                    *item_dragging = true;
                }
                Drag::No => {}
            }
        }
    }
    pub fn tile_palette(&mut self, ui: &mut Ui) {
        let item_dragging = &mut self.item_dragging;
        let slot_size = 54.0;

        for (n, item) in self.tile_palette.iter().enumerate() {
            let drag = Group::new(
                hash!("tile_palette", n),
                vec2(slot_size + 4., slot_size + 4.),
            )
            .draggable(true)
            .ui(ui, |ui| {
                if self.tiles_atlas.is_some() {
                    ui.texture(
                        self.tile_textures[*item as usize - 1].clone(),
                        slot_size,
                        slot_size,
                    );
                }
            });

            match drag {
                Drag::Dropped(_, Some(id)) => {
                    self.fit_command = Some(FittingCommand::Fit {
                        target_slot: (id),
                        item: (*item),
                    });
                    *item_dragging = false;
                }
                Drag::Dropped(_, _) => {
                    *item_dragging = false;
                }
                Drag::Dragging(pos, id) => {
                    debug!("tile palette pos: {:?}, id: {:?}", pos, id);

                    *item_dragging = true;
                }
                _ => {}
            }
        }
    }
    pub fn set_item(&mut self, id: u64, item: Option<Tile>) {
        for i in 0..self.hot_bar.len() {
            if self.hot_bar[i].1.id == id {
                self.hot_bar[i].1.item = item;
                break;
            }
        }
    }
}
