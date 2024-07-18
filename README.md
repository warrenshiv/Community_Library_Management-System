# Restaurant Management System

This project is a decentralized platform built on the Internet Computer, aiming to facilitate the management of restaurant details, staff, menus, orders, reservations, inventory, and expenses. It leverages the power of blockchain to ensure transparency and reliability in the management processes.

## Key Features

### Restaurant Management
- **Create Restaurant:** Allows the creation of new restaurant profiles with validation for input fields.
- **Get Restaurants:** Retrieves all registered restaurant profiles.
- **Get Restaurant by ID:** Retrieves the profile of a specific restaurant by its unique ID.

### Staff Management
- **Create Staff:** Allows the creation of new staff profiles for a restaurant.
- **Get Staff:** Retrieves all registered staff profiles.
- **Get Staff by ID:** Retrieves the profile of a specific staff member by their unique ID.

### Menu Management
- **Create Menu Item:** Allows the creation of new menu items for a restaurant.
- **Get Menu Items:** Retrieves all registered menu items.
- **Get Menu Item by ID:** Retrieves the details of a specific menu item by its unique ID.

### Order Management
- **Create Order:** Allows the creation of new orders for a restaurant.
- **Get Orders:** Retrieves all registered orders.
- **Get Order by ID:** Retrieves the details of a specific order by its unique ID.

### Reservation Management
- **Create Reservation:** Allows the creation of new reservations for a restaurant.
- **Get Reservations:** Retrieves all registered reservations.
- **Get Reservation by ID:** Retrieves the details of a specific reservation by its unique ID.

### Inventory Management
- **Create Inventory Item:** Allows the creation of new inventory items for a restaurant.
- **Get Inventory Items:** Retrieves all registered inventory items.
- **Get Inventory Item by ID:** Retrieves the details of a specific inventory item by its unique ID.

### Expense Management
- **Create Expense:** Allows the creation of new expenses for a restaurant.
- **Get Expenses:** Retrieves all registered expenses.
- **Get Expense by ID:** Retrieves the details of a specific expense by its unique ID.

### Error Handling
- **Not Found:** Returns an error if a requested resource (restaurant, staff, menu item, order, reservation, inventory item, expense) is not found.
- **Invalid Input:** Handles errors related to invalid input fields.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```