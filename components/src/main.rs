use gtk::prelude::{
    ApplicationExt, ButtonExt, DialogExt, GtkWindowExt, ToggleButtonExt, WidgetExt,
};
use relm4::*;

struct HeaderModel;

#[derive(Debug)]
enum HeaderOutput{
    View,
    Edit, 
    Export
}

#[relm4::component]
impl SimpleComponent for HeaderModel {
   type Init = (); 
   type Input = ();
   type Output = HeaderOutput;

   view!{
       #[root]
       gtk::HeaderBar{
           #[wrap(Some)]
           set_title_widget = &gtk::Box{
               add_css_class: "linked",
               #[name = "group"]
               gtk::ToggleButton {
                   set_label: "View",
                   set_active: true,
                   connect_toggled[sender] => move |btn| {
                       if btn.is_active(){
                           sender.output(HeaderOutput::View).unwrap()
                       }
                   }
               },

               gtk::ToggleButton {
                   set_label: "Edit",
                   set_group: Some(&group),
                   connect_toggled[sender] => move |btn| {
                       if btn.is_active(){
                           sender.output(HeaderOutput::Edit).unwrap()
                       }
                   }
               },

               gtk::ToggleButton {
                   set_label: "Export",
                   set_group: Some(&group),
                   connect_toggled[sender] => move |btn| {
                       if btn.is_active() {
                           sender.output(HeaderOutput::Export).unwrap()
                       }
                   }
               }

           }
       }
   }

   fn init(
           init: Self::Init,
           root: Self::Root,
           sender: relm4::ComponentSender<Self>,
       ) -> relm4::ComponentParts<Self> {
       let model = HeaderModel;
       let widgets = view_output!();

       ComponentParts{model, widgets}
   }
}


#[derive(Debug)]
enum AppMode {
    View,
    Edit,
    Export,
}

#[derive(Debug)]
enum AppMsg {
    SetMode(AppMode),
}

struct AppModel{
    mode: AppMode,
    header: Controller<HeaderModel>,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = AppMode;
    type Input = AppMsg;
    type Output = ();

    view! {
        main_window = gtk::Window {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(model.header.widget()),

            gtk::Label{
                #[watch]
                set_label: &format!("placeholder for {:?}", model.mode)
            }
        }
    }

    fn init(
            init: Self::Init,
            root: Self::Root,
            sender: ComponentSender<Self>,
        ) -> ComponentParts<Self> {
       
        let header: Controller<HeaderModel> = HeaderModel::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| match msg {
                HeaderOutput::View => AppMsg::SetMode(AppMode::View),
                HeaderOutput::Edit => AppMsg::SetMode(AppMode::Edit),
                HeaderOutput::Export => AppMsg::SetMode(AppMode::Export),
            });

        let model = AppModel{
            mode: init,
            header, 
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
           AppMsg::SetMode(mode)  => {
               self.mode = mode;
           },
        }
    }
}


fn main() {
    let relm = RelmApp::new("rafa.test.component");
    relm.run::<AppModel>(AppMode::View);
}
