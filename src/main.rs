extern crate scraper;
use regex::Regex;
use reqwest::header::{HeaderMap, ACCEPT, ACCEPT_LANGUAGE, COOKIE, USER_AGENT};
use reqwest::Client;
use scraper::{Html, Selector};
use std::fs;
use std::num::ParseIntError;
use time::Instant;
use tokio::runtime::Runtime;

const URL: &str = "https://www.leboncoin.fr/recherche?category=9&locations=d_89%2Cd_21&real_estate_type=1&price=200000-350000&square=180-max&rooms=8-max";
const FILE_PATH: &str = "saved_web_pages.txt";

fn get_header() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/112.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(
        ACCEPT_LANGUAGE,
        "fr-FR,fr;q=0.9,en-US;q=0.8,en;q=0.7".parse().unwrap(),
    );
    headers.insert(COOKIE, "__Secure-Install=39262fcf-20df-47c2-8ce3-e9946e305901; __Secure-InstanceId=39262fcf-20df-47c2-8ce3-e9946e305901; didomi_token=eyJ1c2VyX2lkIjoiMTg1NmZhY2QtZjZjYS02YTBhLWE5YTQtM2E4MDkwYWY4YThjIiwiY3JlYXRlZCI6IjIwMjMtMDEtMDFUMjM6MzE6MjEuNDQwWiIsInVwZGF0ZWQiOiIyMDIzLTAxLTAxVDIzOjMxOjIxLjQ0MFoiLCJ2ZW5kb3JzIjp7ImVuYWJsZWQiOlsiZ29vZ2xlIiwiYzpsYmNmcmFuY2UiLCJjOnJldmxpZnRlci1jUnBNbnA1eCIsImM6ZGlkb21pIl19LCJwdXJwb3NlcyI6eyJlbmFibGVkIjpbInBlcnNvbm5hbGlzYXRpb25tYXJrZXRpbmciLCJwcml4IiwibWVzdXJlYXVkaWVuY2UiLCJleHBlcmllbmNldXRpbGlzYXRldXIiXX0sInZlbmRvcnNfbGkiOnsiZW5hYmxlZCI6WyJnb29nbGUiXX0sInZlcnNpb24iOjIsImFjIjoiRExXQkFBRUlBSXdBV1FCLWdHRkFQeUFra0JKWUVBd0lrZ1NrQXR5QnhBRHB3SFZnUU1BaW9CSE9DU2NFdFlLREFVSWdvdEJYT0N3VUZ0NExqQVhMQXdHQmhFREUwR1dvLkRMV0EtQUVJQUl3QV9RRENnSDVBU1NBa3NDQVlFU1FKU0FXNUE0Z0IwNERxd0lHQVJVQWpuQkpPQ1dzRkJnS0VRVVdncm5CWUtDMjhGeGdMbGdZREF3aUJpYURMVUFBQSJ9; euconsent-v2=CPk6AIAPk6AIAAHABBENCuCgAPLAAHLAAAAAIAtB_G_dTyPi-f59YvtwYQ1P4VQnoyACjgaNgwwJiRLBMI0EhmAIKAHqAAACIBAkICJAAQBlCAHAAAAA4IEAASMMAAAAIRAIIgCAAEAAAiJICABZCxAAAQAQgkwAABQAgAICABMgSDAAAAAAFAAAAAgAAAAAAAAAAAAAQAAAAAAAAggCACYatxAA2JY4E0gYRAAARhAEAUAIAKKAIWCAAgJEAAgjAAUYAAAAAoAAAAAAAAgBgAAAAEACEAAAADggEABAAgAAAAgEAgAAAAAQAAAYAAAAAABgAAAAAEABAAABQCAAAIAEABIEAAQAAAEAAAAAAAAAEAgAAAAAAAAAAAAAAACAGKAAwABBJcYABgACCS5AADAAEElw.flgADlgAAAAA; include_in_experiment=true; dblockV=1; ry_ry-l3b0nco_realytics=eyJpZCI6InJ5Xzk1NjFDNjRCLUJGQkUtNDIyRi05MzhGLTA3QjdEMjRDNTg2OCIsImNpZCI6bnVsbCwiZXhwIjoxNzA0MTUxODgzNTIyLCJjcyI6bnVsbH0%3D; __gsas=ID=19113b70d6735400:T=1672616101:S=ALNI_MYY83DWmFuW6rvBsqAuQvvT3-i9_Q; _hjSessionUser_2783207=eyJpZCI6IjU4YzEwMjY1LTg3YzYtNTQ3ZC05NWYzLTc2MWZlNmQ4NGVhMCIsImNyZWF0ZWQiOjE2NzI2MTU4ODUxODgsImV4aXN0aW5nIjp0cnVlfQ==; __gads=ID=50b6016777f03c0f:T=1672619633:S=ALNI_MYQ5EhWaQqMidN5sit80095iydOVg; _hjCachedUserAttributes=eyJhdHRyaWJ1dGVzIjp7InVzZXJfdHlwZSI6MH0sInVzZXJJZCI6bnVsbH0=; __aaxsc=1; aasd=9%7C1673025496839; _hjHasCachedUserAttributes=true; _gcl_au=1.1.1439786583.1681143525; _pin_unauth=dWlkPVkyWXpZVEUwTm1VdFpEazBNQzAwTVRObUxUbGxZMk10WVdRek9UYzVOVGRrWkRkbA; _scid=74dc54be-82a6-4a15-bcd1-94b04e95a1d0; _pbjs_userid_consent_data=2171422825241883; classifiedAB_adv=AB_TEST_ADVERTISING_CONTROL_GROUP; _scid_r=74dc54be-82a6-4a15-bcd1-94b04e95a1d0; _sctr=1%7C1682308800000; adview_clickmeter=search__listing__32__f0277020-d894-4bae-b0ff-8bc95fc2c374; _hjSession_2783207=eyJpZCI6IjZiMDUyZGRiLTlkY2ItNGM4NC05MDdiLWRmMTI0NzFjNjczNiIsImNyZWF0ZWQiOjE2ODQxOTA0NDQ2ODQsImluU2FtcGxlIjpmYWxzZX0=; _hjAbsoluteSessionInProgress=0; luat=eyJhbGciOiJSUzI1NiIsImtpZCI6IjgyYjFjNmYwLWRiM2EtNTQ2Ny1hYmI2LTJlMzAxNDViZjc3MiIsInR5cCI6IkpXVCJ9.eyJjbGllbnRfaWQiOiJsYmMtZnJvbnQtd2ViIiwiZGVwcmVjYXRlZF9zdG9yZV9pZCI6MzEyNTM0NTksImV4cCI6MTY4NDE5NzY0NSwiaWF0IjoxNjg0MTkwNDQ1LCJpZCI6IjMyMTZjNDI5LTA5NWMtNDYzOS1iZTc0LWRhMTBlZGZiZTkwMiIsImluc3RhbGxfaWQiOiIzOTI2MmZjZi0yMGRmLTQ3YzItOGNlMy1lOTk0NmUzMDU5MDEiLCJqdGkiOiIyN2E2M2YwZC0yMzZhLTQ5YWQtYWIzYi1lM2M0Zjc3YjUyZTUiLCJyZWZ1c2VkX3Njb3BlcyI6bnVsbCwicmVxdWVzdF9pZCI6IjdmOGY4NGNlLTAyMmUtNDFiNC1hMGIzLTJjMzRiZjg1MTZhZCIsInNjb3BlcyI6WyJsYmNncnAuYXV0aC50d29mYWN0b3IubWUuKiIsImxiYy4qLioubWUuKiIsImxiYy5lc2Nyb3dhY2NvdW50Lm1haW50ZW5hbmNlLnJlYWQiLCJvZmZsaW5lIiwibGJjZ3JwLmF1dGgudHdvZmFjdG9yLnNtcy5tZS5hY3RpdmF0ZSIsImxiY2dycC5hdXRoLnNlc3Npb24ubWUucmVhZCIsImxiYy4qLm1lLioiLCJsYmMucHJpdmF0ZSIsImxiY2dycC5hdXRoLnNlc3Npb24ubWUuZGVsZXRlIiwibGJjZ3JwLmF1dGguc2Vzc2lvbi5tZS5kaXNwbGF5IiwibGJjLmF1dGguZW1haWwucGFydC5jaGFuZ2UiLCJiZXRhLmxiYy5hdXRoLnR3b2ZhY3Rvci5tZS4qIl0sInNlc3Npb25faWQiOiJkOGUwMzNlMS05NGZmLTQ1NzItYTU0Yy00YzZjOTdkNzY1NTciLCJzdWIiOiJsYmM7NTFjODY3Y2UtM2Q0MC00YjE3LTk2MDYtYjllNjg5MjI3NWRhOzMxMjUzNDU5In0.kNuQrNlvVcz1yEBLGVQs9d1YNsbFKfBhXbZsQW2lqj0KUNhUPuVZQuMaB-IbA09eXZheAEWRjv5YyIXI6r7ifTHz0VlSxukHepeX1vA1Hg1CP4chRUlCsEhRbdmgqcqh6bTKGBOgBnMDdY0WYpVKjkVCJu2TmQFZGJeWGrEBbAMqb07ZGd-4qe_e611HBLNVH1dMs5fc3367NreDASyOtWaUvBDsS2-8teH-aY3St2I6BESuCIe6ZsIuDewZxrvD0WZl-ftGXHdIPv_z4ZCYJbfFKPaHNpZ_5rSsCVX67XSWMewyY85RT1J7FOwU45ROR5lMOxfboy5fW4k87ZofEI8WmvHyxMCqw2ABxkeLqcgV4SD-kBWVLn9N7gdZYu5k9vo4K1DaLN22PcgxFRm5sTsofcbYLLSXbwOgrlI31wd-2qIMv4Knt9k7WKWgYlQRj-iYYcm0HYE-iIaRtHzkxB3YXUy-Z9eFsZd5z5yvr5kS4MPiGZFKhG9ZA4p3wFNiJpDAFsnFrE4akv2ggFlMloLwNbcdYMoW7jtyaKVpKRqZwkHRsVOWtIlwKjk8xkNYScQs3lvosoIjph1Tp2OVmoKDpANQVP_wCJX9mNrCi1pD1Fj1JD4cUjHkMGKeT7G1qn9ZCyqo25Yt0lzLSFpwEXjCS2-u3gpQon4Z9Ut10Vo; atidvisitor=%7B%22name%22%3A%22atidvisitor%22%2C%22val%22%3A%7B%22vrn%22%3A%22-562498--619756-%22%2C%22an%22%3A%2231253459%22%2C%22ac%22%3A%221%22%7D%2C%22options%22%3A%7B%22path%22%3A%22%2F%22%2C%22session%22%3A34128000%2C%22end%22%3A34128000%7D%7D; ry_ry-l3b0nco_so_realytics=eyJpZCI6InJ5Xzk1NjFDNjRCLUJGQkUtNDIyRi05MzhGLTA3QjdEMjRDNTg2OCIsImNpZCI6bnVsbCwib3JpZ2luIjp0cnVlLCJyZWYiOm51bGwsImNvbnQiOm51bGwsIm5zIjpmYWxzZX0%3D; _hjIncludedInSessionSample_2783207=0; cto_bundle=vU3H-19xUTlqbWdhJTJCS2l6S1lodFpqVmIlMkJlRURLUEd0UmZjSEkwRlpwa3J1aXVKVzE3Um5HZiUyRjJVamNCTTQ4Z2ZMUVlNeUFzeWJhcEsxYWhybjBFaCUyRkNJU1hFWmN4eEgzcER5ZWJPWjRHVHBweWppZk85elB2QjNVT3AlMkJWZnZIaXJGUCUyQkN2Sk91bmFXQlE1OFNPMkc2cm9LWDNhWDJGNWVNRTlLJTJGT2FpUDY2TE5mTnZhcXpDWVlTJTJCUDdGQXlkNnY1N0F0djdVdXFOVFJxOFlmS1RuVXg0WjRlZyUzRCUzRA; __gpi=UID=00000bb99d62b052:T=1672619633:RT=1684192342:S=ALNI_MYI84CyTiIqsrsQGvm3MT8vQGI_vw; panoramaId_expiry=1684278744181; datadome=6n1h3iWxlpqFYoPQ97GEPDGFQaK1fUJgKaOoL6dfpte4Bp_RpLUNawVu2Nd_SngKWA92JsDoIY3QzCLNToCwaCjA4EVeTR-~pc9EogeuYzprka34uQawfM~O1RdQks8G; atauthority=%7B%22name%22%3A%22atauthority%22%2C%22val%22%3A%7B%22authority_name%22%3A%22default%22%2C%22visitor_mode%22%3A%22optin%22%7D%2C%22options%22%3A%7B%22end%22%3A%222024-06-13T23%3A13%3A36.102Z%22%2C%22path%22%3A%22%2F%22%7D%7D; utag_main=v_id:01856facdc24001ae96ace7fa91402075001906d00942$_sn:12$_ss:1$_st:1684194216134$_pn:1%3Bexp-session$ses_id:1684192324385%3Bexp-session; dblockS=369".parse().unwrap());

    return headers;
}

async fn get_html_online() -> Result<String, String> {
    let client = Client::new();

    let headers = get_header();

    let response = client.get(URL).headers(headers).send().await.unwrap();

    if response.status().is_success() {
        return Ok(response.text().await.unwrap());
    } else {
        let error_message = format!("La requête a échoué avec le code : {}", response.status());
        println!("{}", error_message);
        Err(error_message)
    }
}

async fn read_from_file_or_get_online() -> String {
    return match fs::read_to_string(&FILE_PATH) {
        Ok(content) => {
            println!("Using saved web page 📜");
            content
        }
        Err(err) => {
            println!("Erreur lors de la lecture du fichier : {}", err);
            get_html_online().await.unwrap()
        }
    };
}

fn check_is_price(input: &str) -> bool {
    let regex = Regex::new(r"\b(?:[1-9]\d{0,2}(?:[ \xa0]\d{3})*|0)(?:\.\d{1,2})?[ \xa0]*€(?:\s|$)")
        .unwrap();
    regex.is_match(input)
}

fn check_is_price_per_meter_square(input: &str) -> bool {
    input.ends_with("€/m²")
}

fn check_is_location(input: &str) -> bool {
    let regex = Regex::new(r"(?i)\b[a-zÀ-ÿ' -]+\b\s+\d{2}[ ]?\d{3}\b").unwrap();
    regex.is_match(input)
}

fn convert_price_string_to_u64(price_as_string: &str) -> Result<u64, ParseIntError> {
    let cleaned_input = price_as_string.replace(['\u{a0}', '€'], "");
    cleaned_input.parse::<u64>()
}

fn convert_price_per_meter_square_string_to_u64(
    price_per_meter_square_as_string: &str,
) -> Result<u32, ParseIntError> {
    let cleaned_input: String = price_per_meter_square_as_string
        .chars()
        .filter(|c| c.is_digit(10))
        .collect();

    cleaned_input.parse::<u32>()
}

fn parse_city_and_postal_code(location: &str) -> Option<Location> {
    let re = Regex::new(r"^(?P<city_name>[^\d]+)\s+(?P<postal_code>\d{5})$").unwrap();

    if let Some(captures) = re.captures(location) {
        let city_name = captures
            .name("city_name")
            .unwrap()
            .as_str()
            .trim()
            .to_string();
        let postal_code = captures.name("postal_code").unwrap().as_str().to_string();
        Some(Location {
            city_name: Some(city_name),
            postal_code: Some(postal_code),
        })
    } else {
        None
    }
}

#[derive(Debug)]
struct DataFromAds {
    price: Option<u64>,
    price_per_square_meter: Option<u32>,
    location: Option<Location>,
}

#[derive(Debug)]
struct Location {
    city_name: Option<String>,
    postal_code: Option<String>,
}

fn main() {
    let start = Instant::now();
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let body = read_from_file_or_get_online().await;

        let document = Html::parse_document(body.as_str());

        let selector = Selector::parse(r#"a[data-test-id="ad"]"#).unwrap();

        for element in document.select(&selector) {
            let children_with_text = element.text().enumerate();

            let mut ads = DataFromAds {
                price: None,
                price_per_square_meter: None,
                location: None,
            };

            for child_with_text in children_with_text {
                let childs_text = child_with_text.1;

                if check_is_price(childs_text) {
                    match convert_price_string_to_u64(childs_text) {
                        Ok(number) => ads.price = Some(number),
                        Err(err) => {
                            println!("La chaîne ne représente pas un entier valide: {}", err)
                        }
                    }
                } else if check_is_price_per_meter_square(childs_text) {
                    match convert_price_per_meter_square_string_to_u64(childs_text) {
                        Ok(number) => ads.price_per_square_meter = Some(number),
                        Err(err) => {
                            println!("La chaîne ne représente pas un entier valide: {}", err)
                        }
                    }
                } else if check_is_location(childs_text) {
                    ads.location = parse_city_and_postal_code(childs_text);
                } else {
                    // println!(
                    //     "La chaîne '{}' ne correspond à aucune des datas collectées",
                    //     childs_text
                    // );
                }
            }

            println!("{:?}", ads)
        }
    });

    let elapsed = start.elapsed();
    println!("Temps écoulé: {:?}", elapsed);
}
