use std::fmt;

#[derive(Debug)]
pub struct Ad {
    pub price: Option<u64>,
    pub price_per_square_meter: Option<u32>,
    pub location: Option<Location>,
}

impl fmt::Display for Ad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(price) = self.price {
            write!(f, "Price: {}\n", price)?;
        }
        if let Some(price_per_square_meter) = self.price_per_square_meter {
            write!(f, "Price per square meter: {}\n", price_per_square_meter)?;
        }
        if let Some(location) = &self.location {
            write!(f, "{}", location)?;
        }
        Ok(())
    }
}

impl Ad {
    pub fn new(
        price: Option<u64>,
        price_per_square_meter: Option<u32>,
        location: Option<Location>,
    ) -> Self {
        Ad {
            price,
            price_per_square_meter,
            location,
        }
    }
}

#[derive(Debug)]
pub struct Location {
    pub city_name: Option<String>,
    pub postal_code: Option<String>,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(city_name) = &self.city_name {
            write!(f, "City: {}\n", city_name)?;
        }
        if let Some(postal_code) = &self.postal_code {
            write!(f, "Postal code: {}\n", postal_code)?;
        }
        Ok(())
    }
}
