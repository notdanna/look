//! Unit conversions (data/storage, length, mass, temperature, speed, time, volume).
//! Deterministic, zero-network instant answer.

use crate::{fmt, parse::UnitQuery, types::Answer};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dimension {
    Data,
    Length,
    Mass,
    Temperature,
    Speed,
    Time,
    Volume,
}

struct UnitDef {
    aliases: &'static [&'static str],
    display: &'static str,
    dimension: Dimension,
    /// Factor to convert 1 unit into the dimension's base unit.
    /// Not used for Temperature (which has affine formulas).
    to_base: f64,
}

static UNITS: &[UnitDef] = &[
    // --- Data (Base: Byte) ---
    UnitDef {
        aliases: &["b", "byte", "bytes"],
        display: "B",
        dimension: Dimension::Data,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["bit", "bits"],
        display: "bits",
        dimension: Dimension::Data,
        to_base: 0.125,
    },
    UnitDef {
        aliases: &["kb", "kilobyte", "kilobytes"],
        display: "KB",
        dimension: Dimension::Data,
        to_base: 1_000.0,
    },
    UnitDef {
        aliases: &["kib", "kibibyte", "kibibytes"],
        display: "KiB",
        dimension: Dimension::Data,
        to_base: 1_024.0,
    },
    UnitDef {
        aliases: &["mb", "megabyte", "megabytes"],
        display: "MB",
        dimension: Dimension::Data,
        to_base: 1_000_000.0,
    },
    UnitDef {
        aliases: &["mib", "mebibyte", "mebibytes"],
        display: "MiB",
        dimension: Dimension::Data,
        to_base: 1_048_576.0,
    },
    UnitDef {
        aliases: &["gb", "gigabyte", "gigabytes"],
        display: "GB",
        dimension: Dimension::Data,
        to_base: 1_000_000_000.0,
    },
    UnitDef {
        aliases: &["gib", "gibibyte", "gibibytes"],
        display: "GiB",
        dimension: Dimension::Data,
        to_base: 1_073_741_824.0,
    },
    UnitDef {
        aliases: &["tb", "terabyte", "terabytes"],
        display: "TB",
        dimension: Dimension::Data,
        to_base: 1_000_000_000_000.0,
    },
    UnitDef {
        aliases: &["tib", "tebibyte", "tebibytes"],
        display: "TiB",
        dimension: Dimension::Data,
        to_base: 1_099_511_627_776.0,
    },
    UnitDef {
        aliases: &["pb", "petabyte", "petabytes"],
        display: "PB",
        dimension: Dimension::Data,
        to_base: 1_000_000_000_000_000.0,
    },
    UnitDef {
        aliases: &["pib", "pebibyte", "pebibytes"],
        display: "PiB",
        dimension: Dimension::Data,
        to_base: 1_125_899_906_842_624.0,
    },
    UnitDef {
        aliases: &["kbit", "kilobit", "kilobits"],
        display: "Kbit",
        dimension: Dimension::Data,
        to_base: 125.0,
    },
    UnitDef {
        aliases: &["mbit", "megabit", "megabits"],
        display: "Mbit",
        dimension: Dimension::Data,
        to_base: 125_000.0,
    },
    UnitDef {
        aliases: &["gbit", "gigabit", "gigabits"],
        display: "Gbit",
        dimension: Dimension::Data,
        to_base: 125_000_000.0,
    },
    // --- Length (Base: Meter) ---
    UnitDef {
        aliases: &["mm", "millimeter", "millimeters"],
        display: "mm",
        dimension: Dimension::Length,
        to_base: 0.001,
    },
    UnitDef {
        aliases: &["cm", "centimeter", "centimeters"],
        display: "cm",
        dimension: Dimension::Length,
        to_base: 0.01,
    },
    UnitDef {
        aliases: &["m", "meter", "meters", "metre", "metres"],
        display: "m",
        dimension: Dimension::Length,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["km", "kilometer", "kilometers", "kilometre", "kilometres"],
        display: "km",
        dimension: Dimension::Length,
        to_base: 1_000.0,
    },
    UnitDef {
        aliases: &["in", "inch", "inches"],
        display: "inches",
        dimension: Dimension::Length,
        to_base: 0.0254,
    },
    UnitDef {
        aliases: &["ft", "foot", "feet"],
        display: "ft",
        dimension: Dimension::Length,
        to_base: 0.3048,
    },
    UnitDef {
        aliases: &["yd", "yard", "yards"],
        display: "yards",
        dimension: Dimension::Length,
        to_base: 0.9144,
    },
    UnitDef {
        aliases: &["mi", "mile", "miles"],
        display: "miles",
        dimension: Dimension::Length,
        to_base: 1_609.344,
    },
    // --- Mass (Base: Kilogram) ---
    UnitDef {
        aliases: &["mg", "milligram", "milligrams"],
        display: "mg",
        dimension: Dimension::Mass,
        to_base: 0.000_001,
    },
    UnitDef {
        aliases: &["g", "gram", "grams"],
        display: "g",
        dimension: Dimension::Mass,
        to_base: 0.001,
    },
    UnitDef {
        aliases: &["kg", "kilogram", "kilograms"],
        display: "kg",
        dimension: Dimension::Mass,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["ton", "tons", "tonne", "tonnes"],
        display: "tons",
        dimension: Dimension::Mass,
        to_base: 1_000.0,
    },
    UnitDef {
        aliases: &["oz", "ounce", "ounces"],
        display: "oz",
        dimension: Dimension::Mass,
        to_base: 0.028349523125,
    },
    UnitDef {
        aliases: &["lb", "lbs", "pound", "pounds"],
        display: "lbs",
        dimension: Dimension::Mass,
        to_base: 0.45359237,
    },
    // --- Temperature ---
    UnitDef {
        aliases: &["c", "celsius", "°c"],
        display: "°C",
        dimension: Dimension::Temperature,
        to_base: 0.0,
    },
    UnitDef {
        aliases: &["f", "fahrenheit", "°f"],
        display: "°F",
        dimension: Dimension::Temperature,
        to_base: 0.0,
    },
    UnitDef {
        aliases: &["k", "kelvin"],
        display: "K",
        dimension: Dimension::Temperature,
        to_base: 0.0,
    },
    // --- Speed (Base: m/s) ---
    UnitDef {
        aliases: &["km/h", "kmh", "kph"],
        display: "km/h",
        dimension: Dimension::Speed,
        to_base: 1.0 / 3.6,
    },
    UnitDef {
        aliases: &["mph"],
        display: "mph",
        dimension: Dimension::Speed,
        to_base: 0.44704,
    },
    UnitDef {
        aliases: &["m/s"],
        display: "m/s",
        dimension: Dimension::Speed,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["knot", "knots"],
        display: "knots",
        dimension: Dimension::Speed,
        to_base: 0.514444,
    },
    // --- Time (Base: Seconds) ---
    UnitDef {
        aliases: &["ms", "millisecond", "milliseconds"],
        display: "ms",
        dimension: Dimension::Time,
        to_base: 0.001,
    },
    UnitDef {
        aliases: &["s", "sec", "second", "seconds"],
        display: "s",
        dimension: Dimension::Time,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["min", "minute", "minutes"],
        display: "min",
        dimension: Dimension::Time,
        to_base: 60.0,
    },
    UnitDef {
        aliases: &["h", "hr", "hour", "hours"],
        display: "hours",
        dimension: Dimension::Time,
        to_base: 3_600.0,
    },
    UnitDef {
        aliases: &["day", "days"],
        display: "days",
        dimension: Dimension::Time,
        to_base: 86_400.0,
    },
    UnitDef {
        aliases: &["week", "weeks"],
        display: "weeks",
        dimension: Dimension::Time,
        to_base: 604_800.0,
    },
    // --- Volume (Base: Liter) ---
    UnitDef {
        aliases: &["ml", "milliliter", "milliliters"],
        display: "ml",
        dimension: Dimension::Volume,
        to_base: 0.001,
    },
    UnitDef {
        aliases: &["l", "liter", "liters", "litre", "litres"],
        display: "liters",
        dimension: Dimension::Volume,
        to_base: 1.0,
    },
    UnitDef {
        aliases: &["gal", "gallon", "gallons"],
        display: "gallons",
        dimension: Dimension::Volume,
        to_base: 3.78541,
    },
    UnitDef {
        aliases: &["cup", "cups"],
        display: "cups",
        dimension: Dimension::Volume,
        to_base: 0.236588,
    },
    UnitDef {
        aliases: &["pint", "pints"],
        display: "pints",
        dimension: Dimension::Volume,
        to_base: 0.473176,
    },
];

fn find_unit(token: &str) -> Option<&'static UnitDef> {
    let lowered = token.to_lowercase();
    let trimmed = lowered.trim_matches('°');
    UNITS.iter().find(|u| {
        u.aliases
            .iter()
            .any(|&a| a == lowered || a.trim_matches('°') == trimmed)
    })
}

pub fn is_compatible(from: &str, to: &str) -> bool {
    let u1 = find_unit(from);
    let u2 = find_unit(to);
    match (u1, u2) {
        (Some(a), Some(b)) => a.dimension == b.dimension,
        _ => false,
    }
}

pub fn answer(q: &UnitQuery) -> Option<Answer> {
    let u_from = find_unit(&q.from)?;
    let u_to = find_unit(&q.to)?;
    if u_from.dimension != u_to.dimension {
        return None;
    }

    let value = if u_from.dimension == Dimension::Temperature {
        convert_temperature(q.amount, u_from.aliases[0], u_to.aliases[0])
    } else {
        (q.amount * u_from.to_base) / u_to.to_base
    };

    let text = format!(
        "{} {} = {} {}",
        fmt::format_unit_number(q.amount),
        u_from.display,
        fmt::format_unit_number(value),
        u_to.display
    );
    Some(Answer::text(text, "Converter"))
}

fn convert_temperature(amount: f64, from_alias: &str, to_alias: &str) -> f64 {
    // Normalize to Celsius
    let celsius = match from_alias {
        "c" | "celsius" | "°c" => amount,
        "f" | "fahrenheit" | "°f" => (amount - 32.0) * 5.0 / 9.0,
        "k" | "kelvin" => amount - 273.15,
        _ => amount,
    };

    // Convert from Celsius to target
    match to_alias {
        "c" | "celsius" | "°c" => celsius,
        "f" | "fahrenheit" | "°f" => (celsius * 9.0 / 5.0) + 32.0,
        "k" | "kelvin" => celsius + 273.15,
        _ => celsius,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_data_storage() {
        let q = UnitQuery {
            amount: 1024.0,
            from: "mb".to_string(),
            to: "gb".to_string(),
        };
        let ans = answer(&q).unwrap();
        assert_eq!(ans.text, "1,024 MB = 1.024 GB");

        let q2 = UnitQuery {
            amount: 1024.0,
            from: "mib".to_string(),
            to: "gib".to_string(),
        };
        let ans2 = answer(&q2).unwrap();
        assert_eq!(ans2.text, "1,024 MiB = 1 GiB");

        let q3 = UnitQuery {
            amount: 16.0,
            from: "gb".to_string(),
            to: "mb".to_string(),
        };
        let ans3 = answer(&q3).unwrap();
        assert_eq!(ans3.text, "16 GB = 16,000 MB");
    }

    #[test]
    fn converts_temperature() {
        let q = UnitQuery {
            amount: 32.0,
            from: "f".to_string(),
            to: "c".to_string(),
        };
        assert_eq!(answer(&q).unwrap().text, "32 °F = 0 °C");

        let q2 = UnitQuery {
            amount: 100.0,
            from: "c".to_string(),
            to: "f".to_string(),
        };
        assert_eq!(answer(&q2).unwrap().text, "100 °C = 212 °F");
    }

    #[test]
    fn converts_length() {
        let q = UnitQuery {
            amount: 10.0,
            from: "km".to_string(),
            to: "m".to_string(),
        };
        assert_eq!(answer(&q).unwrap().text, "10 km = 10,000 m");
    }

    #[test]
    fn rejects_incompatible_dimensions() {
        let q = UnitQuery {
            amount: 10.0,
            from: "mb".to_string(),
            to: "km".to_string(),
        };
        assert!(answer(&q).is_none());
    }
}
