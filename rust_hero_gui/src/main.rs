//Remove tela de linha de comando
#![windows_subsystem = "windows"]

// Biblioteca interface
use fltk::{app::*, button::*, frame::*, menu::*, window::*};

pub fn main() {
    let app = App::default().with_scheme(AppScheme::Gtk);
    let mut janela = Window::default()
        .with_size(300, 500)
        .center_screen()
        .with_label("Rust Hero");

    let mut frame = Frame::default().with_size(300, 50).with_label("Rust Hero");

    frame.set_color(Color::from_u32(0xaaaaaa));
    frame.set_label_size(30);
    frame.set_frame(FrameType::DownBox);

    let mut botao_novo = Button::default()
        .with_size(100, 50)
        .below_of(&frame, 0)
        .with_label("Novo jogo");

    botao_novo.set_color(Color::from_u32(0xbbbbbb));

    let mut botao_salvar = Button::default()
        .size_of(&botao_novo)
        .right_of(&botao_novo, 0)
        .with_label("Salvar");

    botao_salvar.set_color(Color::from_u32(0x2e7d32));

    let mut botao_carregar = Button::default()
        .size_of(&botao_salvar)
        .right_of(&botao_salvar, 0)
        .with_label("Carregar");

    botao_carregar.set_color(Color::from_u32(0x1565c032));

    botao_novo.set_callback(Box::new(|| {
        let v = vec!["1st val", "2nd val", "3rd val"];
        let mut x = MenuItem::new(&v);
        match x.popup(100, 100) {
            None => println!("No value was chosen!"),
            Some(val) => println!("{}", val.label().unwrap()),
        }
    }));

    janela.make_resizable(false);
    janela.end();
    janela.show();
    app.run().unwrap();
}