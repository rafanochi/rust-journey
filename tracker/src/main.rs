use gtk::prelude::{BoxExt, ButtonExt, OrientableExt};
use relm4::{RelmApp, RelmWidgetExt, SimpleComponent, gtk};
use track::{gen_unique_icon, random_icon};

#[tracker::track]
struct AppModel {
    first_icon: &'static str,
    second_icon: &'static str,
    identical: bool,
}

#[derive(Debug)]
enum AppInput {
    UpdateFirst,
    UpdateSecond,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view!(
        #[root]
        gtk::ApplicationWindow{
            #[track = "model.changed(AppModel::identical())"]
            set_class_active: ("identical", model.identical),

            gtk::Box{
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_margin_all: 100,

                gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image{
                        set_pixel_size: 50,
                        #[track="model.changed(AppModel::first_icon())"]
                        set_icon_name: Some(model.first_icon)
                    },
                    gtk::Button {
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateFirst);
                        }
                    }
                },

                append = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Image {
                        set_pixel_size: 50,
                        #[track="model.changed(AppModel::second_icon())"]
                        set_icon_name: Some(model.second_icon)
                    },
                    gtk::Button{
                        set_label: "New random image",
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::UpdateSecond);
                        }
                    }
                }
            }
        }
    );

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel {
            first_icon: random_icon(),
            second_icon: random_icon(),
            identical: false,
            tracker: 0,
        };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        self.reset();

        match message {
            AppInput::UpdateFirst => {
                self.set_first_icon(gen_unique_icon(self.first_icon));
            }
            AppInput::UpdateSecond => {
                self.set_second_icon(gen_unique_icon(self.second_icon));
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("rafa.test.tracker");
    relm4::set_global_css(".identical { background: #00ad5c; }");
    app.run::<AppModel>(());
}
