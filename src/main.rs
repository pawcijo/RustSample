extern crate azul;

use azul::{prelude::*, widgets::{label::Label, button::Button}};

struct CounterApplication {
    counter: usize,
}

impl Layout for CounterApplication {

    fn layout(&self, _info: LayoutInfo<Self>) -> Dom<Self> {
        let what = Label::new("Cook(ies):").dom();
        let label = Label::new(format!("{}", self.counter)).dom();
        let button = Button::with_label("Cook(ie) counter").dom()
            .with_callback(On::MouseUp, update_counter);

        Dom::div()
            .with_child(what)
            .with_child(label)
            .with_child(button)
    }

}

fn update_counter(event: CallbackInfo<CounterApplication>) -> UpdateScreen {
    event.state.data.counter += 1;
    Redraw
}



fn main() {
    let mut app = App::new(CounterApplication { counter: 0 }, AppConfig::default()).unwrap();
    
    
    let mut window_options = WindowCreateOptions::default();
    window_options.state.title = "Cook (ie) clicker".into();
    
    let window = app.create_window(window_options, css::native()).unwrap();
    app.run(window).unwrap();
}