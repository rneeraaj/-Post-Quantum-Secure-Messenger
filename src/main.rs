use std::io::{self, Write};

mod user;
mod chatroom;

use user::User;
use chatroom::ChatRoom;

fn main() {
    let mut chatroom = ChatRoom::new();

    println!("🚀 Post-Quantum Secure Messenger Started!");
    println!("👥 Type 'new' to create a new user");
    println!("👥 Type 'list' to see all users");
    println!("👥 Type 'exit' to quit");

    loop {
        println!("\n🔁 New Message");

        // Get sender
        print!("From: ");
        io::stdout().flush().unwrap();
        let mut from = String::new();
        io::stdin().read_line(&mut from).unwrap();
        let from = from.trim();
        
        if from.eq_ignore_ascii_case("exit") { break; }
        
        if from.eq_ignore_ascii_case("new") {
            print!("Enter new username: ");
            io::stdout().flush().unwrap();
            let mut username = String::new();
            io::stdin().read_line(&mut username).unwrap();
            let username = username.trim();
            
            if chatroom.has_user(username) {
                println!("❌ User '{}' already exists.", username);
                continue;
            }
            
            let new_user = User::new(username);
            chatroom.add_user(new_user);
            println!("✅ User '{}' created successfully!", username);
            continue;
        }
        
        if from.eq_ignore_ascii_case("list") {
            println!("👥 Available Users:");
            for user in chatroom.get_users() {
                println!("- {}", user);
            }
            continue;
        }

        // Check if sender exists
        if !chatroom.has_user(from) {
            println!("❌ Unknown user '{}'. Please enter a valid username or type 'new' to create one.", from);
            continue;
        }

        // Get recipient
        print!("To: ");
        io::stdout().flush().unwrap();
        let mut to = String::new();
        io::stdin().read_line(&mut to).unwrap();
        let to = to.trim();
        if to.eq_ignore_ascii_case("exit") { break; }

        // Check if recipient exists
        if !chatroom.has_user(to) {
            println!("❌ Unknown user '{}'. Please enter a valid username.", to);
            continue;
        }

        // Get message
        print!("Message: ");
        io::stdout().flush().unwrap();
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).unwrap();
        let msg = msg.trim();
        if msg.eq_ignore_ascii_case("exit") { break; }

        // Send encrypted message
        match chatroom.send_encrypted(from, to, msg) {
            Some((ciphertext, nonce)) => {
                println!("\n🔐 Message sent securely!");
                println!("📦 Ciphertext: {:?}", ciphertext);
                println!("🕵️‍♀️ Nonce: {:?}", nonce);

                match chatroom.receive_encrypted(to, from, &ciphertext, &nonce) {
                    Some(decrypted) => println!("📬 {} received: \"{}\"", to, decrypted),
                    None => println!("❌ {} failed to decrypt the message.", to),
                }
            }
            None => println!("❌ Failed to send encrypted message. Check usernames."),
        }
    }

    println!("\n👋 Exiting Secure Messenger. Stay quantum safe!");
}
