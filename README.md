### The Memento Design Pattern

The Memento design pattern in Rust is a behavioral design pattern used to
capture and restore an object's internal state without exposing its
implementation details. It is primarily leveraged to build undo/redo systems,
transactional rollbacks, or checkpoint mechanics.

The Memento pattern allows us to save and restore an object's state without exposing its internal
structure. This pattern provides a way to implement features like undo/redo, save/load
functionality, and state histories, all while maintaining encapsulation.

While the Command pattern from the previous chapter allows us to undo and redo individual
operations, we sometimes need more comprehensive state management. Users might want to save
the entire calculator state.

Because Rust enforces strict memory safety and ownership, implementing this
pattern differs slightly from traditional object-oriented languages. It relies
on value-based data structures rather than object reference pointers.

### When to use the Memento pattern

The Memento pattern fits scenarios where you need to save and restore object state without
violating encapsulation. It's appropriate for implementing undo/redo beyond simple operation
reversal, creating snapshots at significant points, supporting save/load functionality, and
implementing checkpoints for recovery.

The pattern complements the Command pattern. Commands track individual operations and their
inverses; mementos capture complete state. For our calculator, commands enable fine-grained
undo of specific operations, while mementos enable coarse-grained restoration to saved
checkpoints.

Consider memory usage when applying this pattern. Each memento clones the originator's state,
which can be expensive for large objects. Strategies to manage memory include limiting undo
history depth, storing partial states (only the fields that actually changed) rather than full
snapshots, using incremental deltas for frequent saves, and employing copy-on-write for large
immutable portions of state.

Rust's ownership model makes memento implementation straightforward. Mementos own their
data (via cloning), so there are no concerns about dangling references or shared mutation. The
privacy of the memento constructor, enforced by Rust's module system, ensures only the originator
creates valid mementos.

### Core Components

The pattern uses three main structural roles:

* Memento: A lightweight, immutable data structure that contains the saved state snapshot.
* Originator: The live application object that holds the current state. It creates snapshots of itself and consumes them to revert its state.
* Caretaker: The object responsible for tracking history (usually using a Vec stack). It stores and returns mementos but cannot view or modify their contents.

### Rust-Specific Design Trade-offs

| Feature | Advantage in Rust | Drawback |
|---|---|---|
| Encapsulation | Keeping fields private within a module ensures the Caretaker cannot view inside the Memento. | Over-allocating heap data via continuous .clone() operations can degrade performance. |
| Ownership | Moving a Memento into history.push() guarantees nobody else can mutate it while it sits in history. | If the Originator's internal structure changes, your Memento struct must be explicitly updated. |
