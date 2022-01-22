#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
struct UserId(u64);

#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((uid, _tid)) => println!("user {:?} joined", uid),
        Event::Leave((uid, tid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((_, _, msg)) => println!("broadcast: {}", msg),
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast: {}", msg);
    }
}

#[cfg(test)]
mod tests {
    use super::{Event, Topic, TopicId, User, UserId};

    #[test]
    fn it_works() {
        let alice = User {
            id: UserId(1),
            name: "Alice".into(),
            gender: super::Gender::Female,
        };
        let bob = User {
            id: UserId(2),
            name: "Bob".into(),
            gender: super::Gender::Male,
        };

        let topic = Topic {
            id: TopicId(1),
            name: "rust".into(),
            owner: UserId(1),
        };
        let event1 = Event::Join((alice.id, topic.id));
        let event2 = Event::Join((bob.id, topic.id));
        let event3 = Event::Message((alice.id, topic.id, "Hello World!".into()));

        println!(
            "event1: {:?}, event2: {:?}, event3: {:?}",
            event1, event2, event3
        );
    }
}
