use macroquack::prelude::IVec2;

#[repr(u8)]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Tile {
    Empty,
    Sand,
    Water,
    Steel,
    Fire,
    Lava,
    GunPowder,
    Wood,
    BurntWood,
    Coal,
    Leaf,
    Steam,
    Smoke,
    Snow,
    Size,
}

impl Tile {
    fn get_density(&self) -> f32 {
        match self {
            Tile::Empty => 0.0,
            Tile::Steam => 0.6,
            Tile::Smoke => 0.85,
            Tile::Leaf => 0.95,
            Tile::Water => 1.0,
            Tile::Sand => 1.6,
            Tile::GunPowder => 1.2,
            Tile::Wood => 0.77,

            _ => 1.0,
        }
    }

    pub fn is_liquid_or_gas(&self) -> bool {
        matches!(self, Self::Water | Self::Fire | Self::Steam | Self::Smoke)
    }

    pub fn is_solid(&self) -> bool {
        matches!(self, Self::Wood | Self::Steel)
    }

    pub fn can_swap(&self, other: &Tile, dir: IVec2) -> bool {
        if (self.is_liquid_or_gas() || other.is_liquid_or_gas())
            && !(self.is_solid() || other.is_solid())
        {
            (dir.y == 1 && self.get_density() > other.get_density())
                || (dir.y == -1 && self.get_density() < other.get_density())
                || *other == Tile::Empty
        } else {
            *other == Tile::Empty
        }
    }
}

// TODO: Move this into a macro
impl From<u8> for Tile {
    fn from(value: u8) -> Self {
        match value {
            1 => Tile::Sand,
            2 => Tile::Water,
            3 => Tile::Steel,
            4 => Tile::Fire,
            5 => Tile::Lava,
            6 => Tile::GunPowder,
            7 => Tile::Wood,
            8 => Tile::BurntWood,
            9 => Tile::Coal,
            10 => Tile::Leaf,
            11 => Tile::Steam,
            12 => Tile::Smoke,
            13 => Tile::Snow,
            _ => Tile::Empty,
        }
    }
}
