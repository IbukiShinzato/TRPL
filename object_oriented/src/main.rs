extern crate object_oriented;

use std::io;

use object_oriented::AveragedCollection;

fn main() -> io::Result<()> {
    let mut averaged_collection = AveragedCollection::new(vec![], 0.0);

    let mut average = 0.0;

    averaged_collection.add(10);
    println!("{}", average);
    averaged_collection.update_average();
    average = averaged_collection.average();
    println!("{}", average);
    averaged_collection.add(5);
    averaged_collection.update_average();
    average = averaged_collection.average();
    println!("{}", average);
    averaged_collection.remove();
    averaged_collection.update_average();
    average = averaged_collection.average();
    println!("{}", average);
    Ok(())
}
