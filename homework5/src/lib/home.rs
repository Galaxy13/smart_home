use crate::control::DeviceInterface;
use crate::helper_traits::Report;
use crate::room::Room;
use crate::Devices;
use std::collections::HashMap;

pub struct Home {
    name: String,
    rooms: HashMap<String, Room>,
}

impl Default for Home {
    fn default() -> Self {
        Home {
            name: String::from("Untitled"),
            rooms: HashMap::new(),
        }
    }
}

impl Home {
    pub fn new(home_name: String) -> Self {
        Home {
            name: home_name,
            rooms: HashMap::new(),
        }
    }

    pub fn home_name(&self) -> &String {
        &self.name
    }

    pub fn get_rooms_names(&self) -> Option<Vec<String>> {
        let mut room_names: Vec<String> = vec![];
        for room in self.rooms.keys() {
            room_names.push(room.clone())
        }
        match room_names.len() {
            0 => None,
            _ => Some(room_names),
        }
    }

    pub fn get_room_by_name(&mut self, room_name: &str) -> &mut Room {
        self.rooms
            .get_mut(room_name)
            .unwrap_or_else(|| panic!("No {} room in this Home instance", room_name))
    }

    pub fn get_rooms_list(&self) -> Option<Vec<&Room>> {
        let mut rooms = vec![];
        for room in self.rooms.values() {
            rooms.push(room);
        }
        match rooms.len() {
            0 => None,
            _ => Some(rooms),
        }
    }

    pub fn get_room_map(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn _get_room_devices(&mut self, room_name: &str) -> &HashMap<String, Box<dyn DeviceInterface>> {
        let room: &mut Room = self.rooms.get_mut(room_name).unwrap();
        room.get_devices()
    }

    pub fn create_room(&mut self, room_name: &str) {
        let is_room_in_home: bool = self.rooms.contains_key(room_name);
        if is_room_in_home {
            println!(
                "{} already in home {}. Try another room name",
                room_name, self.name
            )
        } else {
            self.rooms
                .insert(room_name.to_string(), Room::new(room_name.to_string()));
        }
    }

    fn create_report(&self) -> String {
        let mut owned_string = String::from("\nSmart home name: ");
        owned_string.push_str(self.home_name());
        owned_string.push_str("\nCurrent rooms number: ");
        owned_string.push_str(self.rooms.len().to_string().as_str());
        let mut room_names = self.get_rooms_names().unwrap_or_default();
        room_names.sort();
        for room_name in room_names {
            owned_string.push('\n');
            owned_string.push_str(room_name.as_str());
        }
        owned_string.push('\n');
        owned_string
    }

    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    pub fn create_device(&mut self, room_name: &str, device_type: Devices, device_name: &str) {
        let room: &mut Room = self
            .rooms
            .get_mut(room_name)
            .unwrap_or_else(|| panic!("No room {} found", room_name));
        room.add_device(device_type, device_name);
    }
}

impl Report for Home {
    fn string_report(&self) -> String {
        self.create_report()
    }
}
