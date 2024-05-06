# TODO


## In Progress



## ToDo

- [ ] Improve projecty file management (load, save, ..., etc)

- [ ] Highlight save button if resource is modified

- [ ] Add line numbers to shader editor

- [ ] vec2 position property picker
- [ ] Capture McGuffin errors and show them in the editor
- [ ] Add builtin minimal shader examples
- [ ] Add frame time display


- [ ] Implement clone for McGuffinContainer
- [ ] Add versioning to UniformManager

- [ ] Fix McGuffin size after returning from fullscreen mode

- [ ] Handle resoure loading and saving via command queue

## Later
- [ ] Fix Quit in File dialog crash
- [ ] Rename shaders window
- [ ] Add code editor theme selection
- [ ] Replace lazy_static with lazy_cell once stable (e.g. for CommandQueue)

## Done

### 2024-05-06
- [x] Make resource/shader path relative to project
- [+] Disable reload & save if no file name is set
- [x] Prefill filename in file picker dialog (save as...)
- [x] Save (resources) on quit

### 2024-05-03
- [x] Disable McGuffin window dragging
- [x] Connect property to hover position
- [x] Store window state

- [x] Add menu and allow window visibility management
- [x] Add close button to windows
- [x] Add fullscreen mode

### 2024-05-02
- [x] Show filename as tooltip on reload button
- [x] Load shaders/resources on startup
- [x] Clean up McGuffin
	- [x] Remove shader loading & saving
	- [x] Remove all traces serialisation
- [x] Get shader log back into editor
- [x] Get uniforms/properties back to working
- [x] Write a nice shader ;) -> Mandelbrot

- [x] Order properties alphabetically
- [x] Fix McGuffinContainer .clone()
- [x] Allow deleting of individual properties
- [x] Add vec2 property
- [x] Connect property to click position (fMouseClick)

### before 2024-05-02

Note: Bad tracking before this date.

- [x] Render untextured white triangle

## Released
