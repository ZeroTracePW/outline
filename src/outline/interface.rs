pub mod interface {
    use std::collections::HashMap;
    use std::io::{stdin, stdout, Write};
    use crossterm::cursor::MoveTo;
    use crossterm::execute;
    use crossterm::style::{Color, Stylize};
    use crossterm::terminal::{Clear, ClearType, SetTitle};

    fn formatted_text(option: &str, text: &str) -> String {
        let map: HashMap<String, Color> = HashMap::from([
            (">".to_string(), Color::Rgb { r: 148, g: 0, b: 211 }),
            ("+".to_string(), Color::Rgb { r: 0, g: 255, b: 0 }),
            ("-".to_string(), Color::Rgb { r: 255, g: 0, b: 0 }),
            ("?".to_string(), Color::Rgb { r: 255, g: 165, b: 0 }),
        ]);

        let selected_color = match map.get(option) {
            Some(color) => *color,
            None => Color::Rgb { r: 0, g: 255, b: 255 },
        };

        let formatted_text = format!(
            "{}{}{} {}",
            "[".dark_grey(),
            option.with(selected_color),
            "]".dark_grey(),
            text.grey()
        );

        formatted_text
    }

    pub fn write(option: &str, text: &str) {
        let text = formatted_text(option, text);
        print!("{}", text);
    }

    pub fn write_ln(option: &str, text: &str) {
        let text = formatted_text(option, text);
        println!("{}", text);
    }

    pub fn logo() {
        clear();
        let ascii: [&str; 6] = [
            " ██████╗ ██╗   ██╗████████╗██╗     ██╗███╗   ██╗███████╗",
            "██╔═══██╗██║   ██║╚══██╔══╝██║     ██║████╗  ██║██╔════╝",
            "██║   ██║██║   ██║   ██║   ██║     ██║██╔██╗ ██║█████╗  ",
            "██║   ██║██║   ██║   ██║   ██║     ██║██║╚██╗██║██╔══╝  ",
            "╚██████╔╝╚██████╔╝   ██║   ███████╗██║██║ ╚████║███████╗",
            " ╚═════╝  ╚═════╝    ╚═╝   ╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝",
        ];

        let mut colors = (0, 255, 255);

        for line in ascii.iter() {
            println!("{}", line.with(Color::Rgb { r: colors.0, g: colors.1, b: colors.2 }));

            colors.1 = colors.1.saturating_sub(29);
            colors.2 = colors.2.saturating_sub(29);
        }
    }

    // Use "FromCursorDown" for Updating Numbers without clearing the screen.
    pub fn clear() {
        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0)
        ).unwrap();
    }

    pub fn set_title(title: &str) {
        execute!(
            stdout(),
            SetTitle(title)
        ).unwrap();
    }

    pub fn input() -> String {
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .unwrap_or_else(|_| {
                write_ln("-", "Failed to read line");
                std::process::exit(1);
            });

        input.trim().to_string()
    }

    pub fn input_with_prompt(prompt: &str) -> String {
        write_ln("?", prompt);
        write(">", "");
        input()
    }
}
