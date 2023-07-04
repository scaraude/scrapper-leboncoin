use std::fmt;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum SellerType {
    Owner,
    Professional,
}

impl fmt::Display for SellerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SellerType::Owner => write!(f, "Owner"),
            SellerType::Professional => write!(f, "Professional"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Ad {
    pub price: Option<u64>,
    pub price_per_square_meter: Option<u32>,
    pub location: Option<Location>,
    pub surface: Option<u64>,
    pub seller_type: SellerType,
    pub title: Option<String>,
    pub publication_date: Option<chrono::NaiveDateTime>,
}

pub type Ads = Vec<Ad>;

impl fmt::Display for Ad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(title) = self.title.to_owned() {
            write!(f, "Title: {}\n", title)?;
        }

        if let Some(price) = self.price {
            write!(f, "Price: {}\n", price)?;
        }

        if let Some(price_per_square_meter) = self.price_per_square_meter {
            write!(f, "Price per square meter: {}\n", price_per_square_meter)?;
        }

        if let Some(location) = &self.location {
            write!(f, "{}", location)?;
        }

        if let Some(surface) = &self.surface {
            write!(f, "Surface: {} m²\n", surface)?;
        }

        write!(f, "Seller type: {}\n", &self.seller_type)?;

        if let Some(publication_date) = &self.publication_date {
            write!(f, "Publication date: {}\n", publication_date)?;
        }

        Ok(())
    }
}

impl Ad {
    pub fn new_empty() -> Self {
        Ad {
            price: None,
            price_per_square_meter: None,
            location: None,
            surface: None,
            seller_type: SellerType::Owner,
            publication_date: None,
            title: None,
        }
    }
}

#[derive(Debug, Serialize)]
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
