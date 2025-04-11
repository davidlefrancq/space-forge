use crate::dal::planet_dao::PlanetDAO;

pub struct DAOFactory {
  planetDAO: PlanetDAO
}

impl DAOFactory {
  pub fn new() -> Self {
    DAOFactory {
      planetDAO: PlanetDAO
    }
  }

  pub fn planetDAO(&self) -> &PlanetDAO {
    &self.planetDAO
  }
}
