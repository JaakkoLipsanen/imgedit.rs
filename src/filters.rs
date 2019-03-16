

use std::error::Error;

#[derive(Debug)]
pub enum Filter {
    Identity,
    InvertColor,
    Brighten { amount: f32 }
}

pub fn parse_filters(filters: Vec<String>) -> Result<Vec<Filter>, Box<Error>> {
    return filters
        .into_iter()
        .map(|filter| {
            let filter_name: Vec<&str> = filter.split("=").collect();
            let args: Option<Vec<&str>> = filter_name.get(1).map(|args| args.split(",").collect());

            return match filter_name.first() {
                Some(&"identity") => Ok(Filter::Identity),
                Some(&"invert-color") => Ok(Filter::InvertColor),
                Some(&"brighten") => {
                    match args {
                        Some(args) => {
                            if args.len() != 1 {
                                bail!("Invalid amount of arguments for brighten filter. Only one argument is needd")
                            }

                            match args.first().unwrap().parse::<f32>() {
                                Ok(amount) => Ok(Filter::Brighten { amount }),
                                Err(_err) => bail!("Invalid parameter supplied for brighten filter: must be a number")
                            }
                        },
                        None => bail!("Missing argument in brighten filter. Use: 'brighten=0.5'")
                    }

                }
                _ => bail!("Invalid filter argument '{}'", filter)
            }
        })
        .collect();
}
