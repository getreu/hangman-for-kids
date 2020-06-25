//! Loads the configuration and runs the game workflow.
extern crate crossterm;
extern crate rand;
extern crate thiserror;
mod application;
mod dictionary;
mod game;
mod image;
mod secret;
use application::Application;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::path::PathBuf;
use std::process;

use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToNextLine;
use crossterm::queue;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::SetForegroundColor;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use std::io::stdout;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const AUTHOR: &str = "(c) Jens Getreu, 2016-2020.";

/// Title line.
const TITLE: &str = "ASCII-ART HANGMAN FOR KIDS\n";

/// Text to show as command-line help --help
const COMMANDLINE_HELP: &str = r#"
Hangman is a paper and pencil guessing game for two or more players.  One player
thinks of a word, phrase or sentence and the other tries to guess it by
suggesting letters or numbers, within a certain number of guesses. In this
version for children the computer selects a word, phrase or sentence randomly
out of a word-list defined in a configuration file. In the course of the game
Ascii-Art images - designed for children - are progressively disclosed.

===================================
-----------------------------------
ASCII-ART HANGMAN FOR KIDS

         ,.
        (_|,.
       ,' /, )_______
    __j o``-'        `
   (")
    `-j                |
      `-._(           /
         |_\  |--^.  /
        /_]'|_| /_)_/
           /_]'  /_]'

Lifes:  1       Last guess: 3

 g o o _ _ l _ _ k

Type a letter then type [Enter]:
-----------------------------------
===================================

 Usage: ascii-hangman
        ascii-hangman [FILE]...
        ascii-hangman -h|--help
        asciiart-ascii-hangman-for-kids -V|--version


`[FILE]` are configuration files containing word-lists and optionally Ascii-Art
images.

When no `[FILE]` argument is given, `[FILE]` defaults to `ascii-hangman-words.txt`. In
case no `[FILE]` is found, a template configuration file `ascii-hangman-words.txt` is
written into the current working directory. Multiple `[FILE]`s are concatenated.

`[FILE]` is a UTF-8 file containing 4 different line-types:

- lines starting with a letter, a digit or '-' are secret strings. At the
  beginning of the game one line is randomly chosen and all characters are
  hidden. If you want to give an additional hint, enclose some characters
  with `_`.  The enclosed is then displayed in clear when the game starts.
  For example the configuration line:

      Guess _me_

  is shown in the game as:

      _ _ _ _ _ _ m e

- lines starting with `#` are ignored. This can be used for comments.

- lines starting with `|` are part of an optional custom ASCII-Art image shown
  progressively in the course of the game. If not defined here, built in
  ASCII-Art images are used instead.

- lines starting with `:` are game modifier. They change the logic how the image
  is progressively disclosed:

   `:success-rewarding`       Every guessed character shows a bit more of
                              the image. This mode is default.
   `:traditional-rewarding`   Every lost live discloses a bit more of the
                              image. Choose this mode together with a
                              traditional gallows image (not built in).

The following shows an example of a custom image (copy it left-aligned
into the config-file):

        |  ,~~--~~-.
        | +      | |\
        | || |~ |`,/-\
        | *\_) \_) `-'

If you prefer a traditional gallows image, add the following:

        :traditional-rewarding
        |  ______
        |  |    |
        |  |    O
        |  |   /|\
        |  |    |
        |  |   / \
        |__|_____
        ||      |___
        ||_________|

"#;

/// Number of wrong guess allowed.
const LIVES: u8 = 7;
/// Default configuration filename when no filename is given at the command-line.
const PATHSTR: &str = "ascii-hangman-words.txt";
/// Fallback sample configuration when no configuration file can be found.
const CONF_TEMPLATE: &str =
    "# Type `ascii-hangman -h` to learn how to insert custom ASCII-art images here. \r
\r
guess me\r
hang_man_\r
_good l_uck\r
_3*_7_=21_\r
";

/// Fallback secret when no configuration file can be found.
const CONF_DEMO: &str = "- _Demo: add own words to config file and start a_gain_!";

// ------------------ MAIN ---------------------------------------------

/// Reads the configuration file.
pub fn read_config(pathstr: &PathBuf) -> Result<String, io::Error> {
    let mut f = File::open(pathstr)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

/// Writes a sample configuration file on disk. Called when no configuration file can be found.
pub fn write_config_template(pathstr: &PathBuf) -> Result<(), io::Error> {
    let mut file = File::create(&pathstr)?;
    file.write_all(CONF_TEMPLATE.as_bytes())?;
    Ok(())
}

/// Starts the game.
#[allow(unused_labels)]
fn main() {
    // SHOW HELP TEXT
    match env::args().nth(1) {
        Some(ref a) if a == "-h" || a == "--help" => {
            eprintln!("{}", COMMANDLINE_HELP);
            return;
        }
        Some(ref a) if a == "-V" || a == "--version" => {
            eprintln!("{}", VERSION.unwrap());
            return;
        }
        Some(_) | None => {}
    };

    // READ CONFIG

    // Read all config files given on command line
    let mut conf_file_paths = env::args()
        .skip(1)
        .map(|s| PathBuf::from(s))
        .collect::<Vec<PathBuf>>();

    // if no conf_file_paths are given then use default config path
    if conf_file_paths.is_empty() {
        conf_file_paths.push(PathBuf::from(PATHSTR))
    };

    // read and concatenate all config files given on command line
    let cwd = env::current_dir().unwrap();

    let mut config: String = String::new();
    for conf_file_path in &conf_file_paths {
        let path = conf_file_path;
        let c = match read_config(&path) {
            Ok(s) => s,
            Err(_) => {
                match write_config_template(&path) {
                    Ok(_) => {
                        eprintln!(
                            "As no config-file :\n\
                             \t{:?}\n\
                             was found a template file is written in the \
                             current working directory.\n\
                             \t{:?}\n\n\nPress [Enter] to enter demo mode.",
                            path, cwd
                        );
                        // wait for [Enter] key
                        let s = &mut String::new();
                        io::stdin().read_line(s).unwrap();
                        CONF_DEMO.to_string()
                    }
                    Err(why) => {
                        eprintln!(
                            "Couldn't write ascii-hangman template \
                             config-file:\n\t{:?}\n({})\n\n\
                             Current working directory is:\n\t{:?}\n\n\
                             Press [Enter] to enter demo mode.",
                            path,
                            why.to_string(),
                            cwd
                        );
                        // wait for [Enter] key
                        let s = &mut String::new();
                        io::stdin().read_line(s).unwrap();
                        CONF_DEMO.to_string()
                    }
                }
            }
        };
        config.push_str(&c);
    }

    // INITIALISE THE GAME

    let mut app = match Application::new(&config) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("ERROR IN CONFIGURATION FILE\n{}", e);

            // wait for [Enter] key
            let s = &mut String::new();
            io::stdin().read_line(s).unwrap();
            process::exit(1);
        }
    };

    app.render();

    // PLAY

    'playing: loop {
        // Read user input
        io::stdout().flush().unwrap();
        // Read next char and send it
        let key = &mut String::new();
        io::stdin().read_line(key).unwrap();

        if !app.process_user_input(key) {
            break;
        };

        app.render();
    }

    println!("\n{}", AUTHOR);
}

trait Render {
    fn render(&self) {}
}

impl Render for Application {
    /// Renders and prints the TUI on the terminal.
    fn render(&self) {
        // Disclose parts of the image.

        // Clear all lines in terminal;
        queue!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();

        #[cfg(not(windows))]
        queue!(stdout(), SetForegroundColor(Color::White),).unwrap();
        #[cfg(windows)]
        queue!(stdout(), SetForegroundColor(Color::Grey),).unwrap();

        queue!(
            stdout(),
            Print(&TITLE),
            MoveToNextLine(1),
            SetForegroundColor(Color::DarkYellow),
        )
        .unwrap();

        // Print image.
        queue!(stdout(), Print(self.render_image()), MoveToNextLine(1)).unwrap();

        // Print game status.
        #[cfg(not(windows))]
        queue!(stdout(), SetForegroundColor(Color::White),).unwrap();
        #[cfg(windows)]
        queue!(stdout(), SetForegroundColor(Color::Grey),).unwrap();
        queue!(stdout(), Print(self.render_game_status())).unwrap();

        // Print secret.
        #[cfg(not(windows))]
        queue!(stdout(), SetForegroundColor(Color::DarkGreen),).unwrap();
        #[cfg(windows)]
        queue!(stdout(), SetForegroundColor(Color::White),).unwrap();
        queue!(stdout(), Print(self.render_secret()), MoveToNextLine(1)).unwrap();

        // Print instructions.
        #[cfg(not(windows))]
        queue!(stdout(), SetForegroundColor(Color::White),).unwrap();
        #[cfg(windows)]
        queue!(stdout(), SetForegroundColor(Color::Grey),).unwrap();

        queue!(
            stdout(),
            Print(self.render_instructions()),
            MoveToNextLine(1)
        )
        .unwrap();
        // Print queued.
        stdout().flush().unwrap();
    }
}
