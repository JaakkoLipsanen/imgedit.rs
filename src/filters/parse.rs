use std::error::Error;
use super::Filter;

/// Parses a list of strings as filters
pub fn parse_filters(filters: &Vec<String>) -> Result<Vec<Filter>, Box<Error>> {
    filters
        .into_iter()
        .map(|filter| {
            let filter_name: Vec<&str> = filter.split('=').collect();
            let args: Option<Vec<&str>> = filter_name.get(1).map(|args| args.split(',').collect());

            match filter_name.first() {
                Some(&"identity") => Ok(Filter::Identity),
                Some(&"invert-color") => Ok(Filter::InvertColor),
                Some(&"brighten") => {
                    match args {
                        Some(args) => {
                            if args.len() != 1 {
                                bail!("Invalid amount of arguments for brighten filter. Only one argument is needed")
                            }

                            match args.first().unwrap().parse::<f32>() {
                                Ok(amount) => if amount >= 0.0 && amount <= 1.0 { Ok(Filter::Brighten { amount }) } else { bail!("Parameter supplied for brighten must be between [0, 1]") },
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
        assert_eq!(parse_filters(&vec!["identity".to_string(), "brighten=0.5".to_string()]).unwrap(), vec!(Filter::Identity, Filter::Brighten { amount: 0.5 }))
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
}