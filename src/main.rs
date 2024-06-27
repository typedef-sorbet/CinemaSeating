use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Person(u8);

impl<T> From<T> for Person 
where T: Into<u8> {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ConnectionType {
    Friends,
    InRelationship,
    Acquaintances,
    OneWayCrush
}

#[derive(Debug, Clone, Copy)]
struct Connection {
    person_a: Person,    // In OneWayCrush, this is the "from", or the "crusher"
    person_b: Person,    // In OneWayCrush, this is the "to", or the "crushee"
    kind: ConnectionType
}

fn cost(subject: &Person, seating: Vec<Person>, connections: &Vec<Connection>) -> i16 {
    let mut cost: i16 = 0;
    let subject_position = seating.iter().position(|p| *p == *subject).unwrap();

    // What's that? This makes the overall algo O(n^3)? Too bad!
    for (position, person) in seating.iter().enumerate() {
        if *person == *subject {
            continue;
        }

        for connection in connections.iter() {
            // Avoid double-counting
            if *person == connection.person_b {
                cost += match connection.kind {
                    ConnectionType::Friends => {
                        // Add three cost for every space between the two
                        (position.abs_diff(subject_position) as i16 - 1) * 3
                    },
                    ConnectionType::InRelationship => {
                        // Add five cost for every space between the two
                        (position.abs_diff(subject_position) as i16 - 1) * 5
                    },
                    ConnectionType::Acquaintances => {
                        // Add one cost for every space between the two
                        position.abs_diff(subject_position) as i16 - 1
                    },
                    ConnectionType::OneWayCrush => {
                        // This depends on whose wants we want to prioritize.
                        // Let's say that the one way crush is undesirable to the crushee.
                        // Subtract one cost for every space between the two.
                        (position.abs_diff(subject_position) as i16 - 1) * -1
                    }
                }
            }
        }
    }

    return cost;
}

fn add_connection<T>(connections: &mut HashMap<Person, Vec<Connection>>, _person_a: T, _person_b: T, kind: ConnectionType) 
where T: Into<Person> {
    let person_a = _person_a.into();
    let person_b = _person_b.into();

    if !connections.contains_key(&person_a) {
        connections.insert(person_a, Vec::new());
    }

    if !connections.contains_key(&person_b) {
        connections.insert(person_b, Vec::new());
    }

    let connection = Connection {
        person_a,
        person_b,
        kind
    };

    connections.get_mut(&person_a).unwrap().push(connection);
    connections.get_mut(&person_b).unwrap().push(connection);
}

fn main() {
    // Create connections
    let mut connections: HashMap<Person, Vec<Connection>> = HashMap::new();

    add_connection(&mut connections, 1, 2, ConnectionType::InRelationship);
    add_connection(&mut connections, 2, 3, ConnectionType::Friends);
    add_connection(&mut connections, 2, 6, ConnectionType::Friends);
    add_connection(&mut connections, 3, 6, ConnectionType::Acquaintances);
    add_connection(&mut connections, 3, 4, ConnectionType::Friends);
    add_connection(&mut connections, 4, 5, ConnectionType::Acquaintances);
    add_connection(&mut connections, 4, 7, ConnectionType::InRelationship);
    add_connection(&mut connections, 5, 7, ConnectionType::Friends);
    add_connection(&mut connections, 7, 8, ConnectionType::Acquaintances);
    add_connection(&mut connections, 8, 6, ConnectionType::OneWayCrush);

    let mut least_cost = i16::MAX;
    let mut least_cost_ordering: Vec<u8> = Vec::new();

    // This is gonna take a while...
    for ordering in (1..9).permutations(8) {
        let cost: i16 = (1..9).map(|pi| cost(&Person(pi), ordering.iter().map(|i| Person(*i)).collect_vec(), &connections[&Person(pi)])).sum();

        if cost < least_cost {
            least_cost = cost;
            least_cost_ordering = ordering;
        }
    }

    println!("The least-cost ordering for the given connections is {:?} with cost {}", least_cost_ordering, least_cost);

}
