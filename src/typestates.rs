// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state


struct Luggage(u32);
struct CheckIn(u32);
struct OnLoading(u32);
struct OffLoading(u32);
struct AwaitingPickup(u32);
struct EndCustody(u32);

impl Luggage {
    fn new(id: u32) -> Self {
        Luggage(id)
    }

    fn check_in(&self) -> CheckIn {
        CheckIn(self.0)
    }
}

impl CheckIn {
    fn on_load(self) -> OnLoading {
        OnLoading(self.0)
    }
}

impl OnLoading {
    fn off_load(self) -> OffLoading {
        OffLoading(self.0)
    }
}

impl OffLoading {
    fn await_pickup(self) -> AwaitingPickup {
        AwaitingPickup(self.0)
    }
}

impl AwaitingPickup {
    fn pick_up(self) -> (Luggage, EndCustody) {
        (Luggage(self.0), EndCustody(self.0))
    }
}
fn main() {

    let id: u32 = 1;
    let luggage = Luggage(id);
    let luggage: (Luggage, _)= luggage.check_in().on_load().off_load().await_pickup().pick_up();

    println!("Luggage with id: {}, is picked up", luggage.0.0);
}