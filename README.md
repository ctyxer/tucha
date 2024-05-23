# tucha
tucha - Telegram-based cloud storage for organizing uploaded files.


## How to install 
1. Clone this repository.
2. Get your Telegram API ID and HASH from https://my.telegram.org/apps and insert this values in `src/types/api_keys.rs` file in relevant fields.
3. If you are runnong on Linux, you need to get the [dependecies](https://github.com/emilk/egui?tab=readme-ov-file#demo).
4. Run the project `cargo run --release`.

## Features

**Files**

- Downloading.
- Deleting.
- Uploading.
- ~~Preview~~. (in development, but not tested. Of course, we are all waiting for this)

~~**Directories/Folders**~~ (in development)

**Clients**

- Client switch.
- Add new client.

**OS**

- Linux, MacOS (not tested), Windows.
- All files stored in Telegram database, and can be downloaded in every supported platform.
