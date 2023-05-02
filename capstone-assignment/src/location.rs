#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Country {
    UnitedStates,
    Canada,
    UnitedKingdom,
    Germany,
    France,
    Switzerland,
    Spain,
    Ireland,
    Japan,
    Australia,
    Brazil,
    SouthKorea,
    China,
    India,
}

#[derive(Debug, PartialEq)]
pub enum Continent {
    NorthAmerica,
    Europe,
    Asia,
    Oceania,
    SouthAmerica
}

impl Country {
    pub fn country_to_continent(&self) -> Continent {
        match self {
            Country::UnitedStates | Country::Canada => Continent::NorthAmerica,
            Country::UnitedKingdom 
            | Country::Germany 
            | Country::France 
            | Country::Switzerland 
            | Country::Spain
            | Country::Ireland => Continent::Europe,
            Country::Japan 
            | Country::China
            | Country::SouthKorea
            | Country::India => Continent::Asia,
            | Country::Australia => Continent::Oceania,
            Country::Brazil => Continent::SouthAmerica,
        }
    }
}

impl std::str::FromStr for Country {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UnitedStates" | "USA" => Ok(Country::UnitedStates),
            "Canada" => Ok(Country::Canada),
            "UnitedKingdom" | "UK" => Ok(Country::UnitedKingdom),
            "Germany" => Ok(Country::Germany),
            "France" => Ok(Country::France),
            "Switzerland" => Ok(Country::Switzerland),
            "Spain" => Ok(Country::Spain),
            "Ireland" => Ok(Country::Ireland),
            "Japan" => Ok(Country::Japan),
            "Australia" => Ok(Country::Australia),
            "Brazil" => Ok(Country::Brazil),
            "SouthKorea" | "South Korea" => Ok(Country::SouthKorea),
            "China" => Ok(Country::China),
            "India" => Ok(Country::India),
            _ => Err("Invalid country name"),
        }
    }
}
