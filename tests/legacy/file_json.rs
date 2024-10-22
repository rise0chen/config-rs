#![cfg(feature = "json")]

use serde_derive::Deserialize;

use std::path::PathBuf;

use config::{Config, File, FileFormat, Map, Value};
use float_cmp::ApproxEqUlps;

#[derive(Debug, Deserialize)]
struct Place {
    name: String,
    longitude: f64,
    latitude: f64,
    favorite: bool,
    telephone: Option<String>,
    reviews: u64,
    creator: Map<String, Value>,
    rating: Option<f32>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    debug: f64,
    production: Option<String>,
    place: Place,
    #[serde(rename = "arr")]
    elements: Vec<String>,
}

fn make() -> Config {
    let mut c = Config::default();
    c.merge(File::new("tests/Settings", FileFormat::Json))
        .unwrap();

    c
}

#[test]
fn test_file() {
    let c = make();

    // Deserialize the entire file as single struct
    let s: Settings = c.try_deserialize().unwrap();

    assert!(s.debug.approx_eq_ulps(&1.0, 2));
    assert_eq!(s.production, Some("false".to_owned()));
    assert_eq!(s.place.name, "Torre di Pisa");
    assert!(s.place.longitude.approx_eq_ulps(&43.722_498_5, 2));
    assert!(s.place.latitude.approx_eq_ulps(&10.397_052_2, 2));
    assert!(!s.place.favorite);
    assert_eq!(s.place.reviews, 3866);
    assert_eq!(s.place.rating, Some(4.5));
    assert_eq!(s.place.telephone, None);
    assert_eq!(s.elements.len(), 10);
    assert_eq!(s.elements[3], "4".to_owned());
    if cfg!(feature = "preserve_order") {
        assert_eq!(
            s.place
                .creator
                .into_iter()
                .collect::<Vec<(String, Value)>>(),
            vec![
                ("name".to_owned(), "John Smith".into()),
                ("username".into(), "jsmith".into()),
                ("email".into(), "jsmith@localhost".into()),
            ]
        );
    } else {
        assert_eq!(
            s.place.creator["name"].clone().into_string().unwrap(),
            "John Smith".to_owned()
        );
    }
}

#[test]
fn test_error_parse() {
    let mut c = Config::default();
    let res = c.merge(File::new("tests/Settings-invalid", FileFormat::Json));

    let path_with_extension: PathBuf = ["tests", "Settings-invalid.json"].iter().collect();

    assert!(res.is_err());
    assert_eq!(
        res.unwrap_err().to_string(),
        format!(
            "expected `:` at line 4 column 1 in {}",
            path_with_extension.display()
        )
    );
}

#[test]
fn test_json_vec() {
    let c = Config::default()
        .merge(File::from_str(
            r#"
            {
              "WASTE": ["example_dir1", "example_dir2"]
            }
            "#,
            FileFormat::Json,
        ))
        .unwrap()
        .clone();

    let v = c.get_array("WASTE").unwrap();
    let mut vi = v.into_iter();
    assert_eq!(vi.next().unwrap().into_string().unwrap(), "example_dir1");
    assert_eq!(vi.next().unwrap().into_string().unwrap(), "example_dir2");
    assert!(vi.next().is_none());
}
