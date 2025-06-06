***Coming soon***
* Directly download namespace files from GitHub repositories
* Selecting multiple components
* Copying and pasting
* Font configuration

# v2.2.12
* Upgrade `bevy` to v0.16.1, among other dependencies

# v2.2.11
* Add support for jpg and webp tile image loading

# v2.2.10
* Fix crash on component delete

# v2.2.9
* Fix inaccurate status messages on idle
* Added version checker

# v2.2.8
* Remove confirmation message for deleting a namespace, since it can be undone
* Make `❌` buttons red
* Remove `click_max_offset` option since it is no longer necessary anywhere
* Added square next to component types, showing their colour on the map
* Fix panning moving backwards when zoomed in past the basemap's max zoom
* Position data is now shown as a table
* Fix cursor staying as a pointing finger when hovered out

# v2.2.7
* Support panning with mouse & zooming with pinch gesture on trackpad
* Add select button for components in components list
* UI fixes in component editor

# v2.2.6
* Upgrade `bevy` to v0.16
* Fix some English

# v2.2.5
* Upgrade `bevy` to v0.15
  * This involves major internal refactors
  * Slight movements between mouse down and up will no longer count as a click
* Upgrade `egui_dock` to v0.16, close buttons are now available in the top-right corner of windows
* If the cursor in creation modes is not hidden, a cell cursor is shown instead of a default pointer
* Moving components now snap them to the grid
* Make node-editing circle black instead of grey
* Decrease size and border of all node circles
* Fix `notif_duration` miscellaneous setting not being able to be updated
* Components and their node circles are now only rerendered when needed
* `toml` file filter when importing basemap
* File dialogue states and dock layout are now preserved across sessions
  * they are stored in the cache and data directories respectively
* Components turn olive when hovered over
* `bevy_inspector_egui` in debug builds

# v2.2.4
* The order _should_ be more closely followed when selecting components that are at a higher order/layer over others
* Image file extension field in basemaps
* Default maximum GET requests is now 65536
* Map should no longer move when moving tabs or layout
* Component list is now stable with more than 1 namespace
* Custom font loading in `fonts` folder in the data directory
  * A proper UI will be done in v2.3
* Move coordinates and pending number of tiles from the top menu to the toolbar

# v2.2.3
* Fix skin cache deletion every startup

# v2.2.2
* Update skin format for `tile-renderer` v5
* Sort components by ID before saving

# v2.2.1
* Upgrade `bevy` to v0.14
* Fix crash that happens sometimes (hopefully)

# v2.2.0
* Revamped UI
  * New tabs and windows / docking format from `egui_dock`
  * New notifications from `egui_notify`
  * Menu bar now has
    * status messages
    * zoom level
    * number of pending tiles
    * time per frame in milliseconds
* Added more settings
  * Revamp tilemap settings: multiple registered basemaps, one is selected for rendering
  * Keymap settings
  * Miscellaneous settings
* Added welcome message when opening the application for the first time
* Added project panel
  * Open, save and reload multiple namespace files in the same folder
  * Hide and show namespaces
  * Create and delete namespaces
  * Default project is a scratchpad
* Namespace in component editor is now a dropdown and only accepts visible namespaces
  * `_misc` is the default and cannot be hidden or deleted
* Added autosaving (default: every 60s)
* Added component list panel
* Added history panel
* Added notification log window
* Added green and red circles on start and end of line components respectively to show line direction
  * First and last coordinates in the component editor are also green and red respectively for line components
* If the skin type of a loaded component is invalid, now tries to guess whether it is a point, line or area depending on the nature of the nodes, instead of crashing
* Size of handles in node editing is now independent of the width of the component
* Added safe deleting, moving and writing of all files: now backs old versions up to a cache directory
* Added caching of skin JSON file
* Added Apple Silicon support in `dmg` file

# v2.1.3
* Upgrade `bevy` to v0.13
* Fix mouse icon not changing
* Fix application not existing in `dmg` file

# v2.1.2
* Upgrade `bevy` to v0.12 (painfully)
* No longer able to move the component in node-editing mode, go to select mode to do that
* No longer able to select a line from its centre like an area
* Update default tile URL for Minecart Rapid Transit

# v2.1.1
* Added back limit to max number of HTTP requests
* Added an option to clear tile cache on startup
* All fields in `tile_settings.toml` are now optional too

# v2.1.0
* Added window settings, GPU backend, Linux display server protocol configuration
* Stencil2 now starts in windowed mode instead of fullscreen
* Settings files are saved as toml instead of msgpack
* Moved manual online
* Now recognises `RUST_LOG` environment variable
* Now shows more licences from dependencies
* Tile cache is moved from data folder to cache folder

# v2.0.5
* URLs in tile cache are now escaped to fit Windows file-naming restrictions (especially `:`)

# v2.0.4
* Fix textboxes losing focus (again)
* ~~Changed backend of `surf` (http request lib) from `curl` to `async-h1` (it may or may not help render tiles in Windows?)~~
  * note from v2.0.5: this change was apparently not done; the actual change screwed up the tilemap
* Spantrace in panic files

# v2.0.3
* Update bevy
* Fix tilemap also moving when the window is moving
* Fix tiles not loading fast enough... hopefully
* Option to show checkerboard instead of downloading tiles through environment variable

# v2.0.2
* Fix skin not loading properly due to change in format of `font` field

# v2.0.1
* Fix file where tile settings are stored being created as a folder instead of a file

# v2.0.0
* Added hotkeys
* Fix errors loading tilemap
* Added manual

# v2.0.0-beta.4
* Added license page
* Fixed text boxes not being editable and immediately losing focus
* Undo / redo functionality
  * native-dialog is no longer a dependency
* Asset folder is now bundled together with the executable and unloaded before bevy starts
* Tile settings, initial zoom, and tilemap are now editable
* Better panic handling
* Log output to file

# v2.0.0-beta.3
* For lines and areas, clicking on the last node placed will remove the node
* Move popups (except one temporary one) to a new internal popup system
* New internal file explorer for importing and exporting PLA 2 files
  * mostly because file importing broke on macOS Ventura

# v2.0.0-beta.2
* Stencil 2 is now distributed as .app in .dmg on macOS
* Fix crosshair not showing up on Mac builds (maybe other builds too)
* Shows a confirmation popup when deleting a component of more than 5 nodes

# v2.0.0-beta.1
* Added popup handling system
* Handles in node editing of components with 50+ nodes will only show if they are nearest
* Added changelog window
* Added angle-snapping for lines and areas
* Fix quitting the app crashing on Mac builds (maybe other builds too)
* Fix a majority of z-fighting in components
* Add mouse X and Z-coordinate display
* Increase transparency of areas

# v2.0.0-beta.0
* Added tile map
* Added component modes for creation, deletion, node editing
* Added top menu, toolbar, component panel
* Added component actions for selecting and moving
* Added cursor changing and crosshair
* Added importing and exporting
* Added quitting
* Added info screen
