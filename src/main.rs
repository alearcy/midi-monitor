use console::Term;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use midir::{MidiInput, MidiInputPort};
use std::error::Error;
use std::io::stdin;

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    let midi_input: MidiInput = MidiInput::new("midi_input_monitor").unwrap();
    let ports: &Vec<MidiInputPort> = &midi_input.ports();
    let mut port_names: Vec<String> = Vec::<String>::new();
    let mut midi_ports: Vec<&MidiInputPort> = Vec::new();
    for port in ports.iter() {
        let name = midi_input.port_name(port).unwrap();
        port_names.push(name);
        midi_ports.push(port);
    }
    let midi_selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(&port_names)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();
    let usable_port: &MidiInputPort = if let Some(index) = midi_selection {
        let mut selected_midi_device_name: String = String::from("");
        selected_midi_device_name.push_str(port_names[index].as_str());
        &ports[index]
    } else {
        midi_ports[0]
    };

    println!("\nOpening connection");
    let _conn_in: midir::MidiInputConnection<()> = midi_input
        .connect(
            usable_port,
            "midir-read",
            move |_, bytes, _| {
                println!("Result: {:?}", bytes);
            },
            (),
        )
        .unwrap();
    println!("Connection open, reading input (press enter to exit) ...");
    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}
