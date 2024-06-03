This is a place for me to write docs that'll eventually need a home. These are mostly incomplete
thoughts and might not be entirely correct as I'm still learning Rust and data-driven design. I'll
make sure to get these all cleaned up before any sort of alpha release lol

## Why an ECS Architecture?

TODO: Write docs here about how inheritance doesn't work very well with Rust and how an ECS is not
only the most optimized game engine architecture (source needed lmao), but also is really the only
practical architecture to use in a rust-based game engine

## Manatee Game Lifecycle and Architecture

Everything in Manatee is stored in scenes. Scenes require 4 major parts:

1. A GameMode
2. Entities
3. Components
4. Systems

GameModes control game logic, everything else is a part of the ECS. Here's a very quick rundown on
these ECS parts and how they (in theory at this point) work. In order for this to make logical
sense for those of you that have never used an ECS architecture before, I'll be starting with
components

### Components

A component is basically just a struct (or object if you're not coming from a Rust background). It
has no functionality whatsoever, it literally just holds data. For example, a component to set a
character's health might look something like this

```rust
struct HealthComponent {
  current_health: i32,
}
```

These data objects are great, but you might be wondering how they're actually used inside of a
game. That's where our other two parts, Entities and Systems, come into place. Let's talk about
entities.

### Entities

An entity is essentially a group of one or more components with a unique ID to help the game engine
find those component instances in memory. For example, a game will likely have an entity for the
player character, and we spawn our player entity with our above `HealthComponent` example to give
it a unique `current_health` value.

### Systems

I don't fucking understand how systems work, please help