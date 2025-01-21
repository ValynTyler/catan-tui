use crate::resource::Resource;

pub enum Terrain {
    Desert,
    Fields,
    Forest,
    Hills,
    Mountains,
    Pasture,
}

impl Terrain {
    pub fn produce(&self) -> Option<Resource> {
        match self {
            Terrain::Desert => None,
            Terrain::Fields => Some(Resource::Grain),
            Terrain::Forest => Some(Resource::Lumber),
            Terrain::Hills => Some(Resource::Brick),
            Terrain::Mountains => Some(Resource::Ore),
            Terrain::Pasture => Some(Resource::Wool),
        }
    }
}
