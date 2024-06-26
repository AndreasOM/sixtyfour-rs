# TODO


## In Progress


## ToDo

- [ ] Add different modes for property editors

- [ ] Allow multiselect move, copy, delete, clone, etc

- [ ] Refactor: Extract UiGrid storage into it's own file
- [ ] Refactor: Clean up selection shrinking for UI grid


- [ ] Make ShaderWindow autoselect a shader if needed/possible

- [ ] Add github action to run `cargo check` after push


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
- [ ] Rethink time handling, and resolution when implementing the timeline
- [ ] Look into _off by one_ width for window size restore after fullscreen
- [ ] Block quit when dirty and auto save on quit is off
- [ ] Fix Quit in File dialog crash
- [ ] Rename shaders window
- [ ] Add code editor theme selection
- [ ] Replace lazy_static with lazy_cell once stable (e.g. for CommandQueue)
- [ ] Decide on good default window size
- [ ] Decide on default on windows

## Done

## 2024-06-24
- [x] Allow single cell selection via click
- [x] Fix offset when starting drag on other than top left cell
- [x] Add step to set uniform vec3 f32

## 2024-06-17
- [x] Fix scaling of move/copy mode info box
- [x] Improve grid move & copy preview
- [x] Deselect if new selection is empty

## 2024-06-15
- [x] Remember non fullscreen McGuffin window size across restarts
- [x] Allow move, and clone via mouse
- [x] Cancel grid move & copy with ESCape key
- [x] Add grid move & copy preview

## 2024-06-10
- [x] Fixup "move step" for rect selection
- [x] Fixup "remove step(s)" for rect selection
- [x] Fixup "clone step(s)" for rect selection
- [x] Fixup "add step" for rect selection

## 2024-06-09
- [x] Some progress on grid selection via dragging
- [x] Allow multiselect in flow window
	- [x] Drag selection box in grid with primary mouse button, but allow secondary to drag grid and/or window
	- [x] Dim old selection frame while selecting
	- [x] Support grid scrolling (e.g. via mouse wheel) while selecting
	- [x] Shrink selection to minimum

## 2024-06-07
- [x] Replace fTime with dTime for better precision, and reset after X hours
	Notes:
		- You still have to handle resets of absolute time when using fTime or dTime
		- Don't use dTime at all ;) -> due to double performance in shaders

## 2024-06-05
- [x] Multiselect: Highlight target according to source size
- [x] Make flow grid zoomable

## 2024-05-28
- [x] Set uniform from flow
- [x] Allow "expressions" in uniform values (`${TIME}`)
- [x] Allow steps to be placed on grid
	- [x] Store the grid position for step
	- [x] Use grid position to edit steps
	- [x] Allow cell selection via mouse
	- [x] Move steps in grid/flow
- [x] Remove blocks from flow/project
- [x] Add duplicate step in flow

## 2024-05-27
- [x] Add and remove steps from flow via editor 
- [x] Edit uniform step in flow editor

## 2024-05-26
- [x] Add grid lines to grid view
- [x] Add highlight around selected step

## 2024-05-25
- [x] Add grid for flow
	- [x] Add text to steps/cells
	- [x] Allow selection of step

## 2024-05-24
- [x] Use menu bar for menu bar
- [x] Add names to Program Resources
	- [x] Edit names in Resource Window
	- [x] Show name in "Shader" Window
- [x] Visualise Flow and Steps
	- [x] Add versioning to blocks and steps
	- [x] Hack in basic flow editor for Program Step
	- [x] Fix height of Flow window header
	- [x] Select Program Step resource via drop down
- [x] Show Program name in Flow Window
- [x] Improve Shader Window _top_
	- [x] Make top a panel
	- [x] Show shader/text name
	- [x] Add combo box to select current program
	- [x] Filter "add shader" list for already used shaders
- [x] Fix compile log in Shader Window

## 2024-05-23
- [x] Add versioning to project to avoid constant rebuilds
- [x] Extract opengl setup into `Flow`
	- [x] Implement FullscreenQuad Step
	- [x] Implement Program Step
- [x] Add flow window

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
