use console

fn console_print(s: &String,mumber: &u8){
    match number{
        0 => print!(console::style(s).bold()),
        1 => print!(console::style(s).bold().red()),
        2 => print!(console::style(s).bold().yellow()),
        3 => print!(console::style(s).bold().green()),
        _ => print!(console::style(s).bold().blink),
    }
}