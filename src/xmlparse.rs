#[macro_use]
extern crate log;
extern crate env_logger;

extern crate xml;
extern crate bml;

extern crate typed_arena;

use std::env;
use std::fs::File;

use xml::reader::EventReader;
use xml::reader::events::*;
use bml::bml::{BulletMLData, Node};

fn indent(size: usize) -> String {
    let indent: &'static str = "    ";
    (0..size).map(|_| indent)
             .fold(String::with_capacity(size*indent.len()), |r, s| r + s)
}

fn main() {
    env_logger::init().unwrap();

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    let args: Vec<String> = env::args().collect();
    let file = File::open(args[1].clone()).unwrap();

    let mut parser = EventReader::new(file);
    let mut depth = 0;

    let mut curr_tag = "".to_string();
    let mut curr_label = "".to_string();
    let mut curr_type = "".to_string();

    let arena = typed_arena::Arena::new();
    let root = arena.alloc(Node::new(BulletMLData::new_cell("bulletml")));

    for e in parser.events() {
        match e {
            XmlEvent::StartElement { name, attributes, .. } => {

                match name.local_name.as_ref() {
                    "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" => {

                        curr_tag = name.local_name.to_string();

                        println!("{}+{} {:?}", indent(depth), name.local_name, attributes);
                        depth += 1;

                        match attributes.is_empty() {
                            true => {}
                            false =>  {
                                match attributes[0].name.local_name.as_ref() {
                                    "label" =>  {
                                        curr_label = attributes[0].value.to_string();
                                        curr_type = "none".to_string();
                                    },

                                    "type" => { curr_type = attributes[0].value.to_string();
                                                curr_label = "none".to_string();},
                                    _ => {}
                                }
                            }
                        }

                        let curr_node = arena.alloc(Node::new(BulletMLData::new_cell(&curr_tag)));
                        curr_node.data.borrow_mut().set_type(&curr_type);
                        curr_node.data.borrow_mut().set_label(&curr_label);
                        root.insert(curr_node, depth as i32);

                        curr_tag = "".to_string();
                        curr_label = "".to_string();
                        curr_type = "".to_string();
                    }

                    _ => {}
                }
            }

            XmlEvent::EndElement { name } => {

                match name.local_name.as_ref() {
                    "bulletRef" | "actionRef" | "fireRef"
                    | "bullet" | "fire" | "action"
                    | "changeDirection" | "changeSpeed"
                    | "accel" | "wait" | "vanish" | "repeat"
                    | "direction" | "speed"
                    | "horizontal" | "vertical"
                    | "term" | "param" =>
                        {
                            depth -= 1;
                            println!("{}-{}", indent(depth), &name.local_name);
                        }

                    _ => {}
                }
            }

            XmlEvent::Characters(s) => {
                println!("{} {}", indent(depth), s);
            }

            XmlEvent::Error(e) => {
                println!("Error: {}", e);
                break;
            }

            _ => {}
        }
    }

    println!("{:?}", root);
}
