use glam::DVec3;
use rt::{ camera::Camera, material::Material, shapes::sphere::Sphere, shapes::cylinder::Cylinder };
use std::cell::RefCell;
use std::io;
use std::rc::Rc;
use rt::hittable::Hittable;
extern crate gtk;
use gtk::prelude::*;
use gtk::{Window, WindowType, Button, Entry, Box as GtkBox, Orientation, ComboBoxText, Separator};


struct GuiData {
    object_type: String,
    object_position: (f64, f64, f64),
    object_radius: f64,
    object_material: Material,
    camera_position: (f64, f64, f64),
    camera_look_at: (f64, f64, f64),
    image_width: u32,
    aspect_ratio: f64,
}
fn create_world_from_gui_data(gui_data: &GuiData) -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    // For now, adding one object based on GUI data
    world.push(
        Box::new(Sphere {
            center: DVec3::new(gui_data.object_position.0, gui_data.object_position.1, gui_data.object_position.2),
            radius: gui_data.object_radius,
            material: gui_data.object_material.clone(),
        })
    );

    // TODO: Add more objects based on GUI data

    world
}

fn create_camera_from_gui_data(gui_data: &GuiData) -> Camera {
    Camera::init()
        .image_width(gui_data.image_width)
        .aspect_ratio(gui_data.aspect_ratio)
        .look_from(DVec3::new(gui_data.camera_position.0, gui_data.camera_position.1, gui_data.camera_position.2))
        .look_at(DVec3::new(gui_data.camera_look_at.0, gui_data.camera_look_at.1, gui_data.camera_look_at.2))
        // ... other camera configurations
        .build()
}

fn create_object_config() -> (GtkBox, Rc<RefCell<Entry>>, Rc<RefCell<Entry>>, Rc<RefCell<Entry>>, Rc<RefCell<Entry>>, Rc<RefCell<ComboBoxText>>) {
    
    let hbox = GtkBox::new(Orientation::Horizontal, 5);

    // Wrap the GUI components in Rc<RefCell<T>> for shared mutable state
    let object_selector = Rc::new(RefCell::new(ComboBoxText::new()));
    object_selector.borrow_mut().append_text("Sphere");
    object_selector.borrow_mut().append_text("Cube");
    object_selector.borrow_mut().set_active(Some(0));
    let object_selector = object_selector.clone();

    let pos_x_entry = Rc::new(RefCell::new(Entry::new()));
    pos_x_entry.borrow_mut().set_placeholder_text(Some("X Position"));
    pos_x_entry.borrow_mut().set_text("0.0");
    pos_x_entry.borrow_mut().set_width_chars(10);
    pos_x_entry.borrow_mut().set_max_length(10);
    pos_x_entry.borrow_mut().set_activates_default(true);
    let pos_x_entry = pos_x_entry.clone();

    let pos_y_entry = Rc::new(RefCell::new(Entry::new()));
    pos_y_entry.borrow_mut().set_placeholder_text(Some("Y Position"));
    let pos_y_entry = pos_y_entry.clone();

    let pos_z_entry = Rc::new(RefCell::new(Entry::new())); 
    pos_z_entry.borrow_mut().set_placeholder_text(Some("Z Position"));
    let pos_z_entry = pos_z_entry.clone();
    
    // Connect the changed signal to the Entry to print its content when it changes (more logging)
    pos_x_entry.borrow().connect_changed(|entry| {
        let text = entry.get_text().to_string();
        println!("X Entry changed (fn create_object_config): {}", text);
    });
    

    let radius_entry = Rc::new(RefCell::new(Entry::new()));
    let radius_entry = radius_entry.clone();
    radius_entry.borrow().set_placeholder_text(Some("Radius"));
    
    
    let material_selector = Rc::new(RefCell::new(ComboBoxText::new()));
    let material_selector = material_selector.clone();
    material_selector.borrow().append_text("Dielectric");
    material_selector.borrow().append_text("Lambertian");
    material_selector.borrow().append_text("Metal");
    material_selector.borrow().set_active(Some(0));

    let delete_button = Button::with_label("Delete");
    delete_button.connect_clicked(move |btn| {
        let parent = btn.get_parent().unwrap();
        unsafe { parent.destroy(); }
    });

    hbox.pack_start(&*object_selector.borrow(), false, false, 0);
    hbox.pack_start(&*pos_x_entry.borrow(), false, false, 0);
    hbox.pack_start(&*pos_y_entry.borrow(), false, false, 0);
    hbox.pack_start(&*pos_z_entry.borrow(), false, false, 0); 
    hbox.pack_start(&*radius_entry.borrow(), false, false, 0);
    hbox.pack_start(&*material_selector.borrow(), false, false, 0);
    hbox.pack_start(&delete_button, false, false, 0);

     // Return all the required elements
     (hbox, pos_x_entry, pos_y_entry, pos_z_entry, radius_entry, material_selector)

    

}



fn launch_gui() {
    gtk::init().expect("Failed to initialize GTK.");

    let object_selector = Rc::new(RefCell::new(ComboBoxText::new()));
    let pos_x_entry = Rc::new(RefCell::new(Entry::new()));
    let pos_y_entry = Rc::new(RefCell::new(Entry::new()));
    let pos_z_entry = Rc::new(RefCell::new(Entry::new())); // Added Z position entry
    let radius_entry = Rc::new(RefCell::new(Entry::new()));
    let material_selector = Rc::new(RefCell::new(ComboBoxText::new()));

    pos_x_entry.borrow().connect_changed(|entry| {
        let text = entry.get_text().to_string();
        println!("X Entry changed (fn launch_gui): {}", text);
    });

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Ray Tracing Settings");
    window.set_default_size(600, 600);

    let vbox = GtkBox::new(Orientation::Vertical, 10);
    let vbox_clone = vbox.clone();

    let render_button = Button::with_label("Render picture");
    vbox.pack_start(&render_button, false, false, 0);

    let add_object_button = Button::with_label("Add Object");
    vbox.pack_start(&add_object_button, false, false, 0);

    




    let object_selector_clone = object_selector.clone();
    let pos_x_entry_clone = pos_x_entry.clone();
    let pos_y_entry_clone = pos_y_entry.clone();
    let pos_z_entry_clone = pos_z_entry.clone();

    pos_x_entry_clone.borrow().connect_changed(|entry| {
        let text = entry.get_text().to_string();
        println!("X Entry changed : {}", text);
    });
    
    let pos_x_entry = Rc::new(RefCell::new(Entry::new()));


    add_object_button.connect_clicked(move |_| {
        // Create a new object configuration section
        let (object_config, local_pos_x_entry, pos_y_entry, pos_z_entry, radius_entry, material_selector) = create_object_config();
        vbox_clone.pack_start(&object_config, false, false, 10);
    
        object_selector_clone.borrow_mut().append_text("Sphere");
        pos_x_entry.borrow_mut().set_placeholder_text(Some("X Position"));
        pos_y_entry.borrow_mut().set_placeholder_text(Some("Y Position"));
        pos_z_entry.borrow_mut().set_placeholder_text(Some("Z Position"));
        radius_entry.borrow_mut().set_placeholder_text(Some("Radius"));
        material_selector.borrow_mut().set_active(Some(0));
        
        // Ensure GUI updates have been processed before reading the value
        while gtk::events_pending() {
            gtk::main_iteration();
        }
    
        println!("x text: {:?}", pos_x_entry.borrow().get_text());
        
        vbox_clone.show_all();
    });

    // Separator
    let separator = Separator::new(Orientation::Horizontal);
    vbox.pack_start(&separator, false, false, 10);

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
    

    
let object_selector = object_selector.clone();
let pos_x_entry =  pos_x_entry.clone();
let pos_y_entry = pos_y_entry.clone();
let pos_z_entry = pos_z_entry.clone();
let radius_entry = radius_entry.clone();
let material_selector = material_selector.clone();

// Connect the changed signal to the Entry to print its content when it changes
pos_x_entry.borrow().connect_changed(|entry| {
    let text = entry.get_text().to_string();
    println!("X Entry changed 5: {}", text);
});
// Function to validate X position
fn validate_x_position(x_position_entry: &gtk::Entry) -> Result<f64, String> {
    let x_text = x_position_entry.get_text().trim().to_string();
    if x_text.is_empty() {
        return Err("X position cannot be empty.".into());
    }

    match x_text.parse::<f64>() {
        Ok(num) => Ok(num),
        Err(_) => Err("Invalid X position entered.".into()),
    }
}

//Button to render the scene with the given parameters from the GUI on click 
render_button.connect_clicked(move |_| {

    // Validate and parse X position (testing)
    let x_text = pos_x_entry.borrow().get_text().to_string();
    println!("Raw X position: {:?}", x_text);

    match validate_x_position(&pos_x_entry.borrow()) {
        Ok(x_position) => {
            // Proceed with rendering since X position is valid
            println!("Validated X position: {}", x_position);
            // ... rest of your rendering code using x_position
        },
        Err(error_message) => {
            // Handle the error, e.g., display a message to the user
            println!("Error validating X position: {}", error_message);
            eprintln!("{}", error_message);
            // ... perhaps highlight the input field or show a dialog with the error
        
        },
    }

    // Log the raw values from the GUI
    let raw_x_position = pos_x_entry.clone().borrow().get_text();
    // let raw_y_position = pos_y_entry.borrow().get_text();
    // let raw_z_position = pos_z_entry.borrow().get_text();
    // let raw_radius = radius_entry.borrow().get_text();

    println!("Raw X position: {:?}", raw_x_position);
    // println!("Raw Y position: {:?}", raw_y_position);
    // println!("Raw Z position: {:?}", raw_z_position);
    // println!("Raw Radius: {:?}", raw_radius);

// testing validation and parsing with proper error handling for pos_X_entry
    // Validate and parse X position
    let object_position_x = match raw_x_position.trim().parse::<f64>() {
        Ok(num) => {
            println!("Parsed X position: {}", num);
            num
        },
        Err(e) => {
            eprintln!("Error parsing X position: {}", e);
            1.0 // Default value if parsing fails
        },
    };

        let gui_data = GuiData {
            object_type: object_selector.borrow().get_active_text().map_or_else(|| "".to_string(), |gstr| gstr.to_string()),

            object_position: (
                object_position_x,
                pos_y_entry.borrow().get_text().parse().unwrap_or_else(|_| { println!("Error parsing Y position"); 0.0 }),
                pos_z_entry.borrow().get_text().parse().unwrap_or_else(|_| { println!("Error parsing Z position"); 0.0 })

            ),
            object_radius: radius_entry.borrow().get_text().parse().unwrap_or_else(|_| { println!("Error parsing radius"); 1.0 }),
            object_material: match material_selector.borrow().get_active_text().map_or_else(|| "".to_string(), |gstr| gstr.to_string()).as_str() {
                "Dielectric" => Material::Dielectric { index_of_refraction: 1.5 }, // Adjust as needed
                "Lambertian" => Material::Lambertian { albedo: DVec3::new(0.5, 0.5, 0.5) }, // Adjust as needed
                "Metal" => Material::Metal { albedo: DVec3::new(0.7, 0.6, 0.5), fuzz: 0.0 }, // Adjust as needed
                _ => Material::Lambertian { albedo: DVec3::new(0.5, 0.5, 0.5) }, // Default
            },
            camera_position: (
                cam_x_entry.get_text().parse().unwrap_or(13.0),
                cam_y_entry.get_text().parse().unwrap_or(2.0),
                cam_angle_entry.get_text().parse().unwrap_or(3.0) // Adjust as needed
            ),
            camera_look_at: (0.0, 0.0, 0.0), // Adjust as needed
            image_width: width_entry.get_text().parse().unwrap_or(400),
            aspect_ratio: 16.0 / 9.0, // Adjust as needed
        };
    
        let world = create_world_from_gui_data(&gui_data);
        let camera = create_camera_from_gui_data(&gui_data);
    
        // Trigger the rendering logic:
        camera.render_to_disk(world).unwrap();
                    // Print the rendering information:
                    println!("Rendering with the following parameters:");
                    println!("Object Type: {:?}", gui_data.object_type);
                    println!("Object Position: {:?}", gui_data.object_position);
                    // println!("Object Radius: {:?}", gui_data.object_radius);
                    println!("Object Material: {:?}", gui_data.object_material);
                    println!("Image Width: {:?}", gui_data.image_width);
                    //println!("Raw X position2: {:?}", pos_x_entry.borrow().get_text());


    });
    

    window.add(&vbox);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}


fn main() -> io::Result<()> {
    // Launch the GUI
    launch_gui();

    // The rendering logic will be triggered from the GUI
    Ok(())
}

