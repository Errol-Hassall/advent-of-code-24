mod day_one;

fn main() -> std::io::Result<()>{
    let total_distance = day_one::puzzle_one()?;

    println!("Total distance for puzzle one: {:?}", total_distance);

    let total_distance = day_one::puzzle_two()?;

    println!("Total distance for puzzle one: {:?}", total_distance);

    Ok(())
}
