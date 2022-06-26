use std::io;

struct Character
{
    name: String,
    health: i16,
    attack: i16,
    defense: i16,
}

impl Character
{
    fn temporary_buff(&mut self, buff_param: i16, attribute_param: i16) -> i16
    {
            buff_param * attribute_param
    }
}

fn main()
{
    let mut valnir_reaper = Character
    {
        name: "Valnir the Reaper".to_string(),
        attack: 27,
        defense: 21,
        health: 150,
    };

    let mut player_char = create_character();
    println!("Name: {}\nHealth: {}\nAttack: {}\nDefense: {}",
    player_char.name, player_char.health, player_char.attack, player_char.defense);

    battle(&mut player_char, &mut valnir_reaper);
}

fn create_character() -> Character
{
// There is a bug in this program that still needs to be addressed. If the user inserts values of a type that is not
// the type of that particular variable, the program crashes. For example, if the value s/he inserts is i16 instead of i8,
// or a letter instead of an integer, the program panicks. As I learn Rust more and more, I believe I will be able to
// address this issue, and hopefully fix this bug eventually.
    let mut output_character = Character
    {
        name: "a_name".to_string(),
        attack: 0,
        defense: 0,
        health: 0,
    };

    let mut player_char = String::new();
    println!("Type in the name of your character that will do battle against the enemy!");
    io::stdin().read_line(&mut player_char).expect("Failed to get input.");
    player_char = player_char.trim().to_string();
    output_character.name = player_char.clone();

    loop
    {
        let mut remaining_points:i16 = 50;
        let temp_defense_points:i16 = 10;
        let temp_attack_points:i16 = 10;
        let temp_health_points:i16 = 50;

        println!("Now it's time to put points in your character!\n===========\nYou have {} points to distribute across your \
        character's health points, attack and defense.\n===========\nYou start with 10 points in attack, 10 in defense and 50 \
        in health; the other 50 points will be added according to your choice\n===========\nHowever, you will first be prompted \
        to put points into your character's attack and defense, while the remaining points will be multiplied by 10 and added to \
        your character's health points.", remaining_points);

        println!("You have {} points to distribute across your character's health points, attack and defense.", 
        remaining_points);

        loop
        {
            println!("===========\nHow many points will you put into your character's attack?");
            let mut attack_character = String::new();
            io::stdin().read_line(&mut attack_character).expect("Failed to get attack input.");
            let mut attack_character:i16 = attack_character.trim().parse().expect("Failed to convert to integer.");

            if (attack_character > remaining_points) || (attack_character < 0)
            {
                println!("You cannot put more than {} or less than 0 points into your character's attack.", remaining_points);
            } else
            {
                output_character.attack = attack_character + temp_attack_points;
                remaining_points -= attack_character;
                println!("By default your character has {} points in attack. Now {} points have been added, and \
                {}'s final attack is then set to {}!", temp_attack_points, attack_character, output_character.name,
                output_character.attack);
                break;
            }
        }

        if remaining_points > 0
        {
            loop
            {
                println!("You now have {} points remaining to distribute.\n===========\nHow many points would you \
                like to put on your defense?", remaining_points);
                let mut defense_character = String::new();
                io::stdin().read_line(&mut defense_character).expect("Failed to get attack input.");
                let mut defense_character:i16 = defense_character.trim().parse().expect("Failed to convert to integer.");

                if (defense_character > remaining_points) || (defense_character < 0)
                {
                    println!("You cannot put more than {} or less than 0 points into your character's defense.", remaining_points);
                } else
                {
                    output_character.defense = defense_character + temp_defense_points;
                    remaining_points -= defense_character;
                    println!("By default your character has {} points in defense. Now {} points have been added, and \
                    {}'s final defense is then set to {}!\n===========\n", temp_defense_points, defense_character, output_character.name,
                    output_character.defense);

                    output_character.health = temp_health_points + (remaining_points as i16 * 10);
                    println!("The remaining {} points have been multiplied by 10 and added to your character's health points!\
                    \n===========\nYour character's health points are thus {}!", remaining_points, output_character.health);
                    break;
                }
            }
        } else
        {
            output_character.health = temp_health_points;
            output_character.defense = temp_defense_points;
        }


        loop
        {
            let mut confirm = String::new();
            println!("Your health is {}\nYour attack is {}\nYour defense is {}\n-> Do you confirm this, or would you like to \
            insert new values? Press 1 to confirm or 0 to start again and insert new values:", output_character.health,
            output_character.attack, output_character.defense);
            io::stdin().read_line(&mut confirm).expect("Failed to read input.");
            let mut confirm:i8 = confirm.trim().parse().expect("Failed to convert to integer.");
    
            if confirm == 1
            {
                println!("Your character is now ready to do battle!\n===========\nHere is your character's information:\
                \n===========\n-> Your character's name is {}\n-> Your character's health points are {}\n-> Your \
                character's attack is {}\n-> The defense is {}", output_character.name, output_character.health,
                output_character.attack, output_character.defense);
    
                return output_character;
            } else if confirm == 0
            {
                println!("Then let's start over!");
                break;
            } else
            {
                println!("Please insert a valid answer.");
            }
        }
    }
}

fn result_attack_defense(attack_param: i16, defense_param: i16) -> i16
{
    if attack_param - defense_param > 0
    {
        return attack_param - defense_param;
    } else
    {
        return 0;
    }
}

fn battle(mut player_param: &mut Character, mut enemy_param: &mut Character)
{
    let mut continue_number = String::new();
    let mut turn_number:i32 = 0;
    let enemy_init_attack = enemy_param.attack;
    let player_init_attack = player_param.attack;
    let buff_number:i16 = 2;

    println!("You are doing battle against {}! Let us see if you can win this \
    fight!\n===========\n(Press any number to continue)", enemy_param.name);
    io::stdin().read_line(&mut continue_number).expect("Failed to read input.");
    let mut continue_number:i8 = continue_number.trim().parse().expect("Failed to convert to integer.");

    println!("You start attacking!\n===========\n");

    loop
    {
        let mut continue_number = String::new();
        // For now the values of damage_against_enemy an damage_against_player won't change because I've
        // been having difficulty using Rust's Random number library (https://rust-random.github.io/book/intro.html)
        // In the future I intend to modify this, so that the attack and defense values are random within a certain
        // range

        turn_number += 1;
        println!("======This is turn number {}!======", turn_number);

        if turn_number >= 5 && turn_number <= 10
        {
            println!("{} now has a buff of {}!", enemy_param.name, buff_number);
            enemy_param.attack = enemy_param.temporary_buff(buff_number, enemy_param.attack);
            println!("Now his attack is {}! Will you be able to defeat him now...?", enemy_param.attack);

            if turn_number >= 7
            {
                println!("The Gods favor you! Now YOU also have acquired a buff of {} in your attack!", buff_number);
                player_param.attack = player_param.temporary_buff(buff_number, player_param.attack);
                println!("Now your mighty attack is {}!", player_param.attack);
            }
        } else if turn_number == 11
        {
            enemy_param.attack = enemy_init_attack;
            player_param.attack = player_init_attack;
            println!("{}'s buff has worn out and his attack went back to being {}", enemy_param.name, enemy_param.attack);
            println!("Your buff also has worn out! Your attack is now back to being {}", player_param.attack);
        }

        let mut damage_against_enemy:i16 = result_attack_defense(player_param.attack , enemy_param.defense);
        let mut damage_against_player:i16 = result_attack_defense(enemy_param.attack, player_param.defense);

        println!("Press any number to attack the enemy!");
        io::stdin().read_line(&mut continue_number).expect("Failed to read input.");
        let mut continue_number:i8 = continue_number.trim().parse().expect("Failed to convert to integer.");

        println!("Your foe's current health is {}.", enemy_param.health);

        if damage_against_enemy == 0
        {
            println!("-> Curses! The enemy's armor is too strong! It has fully blocked your attack!");
        } else
        {
            println!("-> You have dealt {} points of damage against {}!", damage_against_enemy, enemy_param.name);
            enemy_param.health -= damage_against_enemy;

            if enemy_param.health <= 0
            {
                enemy_param.health = 0;
                println!("\n===========\nVictory! You have crushed {}! Rejoice upon your glorious victory!",
                enemy_param.name);
                break;
            } else
            {
                println!("Your opponent now has {} points of health!", enemy_param.health);
                println!("\n===========\nNow prepare yourself for the enemy's counter-attack!\n===========\n");
            }
        }

        println!("Your current health is {}", player_param.health);
        if damage_against_player == 0
        {
            println!("-> Ha! Your armor is indeed strong! It has fully blocked the enemy's attack!");
        } else
        {
            println!("-> {} has dealt {} points of damage against you!", enemy_param.name, damage_against_player);
            player_param.health -= damage_against_player;

            if player_param.health <= 0
            {
                player_param.health = 0;
                println!("\n===========\nYou have been utterly beaten by {}! Your body now lays lifeless on the \
                 battlefield!", enemy_param.name);
                break;
            } else
            {
                println!("You now have {} points of health!", player_param.health);
                println!("\n===========\nNow the enemy prepares for your attack!\n===========\n");
            }
        }

        enemy_param.attack = enemy_init_attack;
        player_param.attack = player_init_attack;
    }
}
