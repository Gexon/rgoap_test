extern crate rgoap;

use rgoap::{State, Action, plan};


fn main() {
    // The actions your planner will be allowed to use.
    let mut walk_to_dog = Action::new("walk_to_dog".to_string(), 1);
    walk_to_dog.pre_conditions.insert("dog_person".to_string(), true);
    walk_to_dog.post_conditions.insert("near_dog".to_string(), true);

    let mut dog_wiggles_tail = Action::new("dog_wiggles_tail".to_string(), 1);
    dog_wiggles_tail.pre_conditions.insert("dog_happy".to_string(), true);
    dog_wiggles_tail.post_conditions.insert("tails_wiggling".to_string(), true);

    let mut pet_dog = Action::new("pet_dog".to_string(), 1);
    pet_dog.pre_conditions.insert("near_dog".to_string(), true);
    pet_dog.post_conditions.insert("dog_happy".to_string(), true);

    let possible_actions = [walk_to_dog, pet_dog, dog_wiggles_tail];

    // This is the initial state of the world.
    let mut initial_state = State::new();
    initial_state.insert("near_dog".to_string(), false);
    initial_state.insert("dog_person".to_string(), true);
    initial_state.insert("dog_happy".to_string(), false);
    initial_state.insert("tails_wiggling".to_string(), false);

    // And this is the target state. Note that it doesn't have to include all of the states.
    let mut goal_state = State::new();
    goal_state.insert("tails_wiggling".to_string(), true);

    // Let's find which actions needs to happen to get there.
    let planned_actions = plan(&initial_state, &goal_state, &possible_actions).unwrap();

    // Are the actions what we expected?
    let planned_actions_names: Vec<String> =
        planned_actions.iter().map(|&action| action.name.clone()).collect();
    let expected_actions_names =
        vec!["walk_to_dog".to_string(), "pet_dog".to_string(), "dog_wiggles_tail".to_string()];

    assert_eq!(planned_actions_names, expected_actions_names);
}
