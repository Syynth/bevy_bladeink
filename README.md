# `bevy_bladeink`

Bevy plugin for [Ink](https://www.inklestudios.com/ink/) narrative scripting via [bladeink](https://github.com/bladecoder/blade-ink-rs).

The goal of this plugin was to provide a simple way to integrate an Ink runtime with Bevy. Before writing my own, I looked around and found bladeink, which while it has some notable limitations (namely due to being faithful to the original implementation), was already written.

## Primary limitations

**Only One (1) Story** can be loaded at a time. The bladeink runtime uses `Rc<RefCell>` extensively, so the actual `Story` is stored in a `NonSend` resource.

**All communication with the story is asynchronous.** Actual work is done through either `Command` or `Event` types. You can bind external functions to the runtime, but it will be particularly difficult to do so in a way that allows those external functions to access the rest of the Bevy world. If you really need to, you can query for the story yourself with `NonSendMut<Story>`, but I've designed the API hoping that one day I can implement a thread-safe runtime (not likely lol), so it's preferable to use the provided observers.

## Quickstart Guide

#### Install:
First, add the `bevy_bladeink` dependency to your application:

```sh
cargo add bevy_bladeink
```

1. Add `InkPlugin`
```rust
app.add_plugins(InkPlugin)
```

2. Insert `InkStory` resource
```rust
app.insert_resource(InkStory::new("story.ink.json"));
```

3. Observe `StoryReady` event
```rust
app.add_observer(on_story_ready);

fn on_story_ready(_: On<StoryReady>, mut commands: Commands) {
    // story is ready! go wild!
}
```

4. Begin a sequence
```rust
commands.ink_begin_sequence("npc_1_dialogue");
```

## Sample

Here's possibly the smallest possible integration, that will just play through a story and select the first choice every time.

```rust
use bevy::prelude::*;
use bevy_bladeink::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InkPlugin)
        .insert_resource(InkStory::new("story.ink.json"))
        .add_observer(on_story_ready)
        .add_observer(on_deliver_line)
        .add_observer(on_deliver_choices)
        .run();
}

// Start your story when ready
fn on_story_ready(_: On<StoryReady>, mut commands: Commands) {
    commands.ink_begin_sequence("start");
}

// Handle dialogue lines
fn on_deliver_line(line: On<DeliverLine>, mut commands: Commands) {
    println!("{}", line.text);
    commands.ink_continue_sequence();
}

// Handle player choices
fn on_deliver_choices(choices: On<DeliverChoices>, mut commands: Commands) {
    for (i, choice) in choices.choices.iter().enumerate() {
        println!("{}: {}", i + 1, choice.text());
    }
    commands.ink_select_choice(0); // Select first choice
}
```

## Missing features
This crate is still a work in progress. I'm focused on the use-cases that are blocking my usage in my own game, but suggestions, feature ideas, and pull requests are welcome :)

#### Coming Soon:
- [x] Observing variables
- [ ] Proper load/save/restore functionality
- [ ] Full support for hot-reloading (current implementation untested, should sort of work)
- [ ] An unstyled, pre-built dialogue UI that does most of the heavy lifting for you.
- [ ] A selection of pre-built external function bindings that are useful for common tasks.
- [ ] More examples and documentation

#### Support TBD:
- [ ] Automatic compilation of Ink files
- [ ] Support for threading
- [ ] Support for working with `LIST` types on the Bevy side.

## Asset Credits
- [The Intercept](https://github.com/inkle/the-intercept) - Available under the MIT License.
- [Fira Sans](https://fonts.google.com/specimen/Fira+Sans) - Available under the SIL Open Font License.
