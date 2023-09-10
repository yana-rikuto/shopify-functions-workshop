use serde::Serialize;
mod api;
use api::*;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input: input::Input = serde_json::from_reader(std::io::BufReader::new(std::io::stdin()))?;
    let mut out = std::io::stdout();
    let mut serializer = serde_json::Serializer::new(&mut out);
    function(input)?.serialize(&mut serializer)?;
    Ok(())
}
fn function(input: input::Input) -> Result<FunctionResult, Box<dyn std::error::Error>> {
    let config: input::Configuration = input.configuration();
    let cart_lines = input.cart.lines;

    if cart_lines.is_empty() || config.percentage == 0.0 {
        return Ok(FunctionResult {
            discounts: vec![],
            discount_application_strategy: DiscountApplicationStrategy::First,
        });
    }

    let mut targets = vec![];
    for line in cart_lines {
        if line.quantity >= config.quantity {
            targets.push(Target::ProductVariant {
                id: line.merchandise.id.unwrap_or_default(),
                quantity: None,
            });
        }
    }

    if targets.is_empty() {
        return Ok(FunctionResult {
            discounts: vec![],
            discount_application_strategy: DiscountApplicationStrategy::First,
        });
    }

    Ok(FunctionResult {
        discounts: vec![Discount {
            message: None,
            conditions: None,
            targets,
            value: Value::Percentage(Percentage {
                value: config.percentage,
            }),
        }],
        discount_application_strategy: DiscountApplicationStrategy::First,
    })
}
