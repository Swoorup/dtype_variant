use dtype_variant::*;
use dtype_variant_example_shared::variants::AttackVariant;

#[derive(Clone, Debug, DType)]
#[dtype(
    matcher = "player_input_enum",
    tokens = "dtype_variant_example_shared::variants"
)]
pub enum PlayerInput {
    Move(String),
    Attack(u32),
}

#[derive(Clone, Debug, DType)]
#[dtype(
    matcher = "ai_behavior_enum",
    tokens = "dtype_variant_example_shared::variants"
)]
pub enum AIBehavior {
    Attack(u32),
    Flee(bool),
}

fn main() {
    let player_attack = PlayerInput::from(50_u32); // Attack with power level 50
    let ai_attack = AIBehavior::from(30_u32); // Attack with power level 30

    // Process shared actions (e.g., Attack) between player and AI
    let combined_attack = combine_shared_actions::<AttackVariant, u32>(
        &player_attack,
        &ai_attack,
    );
    match combined_attack {
        Some(total_power) => println!("Combined attack power: {}", total_power),
        None => println!("Actions do not match."),
    }
}

// Function to combine shared actions if their types match
fn combine_shared_actions<Variant, Target>(
    action1: &PlayerInput,
    action2: &AIBehavior,
) -> Option<Target>
where
    PlayerInput: EnumVariantDowncast<Variant, Target = Target>,
    AIBehavior: EnumVariantDowncast<Variant, Target = Target>,
    Target: std::ops::Add<Output = Target> + Clone + 'static,
{
    let inner1 = action1.downcast_ref::<Variant>()?;
    let inner2 = action2.downcast_ref::<Variant>()?;
    Some(inner1.clone() + inner2.clone())
}
