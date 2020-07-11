use std::env::args;

fn wilson(positive: f32, negative: f32) -> (f32, f32) {
    let total = positive + negative;

    let phat = positive / total;
    let z = 1.96;

    let a = phat + z * z / (2.0 * total);
    let b = z * ((phat * (1.0 - phat) + z * z / (4.0 * total)) / total).sqrt();
    let c = 1.0 + z * z / total;

    ((a - b) / c, (a + b) / c)
}

fn main() -> Result<(), &'static str> {
    let mut args = args().skip(1);

    let positive = args.next()
        .ok_or("Positives count missing")?
        .parse::<f32>().map_err(|_| "Unable to parse positives count as a number")?;
    if positive < 0.0 {
        return Err("Positives count must be a non-negative number");
    }

    let negative = args.next()
        .ok_or("Negatives count missing")?
        .parse::<f32>().map_err(|_| "Unable to parse negatives count as a number")?;
    if negative < 0.0 {
        return Err("Negatives count must be a non-negative number");
    }


    let interval = wilson(positive, negative);
    println!("{} {}", interval.0, interval.1);

    Ok(())
}
