#[derive(Debug)]
enum Status {
    Pending,
    InProgress,
    Active,
    Completed,
    Rejected
}

#[derive(Debug)]
enum GenderCategory {
   Male,Female
}
fn main() {
    let male = GenderCategory::Male;
    let female = GenderCategory::Female;

    println!("{:?}",male);
    println!("{:?}",female);
    let pending = Status::Pending;
    let in_progress = Status::InProgress;
    let active = Status::Active;
    let completed = Status::Completed;
    let rejected = Status::Rejected;

    println!("Pending status: {:?}", pending);
    println!("Inprogress status: {:?}", in_progress);
    println!("Active status: {:?}", active);
    println!("Completed status: {:?}", completed);
    println!("Rejected status: {:?}", rejected);
}
