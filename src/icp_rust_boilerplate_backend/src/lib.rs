#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Restaurant {
    id: u64,
    name: String,
    location: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Staff {
    id: u64,
    restaurant_id: u64,
    name: String,
    position: String,
    schedule: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct MenuItem {
    id: u64,
    restaurant_id: u64,
    name: String,
    description: String,
    price: f64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Order {
    id: u64,
    restaurant_id: u64,
    items: Vec<u64>, // Menu item IDs
    total: f64,
    status: String, // e.g., "pending", "completed"
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Reservation {
    id: u64,
    restaurant_id: u64,
    name: String,
    date_time: u64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct InventoryItem {
    id: u64,
    restaurant_id: u64,
    name: String,
    quantity: u32,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    restaurant_id: u64,
    description: String,
    amount: f64,
    created_at: u64,
}

impl Storable for Restaurant {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Restaurant {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Staff {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Staff {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for MenuItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for MenuItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Order {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Order {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Reservation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reservation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for InventoryItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for InventoryItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static RESTAURANT_STORAGE: RefCell<StableBTreeMap<u64, Restaurant, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static STAFF_STORAGE: RefCell<StableBTreeMap<u64, Staff, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static MENU_STORAGE: RefCell<StableBTreeMap<u64, MenuItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static ORDER_STORAGE: RefCell<StableBTreeMap<u64, Order, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static RESERVATION_STORAGE: RefCell<StableBTreeMap<u64, Reservation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static INVENTORY_STORAGE: RefCell<StableBTreeMap<u64, InventoryItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static EXPENSE_STORAGE: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct RestaurantPayload {
    name: String,
    location: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct StaffPayload {
    restaurant_id: u64,
    name: String,
    position: String,
    schedule: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct MenuItemPayload {
    restaurant_id: u64,
    name: String,
    description: String,
    price: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct OrderPayload {
    restaurant_id: u64,
    items: Vec<u64>, // Menu item IDs
    total: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ReservationPayload {
    restaurant_id: u64,
    name: String,
    date_time: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct InventoryItemPayload {
    restaurant_id: u64,
    name: String,
    quantity: u32,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ExpensePayload {
    restaurant_id: u64,
    description: String,
    amount: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

#[ic_cdk::update]
fn create_restaurant(payload: RestaurantPayload) -> Result<Restaurant, Message> {
    if payload.name.is_empty() || payload.location.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'location' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let restaurant = Restaurant {
        id,
        name: payload.name,
        location: payload.location,
        created_at: current_time(),
    };
    RESTAURANT_STORAGE.with(|storage| storage.borrow_mut().insert(id, restaurant.clone()));
    Ok(restaurant)
}

#[ic_cdk::query]
fn get_restaurants() -> Result<Vec<Restaurant>, Message> {
    RESTAURANT_STORAGE.with(|storage| {
        let restaurants: Vec<Restaurant> = storage
            .borrow()
            .iter()
            .map(|(_, restaurant)| restaurant.clone())
            .collect();

        if restaurants.is_empty() {
            Err(Message::NotFound("No restaurants found".to_string()))
        } else {
            Ok(restaurants)
        }
    })
}

#[ic_cdk::query]
fn get_restaurant_by_id(id: u64) -> Result<Restaurant, Message> {
    RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == id)
            .map(|(_, restaurant)| restaurant.clone())
            .ok_or(Message::NotFound("Restaurant not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_staff(payload: StaffPayload) -> Result<Staff, Message> {
    if payload.name.is_empty() || payload.position.is_empty() || payload.schedule.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'position', and 'schedule' are provided.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let staff = Staff {
        id,
        restaurant_id: payload.restaurant_id,
        name: payload.name,
        position: payload.position,
        schedule: payload.schedule,
        created_at: current_time(),
    };
    STAFF_STORAGE.with(|storage| storage.borrow_mut().insert(id, staff.clone()));
    Ok(staff)
}

#[ic_cdk::query]
fn get_staffs() -> Result<Vec<Staff>, Message> {
    STAFF_STORAGE.with(|storage| {
        let staff: Vec<Staff> = storage
            .borrow()
            .iter()
            .map(|(_, staff)| staff.clone())
            .collect();

        if staff.is_empty() {
            Err(Message::NotFound("No staff found".to_string()))
        } else {
            Ok(staff)
        }
    })
}

#[ic_cdk::query]
fn get_staff_by_id(id: u64) -> Result<Staff, Message> {
    STAFF_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, staff)| staff.id == id)
            .map(|(_, staff)| staff.clone())
            .ok_or(Message::NotFound("Staff not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_menu_item(payload: MenuItemPayload) -> Result<MenuItem, Message> {
    if payload.name.is_empty() || payload.description.is_empty() || payload.price <= 0.0 {
        return Err(Message::InvalidPayload(
            "Ensure 'name', 'description', and 'price' are provided and valid.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let menu_item = MenuItem {
        id,
        restaurant_id: payload.restaurant_id,
        name: payload.name,
        description: payload.description,
        price: payload.price,
        created_at: current_time(),
    };
    MENU_STORAGE.with(|storage| storage.borrow_mut().insert(id, menu_item.clone()));
    Ok(menu_item)
}

#[ic_cdk::query]
fn get_menu_items() -> Result<Vec<MenuItem>, Message> {
    MENU_STORAGE.with(|storage| {
        let menu_items: Vec<MenuItem> = storage
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect();

        if menu_items.is_empty() {
            Err(Message::NotFound("No menu items found".to_string()))
        } else {
            Ok(menu_items)
        }
    })
}

#[ic_cdk::query]
fn get_menu_item_by_id(id: u64) -> Result<MenuItem, Message> {
    MENU_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, item)| item.id == id)
            .map(|(_, item)| item.clone())
            .ok_or(Message::NotFound("Menu item not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_order(payload: OrderPayload) -> Result<Order, Message> {
    if payload.items.is_empty() || payload.total <= 0.0 {
        return Err(Message::InvalidPayload(
            "Ensure 'items' are provided and 'total' is valid.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    for item_id in &payload.items {
        let item_exists = MENU_STORAGE.with(|storage| {
            storage
                .borrow()
                .iter()
                .any(|(_, item)| item.id == *item_id)
        });
        if !item_exists {
            return Err(Message::NotFound(format!("Menu item with ID {} not found", item_id)));
        }
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let order = Order {
        id,
        restaurant_id: payload.restaurant_id,
        items: payload.items,
        total: payload.total,
        status: "pending".to_string(),
        created_at: current_time(),
    };
    ORDER_STORAGE.with(|storage| storage.borrow_mut().insert(id, order.clone()));
    Ok(order)
}

#[ic_cdk::query]
fn get_orders() -> Result<Vec<Order>, Message> {
    ORDER_STORAGE.with(|storage| {
        let orders: Vec<Order> = storage
            .borrow()
            .iter()
            .map(|(_, order)| order.clone())
            .collect();

        if orders.is_empty() {
            Err(Message::NotFound("No orders found".to_string()))
        } else {
            Ok(orders)
        }
    })
}

#[ic_cdk::query]
fn get_order_by_id(id: u64) -> Result<Order, Message> {
    ORDER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, order)| order.id == id)
            .map(|(_, order)| order.clone())
            .ok_or(Message::NotFound("Order not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_reservation(payload: ReservationPayload) -> Result<Reservation, Message> {
    if payload.name.is_empty() || payload.date_time == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'date_time' are provided.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let reservation = Reservation {
        id,
        restaurant_id: payload.restaurant_id,
        name: payload.name,
        date_time: payload.date_time,
        created_at: current_time(),
    };
    RESERVATION_STORAGE.with(|storage| storage.borrow_mut().insert(id, reservation.clone()));
    Ok(reservation)
}

#[ic_cdk::query]
fn get_reservations() -> Result<Vec<Reservation>, Message> {
    RESERVATION_STORAGE.with(|storage| {
        let reservations: Vec<Reservation> = storage
            .borrow()
            .iter()
            .map(|(_, reservation)| reservation.clone())
            .collect();

        if reservations.is_empty() {
            Err(Message::NotFound("No reservations found".to_string()))
        } else {
            Ok(reservations)
        }
    })
}

#[ic_cdk::query]
fn get_reservation_by_id(id: u64) -> Result<Reservation, Message> {
    RESERVATION_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, reservation)| reservation.id == id)
            .map(|(_, reservation)| reservation.clone())
            .ok_or(Message::NotFound("Reservation not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_inventory_item(payload: InventoryItemPayload) -> Result<InventoryItem, Message> {
    if payload.name.is_empty() || payload.quantity == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'quantity' are provided.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let inventory_item = InventoryItem {
        id,
        restaurant_id: payload.restaurant_id,
        name: payload.name,
        quantity: payload.quantity,
        created_at: current_time(),
    };
    INVENTORY_STORAGE.with(|storage| storage.borrow_mut().insert(id, inventory_item.clone()));
    Ok(inventory_item)
}

#[ic_cdk::query]
fn get_inventory_items() -> Result<Vec<InventoryItem>, Message> {
    INVENTORY_STORAGE.with(|storage| {
        let inventory_items: Vec<InventoryItem> = storage
            .borrow()
            .iter()
            .map(|(_, item)| item.clone())
            .collect();

        if inventory_items.is_empty() {
            Err(Message::NotFound("No inventory items found".to_string()))
        } else {
            Ok(inventory_items)
        }
    })
}

#[ic_cdk::query]
fn get_inventory_item_by_id(id: u64) -> Result<InventoryItem, Message> {
    INVENTORY_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, item)| item.id == id)
            .map(|(_, item)| item.clone())
            .ok_or(Message::NotFound("Inventory item not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_expense(payload: ExpensePayload) -> Result<Expense, Message> {
    if payload.description.is_empty() || payload.amount <= 0.0 {
        return Err(Message::InvalidPayload(
            "Ensure 'description' and 'amount' are provided and valid.".to_string(),
        ));
    }

    let restaurant = RESTAURANT_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, restaurant)| restaurant.id == payload.restaurant_id)
            .map(|(_, restaurant)| restaurant.clone())
    });
    if restaurant.is_none() {
        return Err(Message::NotFound("Restaurant not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let expense = Expense {
        id,
        restaurant_id: payload.restaurant_id,
        description: payload.description,
        amount: payload.amount,
        created_at: current_time(),
    };
    EXPENSE_STORAGE.with(|storage| storage.borrow_mut().insert(id, expense.clone()));
    Ok(expense)
}

#[ic_cdk::query]
fn get_expenses() -> Result<Vec<Expense>, Message> {
    EXPENSE_STORAGE.with(|storage| {
        let expenses: Vec<Expense> = storage
            .borrow()
            .iter()
            .map(|(_, expense)| expense.clone())
            .collect();

        if expenses.is_empty() {
            Err(Message::NotFound("No expenses found".to_string()))
        } else {
            Ok(expenses)
        }
    })
}

#[ic_cdk::query]
fn get_expense_by_id(id: u64) -> Result<Expense, Message> {
    EXPENSE_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, expense)| expense.id == id)
            .map(|(_, expense)| expense.clone())
            .ok_or(Message::NotFound("Expense not found".to_string()))
    })
}

fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

ic_cdk::export_candid!();
