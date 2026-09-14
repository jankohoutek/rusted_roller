use rand::Rng;
use std::{io, env};

// --- Localization ---

// A structure to hold all user-facing strings, making the UI language-agnostic.
#[derive(Debug)]
struct Localization {
    title: &'static str,
    title_separator: &'static str,
    die_sides_prompt: &'static str,
    target_value_prompt: &'static str,
    num_rolls_prompt: &'static str,
    max_sides_label: &'static str,
    success_req_label: &'static str,
    default_max_sides: &'static str,
    simulation_start_msg: &'static str,
    success_high_label: &'static str,
    critical_success_label: &'static str,
    roll_log_msg: &'static str,
    failure_label: &'static str,
    summary_title: &'static str = "=================================================",
    pre_distraction_label: "Regular Successes (Pre-distraction)",
    bonus_rolls_label: "Bonus Rolls",
    total_bonus_label: "Total Bonus Rolls Performed",
    bonus_success_label: "Bonus Successes",
    bonus_failure_label: "Bonus Failures",
    final_result_label: "Final Adjusted Regular Successes",
    exit_msg: "👋 Thanks for using the Rust Roller CLI Tool!",
    continue_prompt: "Run another simulation? (true/false):",
    error_input_prompt: "❌ Invalid input. Please enter a valid value.",
}

// Default English Localization structure
const LANGUAGE_EN_US: Localization = Localization {
    title: "🎲 Rust Roller CLI Tool 🌍",
    title_separator: "=================================================",
    die_sides_prompt: "Enter the maximum number of sides for the die (e.g., 20):",
    target_value_prompt: "Enter the required success value (Target value):",
    num_rolls_prompt: "Enter the number of total rolls to simulate:",
    max_sides_label: "{} sides",
    success_req_label: "Success requires a roll of {} or higher. (Crit Success: {})",
    default_max_sides: "10",
    simulation_start_msg: "Simulating {} rolls of a {}-sided die...",
    success_high_label: "Success",
    critical_success_label: "CRITICAL SUCCESS",
    roll_log_msg: "Roll {} ({})",
    failure_label: "failure",
    summary_title: &'static str = "=================================================",
    pre_distraction_label: "Regular Successes (Pre-distraction)",
    bonus_rolls_label: "Bonus Rolls",
    total_bonus_label: "Total Bonus Rolls Performed",
    bonus_success_label: "Bonus Successes",
    bonus_failure_label: "Bonus Failures",
    final_result_label: "Final Adjusted Regular Successes",
    exit_msg: "👋 Thanks for using the Rust Roller CLI Tool!🌍",
    continue_prompt: "Run another simulation? (true/false):",
    error_input_prompt: "❌ Invalid input. Please enter a valid value.",
};

// German Localization structure
const LANGUAGE_DE: Localization = Localization {
    title: "🎲 Rust Roller CLI Tool 🇩🇪",
    title_separator: "=================================================",
    die_sides_prompt: "Geben Sie die maximale Anzahl der Würfelseiten ein (z.B. 20):",
    target_value_prompt: "Geben Sie den erforderlichen Erfolgswert ein (Zielwert):",
    num_rolls_prompt: "Geben Sie die Gesamtzahl der Würfe ein:",
    max_sides_label: "{} Seiten",
    success_req_label: "Erfolg erfordert einen Wurf von {} oder höher. (Crit Success: {})",
    default_max_sides: "10",
    simulation_start_msg: "Simuliert {} Würfe eines {}-seitigen Würfels...",
    success_high_label: "Erfolg",
    critical_success_label: "KRITISCHER ERFOLG",
    roll_log_msg: "Wurf {} ({})",
    failure_label: "Fehler",
    summary_title: &'static str = "=================================================",
    pre_distraction_label: "Reguläre Erfolge (Pre-Distraktion)",
    bonus_rolls_label: "Bonuswürfe",
    total_bonus_label: "Durchgeführte Bonuswürfe",
    bonus_success_label: "Bonuserfolge",
    bonus_failure_label: "Bonusfehler",
    final_result_label: "Final angepasste reguläre Erfolge",
    exit_msg: "👋 Vielen Dank für die Nutzung des Rust Roller CLI Tool!🇩🇪",
    continue_prompt: "Weiter simulieren? (true/false):",
    error_input_prompt: "❌ Ungültige Eingabe. Bitte geben Sie einen gültigen Wert ein.",
};

// French Localization structure
const LANGUAGE_FR: Localization = Localization {
    title: "🎲 Outil CLI de Rouleau Rust 🇫🇷",
    title_separator: "=================================================",
    die_sides_prompt: "Entrez le nombre maximum de faces du dé (ex: 20):",
    target_value_prompt: "Entrez la valeur de succès requise (Valeur cible):",
    num_rolls_prompt: "Entrez le nombre total de lancers de dés à simuler:",
    max_sides_label: "{} faces",
    success_req_label: "Le succès nécessite un lancer de {} ou plus. (Critère de Succès: {})",
    default_max_sides: "10",
    simulation_start_msg: "Simulation {} lancers avec un dé de {} faces...",
    success_high_label: "Succès",
    critical_success_label: "SUCCÈS CRITIQUE",
    roll_log_msg: "Lancer {} ({})",
    failure_label: "échec",
    summary_title: &'static str = "=================================================",
    pre_distraction_label: "Succès Réguliers (Pré-distraction)",
    bonus_rolls_label: "Rouleaux Bonus",
    total_bonus_label: "Total de Rouleaux Bonus Effectués",
    bonus_success_label: "Succès Bonus",
    bonus_failure_label: "Échecs Bonus",
    final_result_label: "Succès Réguliers Ajustés Finaux",
    exit_msg: "👋 Merci d'avoir utilisé l'outil Rust Roller CLI !🇫🇷",
    continue_prompt: "Voulez-vous simuler à nouveau ? (true/false):",
    error_input_prompt: "❌ Entrée invalide. Veuillez entrer une valeur valide.",
};

// Czech Localization structure
const LANGUAGE_CS: Localization = Localization {
    title: "🎲 Rust Roller CLI Tool 🇨🇿",
    title_separator: "=================================================",
    die_sides_prompt: "Zadejte maximální počet stran koukoli (např. 20):",
    target_value_prompt: "Zadejte požadovanou hodnotu úspěchu (Cílová hodnota):",
    num_rolls_prompt: "Zadejte celkový počet hodů:",
    max_sides_label: "{} stran",
    success_req_label: "Úspěch vyžaduje hod {} nebo vyšší. (Kritický úspěch: {})",
    default_max_sides: "10",
    simulation_start_msg: "Simuluje {} hodů koukoli se {} stranami...",
    success_high_label: "Úspěch",
    critical_success_label: "KRITICKÝ ÚSPĚCH",
    roll_log_msg: "Hod {} ({})",
    failure_label: "neúspěch",
    summary_title: &'static str = "=================================================",
    pre_distraction_label: "Základní úspěchy (Před odstraňováním)",
    bonus_rolls_label: "Bonus výpady",
    total_bonus_label: "Celkový počet bonus výpadků",
    bonus_success_label: "Úspěchy z bonusu",
    bonus_failure_label: "Neúspěchy z bonusu",
    final_result_label: "Celkově upravené základní úspěchy",
    exit_msg: "👋 Děkujeme za použití nástroje Rust Roller CLI!🇨🇿",
    continue_prompt: "Chcete simulovat znovu? (true/false):",
    error_input_prompt: "❌ Neplatní vstup. Zadejte prosím platní hod.",
};

// --- State Management ---

// A small struct to hold the accumulated state of all rolls.
#[derive(Debug, Clone, Default, Copy)]
struct RollResults {
    regular_successes: u32,
    critical_successes: u32,
    regular_failures: u32,
    critical_failures: u32,

    bonus_rolls_performed: u32,
    bonus_successes: u32,
    bonus_failures: u32,
}

impl RollResults {
    // Factory method for creating the initial state.
    fn initial() -> Self {
        RollResults {
            regular_successes: 0,
            critical_successes: 0,
            regular_failures: 0,
            critical_failures: 0,
            bonus_rolls_performed: 0,
            bonus_successes: 0,
            bonus_failures: 0,
        }
    }

    // Function to combine two states (useful in fold/reduce patterns).
    fn combine(&self, other: &RollResults) -> RollResults {
        RollResults {
            regular_successes: self.regular_successes + other.regular_successes,
            critical_successes: self.critical_successes + other.critical_successes,
            regular_failures: self.regular_failures + other.regular_failures,
            critical_failures: self.critical_failures + other.critical_failures,
            bonus_rolls_performed: self.bonus_rolls_performed + other.bonus_rolls_performed,
            bonus_successes: self.bonus_successes + other.bonus_successes,
            bonus_failures: self.bonus_failures + other.bonus_failures,
        }
    }
    
    // Calculates the final adjusted success count based on game rules. 
    fn calculate_final_successes(&self) -> u32 {
        if self.regular_successes >= self.critical_failures { 
            self.regular_successes - self.critical_failures 
        } else { 
            0 
        }
    }
}

// --- Core Logic ---

/// Processes a single roll event and returns the state update (delta) and the log message.
/// This function is pure: given the same input, it always returns the same output 
/// without modifying outside state. 
fn process_roll(roll: u32, target_value: u32, max_sides: u32, rng: &mut impl Rng) -> RollResults {
    // Base case: No change in state 
    let mut new_results = RollResults::initial(); 

    if roll == max_sides { 
        // Critical Success (Handles the bonus roll logic internally) 
        let mut temp_results = RollResults::initial(); 
        temp_results.critical_successes = 1; 
        
        // Bonus Roll 
        let bonus_roll = rng.gen_range(1..=max_sides); 
        temp_results.bonus_rolls_performed = 1; 
        
        if bonus_roll >= target_value { 
            temp_results.bonus_successes = 1; 
        } else { 
            temp_results.bonus_failures = 1; 
        }
        
        new_results.regular_successes = 0; // Overwrite base success for this roll type 
        new_results.critical_successes = temp_results.critical_successes + 1; 
        new_results.bonus_rolls_performed = temp_results.bonus_rolls_performed; 
        new_results.bonus_successes = temp_results.bonus_successes; 
        new_results.bonus_failures = temp_results.bonus_failures; 

    } else if roll == 1 { 
        // Critical Failure 
        new_results.critical_failures = 1; 
    } else if roll >= target_value { 
        // Regular Success 
        new_results.regular_successes = 1; 
    } else { 
        // Regular Failure 
        new_results.regular_failures = 1; 
    } 

    new_results 
}


// Helper function to read a typed input from the user
fn get_input<T: std::str::FromStr>(prompt_key: &Localization, prompt_template: &str) -> T {
    loop {
        print!("{}: ", &format!("[{}]{}", prompt_key.title, prompt_template));
        io::stdout().flush().expect("Failed to flush stdout"); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        match input.trim().parse::<T>() {
            Ok(value) => break value,
            Err(_) => {
                println!("{} {}", prompt_key.error_input_prompt, ".");
            }
        }
    }
}

fn main() { 
    // --- Language Selection ---
    let locale: &Localization = loop {
        println!("\n=================================================");
        println!("          {}          ", "Rust Roller CLI Tool");
        println!("=================================================");

        print!("Please select a language or type 'exit': ");
        io::stdout().flush().expect("Failed to flush stdout"); 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let selection = input.trim().to_lowercase();

        match selection.as_str() {
            "en" | "english" => break &LANGUAGE_EN_US,
            "fr" | "french" => break &LANGUAGE_FR,
            "de" | "german" => break &LANGUAGE_DE,
            "cs" | "czech" => break &LANGUAGE_CS,
            "exit" => {
                println!("👋 Thanks for using the Rust Roller CLI Tool!");
                return;
            }
            _ => {
                println!("🌍 Unknown language. Please try English (en), French (fr), German (de), or Czech (cs).");
            }
        }
    };
    
    loop {
        println!("\n=================================================");
        println!("          {}          ", locale.title);
        println!("=================================================");

        // Gather interactive inputs using the localized prompts
        let max_sides: u32 = get_input(&locale, &format!("{}{} {}", locale.die_sides_prompt, locale.default_max_sides, ""));
        let target_value: u32 = get_input(&locale, &locale.target_value_prompt);
        let num_rolls: u32 = get_input(&locale, &locale.num_rolls_prompt);

        if max_sides < 2 || num_rolls == 0 {
            println!("\n🛑 Die sides and number of rolls must be at least 2/greater than zero. Exiting.");
            break;
        }
        
        let mut rng = rand::thread_rng(); 
        
        println!("\n=================================================");
        println!("   🎲 Simulating {} rolls of a {}-sided die... 📊", num_rolls, max_sides);
        println!("   ✅ Success requires a roll of {} or higher. (Crit Success: {})", target_value, max_sides);
        println!("=================================================\n");


        // Use fold to iteratively process the state, creating an immutable flow. 
        // We now collect the logs separately to separate side effects (printing) from state calculation.
        let mut accumulated_results = RollResults::initial();
        
        for i in 1..=num_rolls { 
            let roll = rng.gen_range(1..=max_sides); 
            
            // Process the current roll to get the delta state and the log message
            let roll_delta = process_roll(roll, target_value, max_sides, &mut rng); 

            // Log action (This is the only place side effects happen) 
            match roll { 
                max_sides => println!("Roll {} ({})", i, locale.critical_success_label), 
                1 => println!("Roll {} ({})", i, locale.critical_failure_label), 
                r if r >= target_value => println!("Roll {} ({}) {}", i, r, locale.success_high_label), 
                r => print!("Roll {} ({}) {} ", i, r, locale.failure_label), // Use print! for single line logging
            }
            
            // Aggregate the state immutably
            accumulated_results = accumulated_results.combine(&roll_delta); 
        } 


        println!("\n=================================================");
        println!("                📊 Simulation Summary 📊                ");
        println!("=================================================");
        println!("✅ {}: {}", locale.pre_distraction_label, accumulated_results.regular_successes); 
        println!("⭐ {}: {}", locale.critical_success_label, accumulated_results.critical_successes); 
        println!("❌ {}: {}", locale.regular_failures_label, accumulated_results.regular_failures); 
        println!("🚨 {}: {}", locale.critical_failures_label, accumulated_results.critical_failures); 

        println!("\n--- {} (Triggered by {}-roll) ---", locale.bonus_rolls_label, max_sides); 
        println!("  ▶ {}: {}", locale.total_bonus_label, accumulated_results.bonus_rolls_performed); 
        println!("  ✅ {}: {}", locale.bonus_success_label, accumulated_results.bonus_successes); 
        println!("  ❌ {}: {}", locale.bonus_failure_label, accumulated_results.bonus_failures); 

        println!("\n=================================================");
        println!("   🏆 FINAL ADJUSTED RESULT: {} / {}", accumulated_results.calculate_final_successes(), num_rolls);
        println!("=================================================\n");
        
        // Ask user if they want to run again
        let continue_running: bool = get_input(&locale, &locale.continue_prompt);
        if continue_running.to_lowercase() != "true" {
            println!("\n{} {}", locale.exit_msg, ".");
            break;
        }
    }
}
