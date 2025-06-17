use dtype_variant::DType;
use std::collections::HashMap;

// Comprehensive game event system showcasing all dtype_variant features
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

// Event processor demonstrating real-world usage patterns
struct EventProcessor {
    player_positions: HashMap<u32, (f32, f32)>,
    death_count: u32,
    chat_messages: Vec<String>,
    items_collected: Vec<u32>,
    connected_players: u32,
}

impl EventProcessor {
    fn new() -> Self {
        Self {
            player_positions: HashMap::new(),
            death_count: 0,
            chat_messages: Vec::new(),
            items_collected: Vec::new(),
            connected_players: 0,
        }
    }

    fn process_event(&mut self, event: &GameEvent) {
        // Demonstrate grouped matching by priority - use reference to avoid cloning
        let priority = match_by_priority!(event, {
            Critical: GameEvent<T, Variant>(_inner) => { "🚨 CRITICAL" },
            Normal: GameEvent<T, Variant>(_inner) => { "📝 Normal" },
            Info: GameEvent<T> => { "ℹ️  Info" },
        });

        // Demonstrate grouped matching by category - use reference to avoid cloning
        let category = match_by_category!(event, {
            Player: GameEvent<T, Variant>(_inner) => { "Player action" },
            System: GameEvent<T, Variant>(_inner) => { "System event" },
            Communication: GameEvent<T, Variant>(_inner) => { "Communication" },
        });

        println!(
            "{} {} - {}: {:?}",
            priority,
            category,
            self.get_event_type_name(event),
            event
        );

        // Type-safe downcasting with comprehensive handling
        if let Some(movement) = event.downcast_ref::<PlayerMoveVariant>() {
            self.player_positions
                .insert(*movement.player_id, (*movement.x, *movement.y));
            println!(
                "  → Player {} moved to ({:.1}, {:.1})",
                *movement.player_id, *movement.x, *movement.y
            );
        }

        if let Some(death) = event.downcast_ref::<PlayerDeathVariant>() {
            self.death_count += 1;
            self.player_positions.remove(death.player_id);
            println!(
                "  → Player {} died: {} (Total deaths: {})",
                *death.player_id, death.cause, self.death_count
            );
        }

        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            self.chat_messages.push(chat.clone());
            println!("  → Chat message: '{}'", chat);
        }

        if let Some(item_id) = event.downcast_ref::<ItemPickupVariant>() {
            self.items_collected.push(*item_id);
            println!(
                "  → Item {} collected (Total items: {})",
                item_id,
                self.items_collected.len()
            );
        }

        // Handle connection events
        match event {
            GameEvent::PlayerConnect => {
                self.connected_players += 1;
                println!(
                    "  → Player connected (Online: {})",
                    self.connected_players
                );
            }
            GameEvent::PlayerDisconnect => {
                self.connected_players =
                    self.connected_players.saturating_sub(1);
                println!(
                    "  → Player disconnected (Online: {})",
                    self.connected_players
                );
            }
            GameEvent::ServerCrash => {
                println!(
                    "  → 💥 SERVER CRASH DETECTED! Initiating emergency procedures..."
                );
                self.emergency_shutdown();
            }
            _ => {}
        }
    }

    // Demonstrate generic pattern matching
    fn get_event_type_name(&self, event: &GameEvent) -> String {
        match_game_event!(event, GameEvent<Token> => {
            std::any::type_name::<Token>()
                .split("::")
                .last()
                .unwrap_or("Unknown")
                .replace("Variant", "")
        })
    }

    // Demonstrate mutable operations
    fn sanitize_chat_messages(&self, events: &mut Vec<GameEvent>) {
        for event in events {
            if let Some(chat_msg) = event.downcast_mut::<ChatMessageVariant>() {
                *chat_msg = chat_msg
                    .replace("badword", "***")
                    .replace("spam", "[filtered]")
                    .replace("hack", "[blocked]");
            }
        }
    }

    // Emergency procedures for critical events
    fn emergency_shutdown(&mut self) {
        println!("    🔧 Saving player positions...");
        println!(
            "    💾 Backing up chat logs ({} messages)...",
            self.chat_messages.len()
        );
        println!(
            "    🔒 Securing {} connected players...",
            self.connected_players
        );
        println!("    ⚡ Emergency shutdown complete!");
    }

    // Statistics and reporting
    fn print_summary(&self) {
        println!("\n=== Game Session Summary ===");
        println!("👥 Active players: {}", self.connected_players);
        println!("💀 Deaths: {}", self.death_count);
        println!("💬 Chat messages: {}", self.chat_messages.len());
        println!("🎁 Items collected: {}", self.items_collected.len());
        println!("🗺️  Player positions: {:?}", self.player_positions);

        if !self.chat_messages.is_empty() {
            println!(
                "💭 Recent chats: {:?}",
                &self.chat_messages[..self.chat_messages.len().min(3)]
            );
        }

        if !self.items_collected.is_empty() {
            println!(
                "🎯 Items found: {:?}",
                &self.items_collected[..self.items_collected.len().min(5)]
            );
        }
    }
}

fn main() {
    println!("🎮 === Advanced Game Event Processing Demo ===\n");

    let mut processor = EventProcessor::new();

    // Create a realistic sequence of game events
    let mut events = vec![
        GameEvent::PlayerConnect,
        GameEvent::PlayerConnect,
        GameEvent::PlayerMove {
            player_id: 1,
            x: 10.5,
            y: 20.3,
        },
        GameEvent::PlayerMove {
            player_id: 2,
            x: 5.0,
            y: 15.0,
        },
        GameEvent::ChatMessage("Hello everyone!".to_string()),
        GameEvent::ItemPickup(42),
        GameEvent::ChatMessage("This contains a badword!".to_string()),
        GameEvent::PlayerMove {
            player_id: 1,
            x: 12.0,
            y: 18.5,
        },
        GameEvent::ItemPickup(87),
        GameEvent::PlayerDeath {
            player_id: 2,
            cause: "fell into lava".to_string(),
        },
        GameEvent::ChatMessage("RIP player 2".to_string()),
        GameEvent::PlayerDisconnect,
        GameEvent::ItemPickup(156),
        GameEvent::ServerCrash,
    ];

    // Process all events
    println!("📊 Processing {} events...\n", events.len());
    for (i, event) in events.iter().enumerate() {
        println!("Event {}: ", i + 1);
        processor.process_event(event);
        println!();
    }

    // Demonstrate chat sanitization
    println!("🧹 === Chat Sanitization Demo ===");
    println!("Before sanitization:");
    for event in &events {
        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            println!("  '{}'", chat);
        }
    }

    processor.sanitize_chat_messages(&mut events);

    println!("\nAfter sanitization:");
    for event in &events {
        if let Some(chat) = event.downcast_ref::<ChatMessageVariant>() {
            println!("  '{}'", chat);
        }
    }

    // Demonstrate advanced pattern matching
    println!("\n🔍 === Event Analysis Demo ===");
    let test_events = [
        GameEvent::PlayerMove {
            player_id: 99,
            x: 100.0,
            y: 200.0,
        },
        GameEvent::ChatMessage("Advanced test".to_string()),
        GameEvent::ServerCrash,
        GameEvent::ItemPickup(999),
    ];

    for event in &test_events {
        let event_type = processor.get_event_type_name(event);

        let priority = match_by_priority!(event, {
            Critical: GameEvent<T, Variant>(_inner) => { "Critical" },
            Normal: GameEvent<T, Variant>(_inner) => { "Normal" },
            Info: GameEvent<Variant> => { "Info" },
        });

        let category = match_by_category!(event, {
            Player: GameEvent<T, Variant>(_inner) => { "Player" },
            System: GameEvent<T, Variant>(_inner) => { "System" },
            Communication: GameEvent<T, Variant>(_inner) => { "Communication" },
        });

        println!(
            "{:<15} | {:<8} | {:<13} | {:?}",
            event_type, priority, category, event
        );
    }

    // Show final statistics
    processor.print_summary();

    println!("\n✅ All advanced features demonstrated successfully!");
    println!("   • Local token generation (no external tokens needed)");
    println!("   • Struct variants with named field access");
    println!("   • Multiple grouped matchers (priority & category)");
    println!("   • Type-safe downcasting (owned, ref, mut)");
    println!("   • Generic pattern matching");
    println!("   • Mutable operations and data modification");
    println!("   • Real-world event processing pipeline");
}
