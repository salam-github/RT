use glib::clone;
use glib::signal::Inhibit;
use gtk::prelude::*;
use gtk::{Box as GtkBox, Button, Entry, Orientation, Separator, Window, WindowType, Scale};
use std::cell::RefCell;
use std::rc::Rc;

struct AppState {
    object_configs: Vec<ObjectConfig>,
}

struct ObjectConfig {
    pos_x_entry: Rc<RefCell<Entry>>,
    pos_y_entry: Rc<RefCell<Entry>>,
    pos_z_entry: Rc<RefCell<Entry>>,
    radius_entry: Rc<RefCell<Entry>>,
    // material_selector: Rc<RefCell<ComboBoxText>>,
}

// TODO:Remove dead_code
#[allow(dead_code)]
struct GuiData {
    // object_type: String,
    object_position: (f64, f64, f64),
    object_radius: f64,
    // object_material: Material,
    camera_position: (f64, f64, f64),
    camera_look_at: (f64, f64, f64),
    image_width: u32,
    aspect_ratio: f64,
}



pub fn launch_gui() {
    let app_state = Rc::new(RefCell::new(AppState {
        object_configs: Vec::new(),
    }));

    gtk::init().expect("Failed to initialize GTK.");

    // let object_selector = Rc::new(RefCell::new(ComboBoxText::new()));
    let pos_x_entry = Rc::new(RefCell::new(Entry::new()));
    let pos_y_entry = Rc::new(RefCell::new(Entry::new()));
    let pos_z_entry = Rc::new(RefCell::new(Entry::new())); // Added Z position entry
    let radius_entry = Rc::new(RefCell::new(Entry::new()));
    // let material_selector = Rc::new(RefCell::new(ComboBoxText::new()));

    pos_x_entry.borrow().connect_changed(|entry| {
        let text = entry.get_text().to_string();
        println!("X Entry changed (fn launch_gui): {}", text);
    });

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ray Tracing Settings");
    window.set_default_size(600, 600);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    let vbox_clone = vbox.clone();
    
    let vbox = GtkBox::new(Orientation::Vertical, 10);

    let render_button = Button::with_label("Render picture");
    vbox.pack_start(&render_button, false, false, 0);
    
    // Create a horizontal box for the side-by-side buttons
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    
    let add_sphere_button = Button::with_label("Add Sphere");
    hbox.pack_start(&add_sphere_button, false, false, 0);
    
    let add_cylinder_button = Button::with_label("Add Cylinder");
    hbox.pack_start(&add_cylinder_button, false, false, 0);
    
    // Add the horizontal box to the vertical box
    vbox.pack_start(&hbox, false, false, 0);

    // let pos_x_entry = Rc::new(RefCell::new(Entry::new()));

    add_sphere_button.connect_clicked(clone!(@strong app_state => move |_| {

    app_state.borrow_mut().object_configs.push(ObjectConfig {
        pos_x_entry: pos_x_entry.clone(),
        pos_y_entry: pos_y_entry.clone(),
        pos_z_entry: pos_z_entry.clone(),
        radius_entry: radius_entry.clone(),
        // material_selector: material_selector.clone(),
    });
    


        // Ensure GUI updates have been processed before reading the value
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        println!("x text: {:?}", pos_x_entry.borrow().get_text());

        vbox_clone.show_all();
    }));

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 10);

    let brightness_label = gtk::Label::new(Some("Brightness"));
    vbox.pack_start(&brightness_label, false, false, 0);
    let brightness_entry = Scale::with_range(Orientation::Horizontal, 0.0, 1.0, 0.1);
    vbox.pack_start(&brightness_entry, false, false, 0);

    // Camera Options
    let camera_label = gtk::Label::new(Some("Camera Options"));
    vbox.pack_start(&camera_label, false, false, 0);

    let cam_x_entry = Entry::new();
    cam_x_entry.set_placeholder_text(Some("Camera X Position"));
    vbox.pack_start(&cam_x_entry, false, false, 0);

    let cam_y_entry = Entry::new();
    cam_y_entry.set_placeholder_text(Some("Camera Y Position"));
    vbox.pack_start(&cam_y_entry, false, false, 0);

    let cam_angle_entry = Entry::new();
    cam_angle_entry.set_placeholder_text(Some("Camera Angle"));
    vbox.pack_start(&cam_angle_entry, false, false, 0);

    // Resolution Selection
    let resolution_label = gtk::Label::new(Some("Resolution"));
    vbox.pack_start(&resolution_label, false, false, 0);

    let width_entry = Entry::new();
    width_entry.set_placeholder_text(Some("Width"));
    vbox.pack_start(&width_entry, false, false, 0);

    let height_entry = Entry::new();
    height_entry.set_placeholder_text(Some("Height"));
    vbox.pack_start(&height_entry, false, false, 0);


    //Button to render the scene with the given parameters from the GUI on click
    render_button.connect_clicked(clone!(@strong app_state => move |_| {

        // Iterate over all stored object configurations
        for object_config in &app_state.borrow().object_configs {
            // Read values from each object configuration
            let pos_x = validate_and_parse_entry(&object_config.pos_x_entry.borrow(), 0.0, "X position");
            let pos_y = validate_and_parse_entry(&object_config.pos_y_entry.borrow(), 0.0, "Y position");
            let pos_z = validate_and_parse_entry(&object_config.pos_z_entry.borrow(), 0.0, "Z position");
            let radius = validate_and_parse_entry(&object_config.radius_entry.borrow(), 1.0, "Radius");

            // Create GuiData for the current object
            let gui_data = GuiData {
                object_position: (pos_x, pos_y, pos_z),
                object_radius: radius,
            camera_position: (
                cam_x_entry.get_text().parse().unwrap_or(13.0),
                cam_y_entry.get_text().parse().unwrap_or(2.0),
                cam_angle_entry.get_text().parse().unwrap_or(3.0), // Adjust as needed
            ),
            camera_look_at: (0.0, 0.0, 0.0), // Adjust as needed
            image_width: width_entry.get_text().parse().unwrap_or(400),
            aspect_ratio: 16.0 / 9.0, // Adjust as needed
        };

        // Log the rendering information
        println!("Rendering with the following parameters:");
        // println!("Object Type: {:?}", gui_data.object_type);
        println!("Object Position: {:?}", gui_data.object_position);
        println!("Object Radius: {:?}", gui_data.object_radius);
        // println!("Object Material: {:?}", gui_data.object_material);
        println!("Image Width: {:?}", gui_data.image_width);
        //println!("Raw X position2: {:?}", pos_x_entry.borrow().get_text());
    }
    }));

    window.add(&vbox);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}

// // Function to validate and parse position entries
fn validate_and_parse_entry(entry: &gtk::Entry, default_value: f64, label: &str) -> f64 {
    let text = entry.get_text().trim().to_string();
    match text.parse::<f64>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!(
                "Error parsing {}: using default value {}",
                label, default_value
            );
            default_value // Use the default value if parsing fails
        }
    }
}
