use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

pub mod enums;
pub mod structs;

pub enum ItemTypes {
    Pending(Pending),
    Done(Done),
}

pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::Done => ItemTypes::Done(Done::new(title)),
        TaskStatus::Pending => ItemTypes::Pending(Pending::new(title)),
    }
}
