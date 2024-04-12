#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit,
}
fn get_choice(input: &str) -> Result<MenuChoice,String>{
    match input{
        "mainmenu"=>Ok(MenuChoice::MainMenu),
        "start"=>Ok(MenuChoice::Start),
        "quit"=>Ok(MenuChoice::Quit),
        _=>Err("Invalid choice".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice){
   println!("choice={:?}", choice);
}
fn pick_choice(input: &str)-> Result<(),String>{
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}
fn main() {
    pick_choice("mainmenu");
    pick_choice("start");
    pick_choice("quit");
    pick_choice("invalid");
}
