# dtype_variant

A Rust derive macro for creating type-safe enum variants with powerful downcasting capabilities and flexible token management. Build robust data processing pipelines with compile-time type safety and zero runtime overhead.

## ✨ Features

- 🏠 **Local or Shared Tokens**: Generate variant tokens locally or share them across multiple enums
- 🔄 **Three-way Downcasting**: Owned, reference, and mutable reference downcasting with lifetime safety
- 🏗️ **Struct Variants**: Full support for named field variants with generated wrapper types
- 📦 **Container Support**: Optional container type support (e.g., Vec, Box) for variant data
- 🔒 **Compile-time Safety**: Type-safe operations with compile-time validation
- 🎯 **Pattern Matching**: Powerful generated macros for ergonomic variant handling
- 🔗 **Grouped Matching**: Match related variants together by logical categories
- 🛠️ **Constraint Traits**: Enforce trait bounds on variant types

## 🚀 Quick Start

The simplest way to get started - no external token definitions needed:

```rust
use dtype_variant::DType;

#[derive(DType, Debug)]
#[dtype(matcher = match_data)]
enum ProcessingData {
    Numbers(Vec<i32>),
    Text(String),
    Metadata { version: u32, tags: Vec<String> },
}

fn main() {
    let data = ProcessingData::Numbers(vec![1, 2, 3, 4, 5]);
    
    // Type-safe downcasting - no boilerplate needed!
    if let Some(numbers) = data.downcast_ref::<NumbersVariant>() {
        println!("Sum: {}", numbers.iter().sum::<i32>());
    }
    
    // Generated pattern matching
    match_data!(data, ProcessingData<Token> => {
        println!("Processing data variant");
    });
}
```

## 🎯 Real-World Example: Game Event System

Here's a compelling example showing how `dtype_variant` solves real problems in game development:

```rust
use dtype_variant::DType;
use std::collections::HashMap;

// No external tokens needed - they're generated automatically!
#[derive(DType, Debug, Clone)]
#[dtype(matcher = match_game_event)]
#[dtype_grouped_matcher(name = match_by_priority, grouping = [
    Critical(PlayerDeath | ServerCrash),
    Normal(PlayerMove | ChatMessage | ItemPickup),
    Info(PlayerConnect | PlayerDisconnect)
])]
#[dtype_grouped_matcher(name = match_by_category, grouping = [
    Player(PlayerMove | PlayerConnect | PlayerDisconnect | PlayerDeath),
    System(ServerCrash | ItemPickup),
    Communication(ChatMessage)
])]
enum GameEvent {
    // Struct variants with named fields
    PlayerMove { player_id: u32, x: f32, y: f32 },
    PlayerDeath { player_id: u32, cause: String },
    
    // Tuple variants
    ChatMessage(String),
    ItemPickup(u32), // item_id
    
    // Unit variants
    PlayerConnect,
    PlayerDisconnect,
    ServerCrash,
}

// Event processor with type-safe handling
struct EventProcessor {
    player_positions: HashMap<u32, (f32, f32)>,
    death_count: u32,
}

impl EventProcessor {
    fn process_event(&mut self, event: &GameEvent) {
        // Demonstrate multiple grouped matchers
        let priority = match_by_priority!(event.clone(), {
            Critical: GameEvent<T, Variant>(inner) => { "🚨 CRITICAL" },
            Normal: GameEvent<T, Variant>(inner) => { "📝 Normal" },
            Info: GameEvent<T, Variant>(inner) => { "ℹ️  Info" },
        });
        
        let category = match_by_category!(event.clone(), {
            Player: GameEvent<T, Variant>(inner) => { "Player action" },
            System: GameEvent<T, Variant>(inner) => { "System event" },
            Communication: GameEvent<T, Variant>(inner) => { "Communication" },
        });
        
        println!("{} {} - {:?}", priority, category, event);
        
        // Type-safe downcasting for specific event types
        if let Some(movement) = event.downcast_ref::<PlayerMoveVariant>() {
            self.player_positions.insert(
                *movement.player_id, 
                (*movement.x, *movement.y)
            );
            println!("  → Player {} moved to ({:.1}, {:.1})", 
                     *movement.player_id, *movement.x, *movement.y);
        }
        
        if let Some(death) = event.downcast_ref::<PlayerDeathVariant>() {
            self.death_count += 1;
            self.player_positions.remove(death.player_id);
            println!("  → Player {} died: {}", *death.player_id, death.cause);
        }
        
        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            println!("  → Chat: '{}'", chat);
        }
        
        // Handle critical events with emergency procedures
        match event {
            GameEvent::ServerCrash => {
                println!("  → 💥 SERVER CRASH! Initiating emergency shutdown...");
            },
            _ => {}
        }
    }
    
    // Generic event analyzer using pattern matching
    fn analyze_event(&self, event: &GameEvent) -> String {
        match_game_event!(event, GameEvent<Token> => {
            format!("Event type: {:?}", std::any::type_name::<Token>())
        })
    }
    
    // Demonstrate mutable operations
    fn sanitize_chat_messages(&self, events: &mut Vec<GameEvent>) {
        for event in events {
            if let Some(chat_msg) = event.downcast_mut::<ChatMessageVariant>() {
                *chat_msg = chat_msg
                    .replace("badword", "***")
                    .replace("spam", "[filtered]");
            }
        }
    }
}

fn main() {
    let mut processor = EventProcessor::new();
    
    let mut events = vec![
        GameEvent::PlayerConnect,
        GameEvent::PlayerMove { player_id: 1, x: 10.5, y: 20.3 },
        GameEvent::ChatMessage("Hello everyone!".to_string()),
        GameEvent::ItemPickup(42),
        GameEvent::ChatMessage("This contains a badword!".to_string()),
        GameEvent::PlayerDeath { player_id: 2, cause: "fell into lava".to_string() },
        GameEvent::ServerCrash,
    ];
    
    // Process events with comprehensive logging
    for event in &events {
        processor.process_event(event);
    }
    
    // Demonstrate chat sanitization
    println!("Before sanitization:");
    for event in &events {
        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            println!("  '{}'", chat);
        }
    }
    
    processor.sanitize_chat_messages(&mut events);
    
    println!("After sanitization:");
    for event in &events {
        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            println!("  '{}'", chat);
        }
    }
    
    // Advanced analysis with multiple grouped matchers
    for event in &events {
        let priority = match_by_priority!(event.clone(), {
            Critical: GameEvent<T, Variant>(inner) => { "Critical" },
            Normal: GameEvent<T, Variant>(inner) => { "Normal" },
            Info: GameEvent<T, Variant>(inner) => { "Info" },
        });
        
        let category = match_by_category!(event.clone(), {
            Player: GameEvent<T, Variant>(inner) => { "Player" },
            System: GameEvent<T, Variant>(inner) => { "System" },
            Communication: GameEvent<T, Variant>(inner) => { "Communication" },
        });
        
        println!("{} | {} | {:?}", priority, category, event);
    }
}
```

This example demonstrates:

- **Zero Boilerplate**: No manual token definitions needed - tokens generated automatically
- **Multiple Grouped Matchers**: Both priority-based and category-based event classification
- **Struct Variants**: Rich event data with named fields and proper field access
- **Type-Safe Downcasting**: Reference, mutable, and owned downcasting with lifetime safety
- **Comprehensive Event Processing**: Real-world game server patterns and state management
- **Mutable Operations**: Safe batch mutation through mutable references
- **Advanced Pattern Matching**: Generic matching that works across all variant types
- **Emergency Handling**: Critical event processing with automated procedures

## 📖 Core Concepts

### Local vs Shared Tokens

**Local Tokens (Recommended for Most Cases)**:
```rust
#[derive(DType)]
#[dtype(matcher = match_data)]  // No shared_variant_zst_path needed!
enum MyData {
    Text(String),
    Numbers(Vec<i32>),
}
// Tokens `TextVariant` and `NumbersVariant` are generated automatically
```

**Shared Tokens (For Multi-Enum Synchronization)**:
```rust
use dtype_variant::build_dtype_tokens;

// Define shared tokens once
build_dtype_tokens!([Text, Numbers]);

#[derive(DType)]
#[dtype(shared_variant_zst_path = self, matcher = match_data)]
enum MyData {
    Text(String),
    Numbers(Vec<i32>),
}

#[derive(DType)]
#[dtype(shared_variant_zst_path = self, matcher = match_processed)]
enum ProcessedData {
    Text(String),      // Same TextVariant token
    Numbers(Vec<f64>), // Same NumbersVariant token, different data type
}
```

### Three-Way Downcasting

`dtype_variant` provides three separate downcasting methods with proper lifetime safety:

```rust
#[derive(DType)]
#[dtype(matcher = match_data)]
enum MyData {
    Text(String),
    Config { host: String, port: u16 },
}

let data = MyData::Text("hello".to_string());

// 1. Reference downcasting (most common)
let text_ref: Option<&String> = data.downcast_ref::<TextVariant>();

// 2. Mutable reference downcasting
let mut data = MyData::Text("hello".to_string());
let text_mut: Option<&mut String> = data.downcast_mut::<TextVariant>();

// 3. Owned downcasting (consumes the enum)
let text_owned: Option<String> = data.downcast::<TextVariant>();

// Struct variants return wrapper types with named field access
let config = MyData::Config { host: "localhost".to_string(), port: 8080 };
if let Some(config_ref) = config.downcast_ref::<ConfigVariant>() {
    println!("Host: {}, Port: {}", config_ref.host, *config_ref.port);
}
```

### Struct Variants Support

Full support for struct variants with generated wrapper types:

```rust
#[derive(DType)]
#[dtype(matcher = match_user)]
enum UserEvent {
    Login { username: String, timestamp: u64 },
    Logout { username: String },
    UpdateProfile { username: String, email: String, age: u32 },
}

let event = UserEvent::Login { 
    username: "alice".to_string(), 
    timestamp: 1234567890 
};

// Returns UserEventLoginRef<'_> with field access
if let Some(login) = event.downcast_ref::<LoginVariant>() {
    println!("User {} logged in at {}", login.username, *login.timestamp);
}

// Owned downcasting returns UserEventLoginFields with owned data
if let Some(login_data) = event.downcast::<LoginVariant>() {
    println!("Owned data: {} at {}", login_data.username, login_data.timestamp);
}
```

## 🔧 Configuration Options

```rust
#[derive(DType)]
#[dtype(
    shared_variant_zst_path = path::to::tokens,  // Optional: Path to shared tokens
    matcher = match_my_enum,                     // Optional: Generated matcher macro name
    container = Vec,                             // Optional: Container type for variants
    constraint = Display,                        // Optional: Trait constraint
    skip_from_impls = false                      // Optional: Skip From implementations
)]
enum MyEnum {
    // variants...
}
```

### Grouped Variant Matching

Create logical groupings of variants for powerful pattern matching:

```rust
#[derive(DType)]
#[dtype(matcher = match_data)]
#[dtype_grouped_matcher(name = match_by_type, grouping = [
    Numeric(Integer | Float),
    Textual(Text | Json),
    Binary(Bytes | Image)
])]
enum ProcessingData {
    Integer(i64),
    Float(f64),
    Text(String),
    Json(serde_json::Value),
    Bytes(Vec<u8>),
    Image { width: u32, height: u32, data: Vec<u8> },
}

let data = ProcessingData::Float(3.14159);

let category = match_by_type!(data, {
    Numeric: ProcessingData<T, Variant>(value) => {
        format!("Processing numeric data of type {}", std::any::type_name::<T>())
    },
    Textual: ProcessingData<T, Variant>(value) => {
        format!("Processing text data")
    },
    Binary: ProcessingData<T, Variant>(_) => {
        format!("Processing binary data")
    }
});

// Note: For reference-only access, use `&data` instead of consuming:
// match_by_type!(&data, { ... })
```

## 🛠️ Advanced Features

### Container Types

Wrap variant data in container types:

```rust
#[derive(DType)]
#[dtype(container = Vec, constraint = Clone)]
enum BatchData {
    Numbers(Vec<i32>),    // Inner type: i32, Full type: Vec<i32>
    Text(Vec<String>),    // Inner type: String, Full type: Vec<String>
}
```

### Trait Constraints

Ensure all variant types implement specific traits:

```rust
trait Processable {
    fn process(&self) -> String;
}

impl Processable for i32 {
    fn process(&self) -> String { self.to_string() }
}

impl Processable for String {
    fn process(&self) -> String { self.clone() }
}

#[derive(DType)]
#[dtype(constraint = Processable, matcher = match_processable)]
enum ProcessableData {
    Number(i32),
    Text(String),
}

fn process_any(data: &ProcessableData) -> String {
    match_processable!(data, ProcessableData<T, Token>(value) => {
        value.process()  // Guaranteed to work due to constraint
    })
}
```

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
dtype_variant = "0.0.12"
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

MIT

## 🙏 Acknowledgements

This project was inspired by [dtype_dispatch](https://github.com/pcodec/pcodec/tree/main/dtype_dispatch), which provides similar enum variant type dispatch functionality.