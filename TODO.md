# TODO


## In Progress

- [ ] Visualise Flow and Steps

## ToDo
- [ ] Improve property handling in McGuffin

- [ ] Capture McGuffin errors and show them in the editor

- [ ] Improve project file management (load, save, ..., etc)

- [ ] Add builtin minimal shader examples

- [ ] Add versioning to UniformManager

- [ ] Handle resoure loading and saving via command queue



- [ ] Add line numbers to shader editor

- [ ] vec2 position property picker

- [ ] Consider naming projects

- [ ] Find better name for `Flow`

## Later
- [ ] Look into _off by one_ width for window size restore after fullscreen
- [ ] Block quit when dirty and auto save on quit is off
- [ ] Fix Quit in File dialog crash
- [ ] Rename shaders window
- [ ] Add code editor theme selection
- [ ] Replace lazy_static with lazy_cell once stable (e.g. for CommandQueue)
- [ ] Decide on good default window size
- [ ] Decide on default on windows
## Done


## 2024-05-23
- [x] Add versioning to project to avoid constant rebuilds
- [x] Extract opengl setup into `Flow`
	- [x] Implement FullscreenQuad Step
	- [x] Implement Program Step

## 2024-05-14
- [x] Implement clone for McGuffinContainer
- [x] Display current project folder in top/menu bar
- [x] Add frame time display

## 2024-05-13
- [x] Add specialised cosine palette property editor

### 2024-05-08
- [x] Fix McGuffin size after returning from fullscreen mode
- [x] Use ALT-Enter to toggle fullscreen
- [x] Add support for vec3[4] uniforms/properties

### 2024-05-07
- [x] Improve shader list in program window
- [x] Improve resource list in resource window
- [x] Allow removing of resources
	- [x] Remove shader from program
	- [x] Remove resource from project
- [x] Improve properties window

### 2024-05-06
- [x] Make resource/shader path relative to project
- [+] Disable reload & save if no file name is set
- [x] Prefill filename in file picker dialog (save as...)
- [x] Save (resources) on quit
- [x] Highlight save button if resource is modified

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
