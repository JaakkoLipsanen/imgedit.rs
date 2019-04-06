use std::error::Error;
use super::Filter;

/// Parses a list of strings as filters
pub fn parse_filters(filters: &[String]) -> Result<Vec<Filter>, Box<Error>> {
    filters
        .iter()
        .map(|filter| {
            let filter_name: Vec<&str> = filter.split('=').collect();
            let args: Option<Vec<&str>> = filter_name.get(1).map(|args| args.split(',').collect());

            match filter_name.first() {
                Some(&"identity") => Ok(Filter::Identity),
                Some(&"invert-color") => Ok(Filter::InvertColor),
                Some(&"greyscale") => Ok(Filter::Greyscale),
                Some(&"blur") => Ok(Filter::Blur),
                Some(&"unsharp-mask") => Ok(Filter::UnsharpMask),
                Some(&"hue-shift") => {
                    match args {
                        Some(args) => {
                            if args.len() != 1 {
                                bail!("Too many arguments for hue-shift filter. Only one argument is needed")
                            }

                            match args.first().unwrap().parse::<f32>() {
                                Ok(amount) => if amount >= 0.0 && amount <= 360.0 {
                                    Ok(Filter::HueShift { amount })
                                } else  {
                                    bail!("Parameter suppplied for hue-shift must be between [0, 360]")
                                }
                                Err(_err) => bail!("Invalid parameter supplied for hue-shift filter: must be a number")
                            }
                        }
                        None => bail!("Missing argument in hue-shift filter. Use: 'hue-shift=180'")
                    }
                }
                Some(&"brighten") => {
                    match args {
                        Some(args) => {
                            if args.len() != 1 {
                                bail!("Invalid amount of arguments for brighten filter. Only one argument is needed")
                            }

                            match args.first().unwrap().parse::<f32>() {
                                Ok(amount) => if amount >= 0.0 && amount <= 1.0 {
                                    Ok(Filter::Brighten { amount })
                                } else {
                                    bail!("Parameter supplied for brighten must be between [0, 1]")
                                },
                                Err(_err) => bail!("Invalid parameter supplied for brighten filter: must be a number")
                            }
                        },
                        None => bail!("Missing argument in brighten filter. Use: 'brighten=0.5'")
                    }

                }
                _ => bail!("Invalid filter argument '{}'", filter)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_filters_simple() {
        assert_eq!(parse_filters(&vec!["identity".to_string(), "invert-color".to_string()]).unwrap(), vec!(Filter::Identity, Filter::InvertColor))
    }

    #[test]
    fn test_parse_filters_with_arg() {
        assert_eq!(
            parse_filters(&vec!["identity".to_string(), "unsharp-mask".to_string(), "greyscale".to_string(), "brighten=0.5".to_string(), "hue-shift=280".to_string()]).unwrap(),
            vec!(Filter::Identity, Filter::UnsharpMask, Filter::Greyscale, Filter::Brighten { amount: 0.5 }, Filter::HueShift { amount: 280.0 })
        )
    }

    #[test]
    #[should_panic]
    fn test_parse_filters_invalid() {
        parse_filters(&vec!["non_existing_filter".to_string()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_filter_brighten_out_of_range() {
        parse_filters(&vec!["brighten=1.1".to_string()]).unwrap();
    }


    #[test]
    #[should_panic]
    fn test_parse_filter_brighten_invalid_amount() {
        parse_filters(&vec!["brighten=not_a_number".to_string()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_filter_brighten_no_args() {
        parse_filters(&vec!["brighten".to_string()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_filter_hue_shift_out_of_range() {
        parse_filters(&vec!["hue-shift=480".to_string()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_filter_hue_shift_invalid_amount() {
        parse_filters(&vec!["hue-shift=not_a_number".to_string()]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_parse_filter_hue_shift_no_args() {
        parse_filters(&vec!["hue-shift".to_string()]).unwrap();
    }
}