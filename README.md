# CmdGoogle

A blazing fast and convenient way to perform Google searches directly from your terminal!

![image](https://github.com/user-attachments/assets/d1103d81-d588-49b1-81f0-1dded1389317)

![image](https://github.com/user-attachments/assets/74a642f2-aefc-42f2-a8a9-fc1718d0189e)

![image](https://github.com/user-attachments/assets/843cf23a-e8a0-4fe6-a52e-57280caac2b1)


## Features

* **Search directly from your terminal:**  No need to open a web browser for quick searches.
* **Interactive Search Box:**
    * Real-time input as you type.
    * Cursor movement (left/right).
    * Character insertion and deletion.
* **Display Search Results:**
    * Clear and concise presentation of search result URLs and descriptions.
    * Indication of crawl support for each URL (more on this below).
* **Navigate Results:**
    * Use `up` and `down` arrow keys (or `j` and `k`) to select search results.
* **Open URLs:**
    * Press `o` to open the selected URL in your default web browser.
* **Search History:**
    * Access and reuse previous search queries using the `up` and `down` arrow keys in editing mode.
* **Caching:**
    * Search results are cached for a period to speed up subsequent identical searches.
    * Optional notification when a cached result is used.
* **Clear Results:**
    * Press `c` to clear the current search results and the search input.
* **Debug Mode:**
    * Enable debug mode with `d` to save the raw HTML of the search results (useful for development).
* **Configuration:**
    * Toggle cache hit notifications on/off with `n`.
* **Keybindings:**
    * **Normal Mode:**
        * `q`: Quit the application.
        * `e`: Enter editing mode to type a new search query.
        * `c`: Clear the search results and input.
        * `o`: Open the selected URL in the browser.
        * `d`: Toggle debug mode (saves raw HTML).
        * `n`: Toggle cache hit notifications.
        * `up` / `k`: Select the previous search result.
        * `down` / `j`: Select the next search result.
    * **Editing Mode:**
        * `Enter`: Submit the search query.
        * `Esc`: Return to normal mode.
        * `up`: Navigate to the previous search in history.
        * `down`: Navigate to the next search in history.
        * `left`: Move the cursor left.
        * `right`: Move the cursor right.
        * `<type>`: Enter characters for your search query.
        * `Backspace`: Delete the character before the cursor.

## In Progress

* **Easy Crawl Support for In-Terminal Display:**  We are actively working on a feature that will allow you to easily "crawl" websites directly from the search results and display the content within the terminal. This will enable you to quickly view the content of web pages without leaving your terminal. The `Crawl supported` flag currently indicates which URLs *might* be suitable for this feature in the future.
* **Proxy Pool and Robust Bot Detection Avoidance:** To improve reliability and prevent being blocked by Google's anti-bot systems, we are implementing a proxy pool and more robust bot detection avoidance techniques. This will allow for more consistent and uninterrupted search functionality.
* **Option to Use the Official Google API:** We plan to add an option to utilize the official Google Search API. This will provide a more legitimate and structured way to access search results, although it may come with limitations (such as the API's free tier of approximately 100 requests per day). This option will likely require API key configuration.

## Installation

Make sure you have Rust and Cargo installed on your system.

```bash
cargo install --git https://github.com/vTuanpham/CmdGoogle
```

Alternatively, you can clone the repository and build it:

```bash
git clone https://github.com/vTuanpham/CmdGoogle
cd CmdGoogle
cargo build --release
```

You can then run the executable located at `target/release/CmdGoogle`. You might want to add this directory to your system's `PATH` environment variable for easier access.

## Usage

Run the application from your terminal:

```bash
CmdGoogle
```

You will be presented with a search box. Start typing your query and press `Enter` to search.

Use the keybindings mentioned in the "Features" section to navigate and interact with the search results.

## Configuration

Currently, the application offers a basic toggle for cache hit notifications using the `n` key in normal mode. Future versions might include more configuration options.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

