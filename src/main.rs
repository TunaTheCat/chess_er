
use fltk::{app, button::Button, enums, frame, frame::Frame, group, prelude::*, window::Window};
use fltk::enums::{Color, FrameType};
use fltk::group::experimental::Grid;

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 600, 700, "Chess Johnny");
    wind.set_color(Color::Dark1);
    let mut grp = group::Group::default().size_of(&wind);


    let mut grid = Grid::new(grp.width() / 2 - 200, grp.height() / 2 - 200, 400, 400, "");

    grid.show_grid(false); // set to true to show cell outlines and numbers
    grid.set_layout(8, 8);

    for row in 0..8 {
        for col in 0..8 {
            let mut f = Frame::default();
            f.set_frame(enums::FrameType::FlatBox);
            if (row + col ) % 2 == 0 {
                f.set_color(Color::White);
            } else {
                f.set_color(Color::Black);
            }
            grid.set_widget(&mut f,row,col);
        }
    }


    grid.end();
    grp.end();

    wind.end();
    wind.show();
    app.run().unwrap();

}

