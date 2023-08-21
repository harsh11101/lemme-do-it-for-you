use rdev::listen;
use std::time::{Instant, Duration};
use std::thread;
use rusqlite::Connection;
use simulate::{self, Key, EventBuffer};
use std::collections::HashMap;

pub struct buffer_vector_value{
    buffer: EventBuffer,
    step: i32
}
pub struct delay_vector_value{
    delay: Duration,
    step: i32
}
#[derive(Clone, serde::Serialize)]
pub struct payload{
    variables: Vec<String>,
    length: usize,
    value: String,
}
pub fn check_trigger(input: &str)->Option<String>{
    let conn = Connection::open("../Data.db").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS triggervalue (trigger TEXT UNIQUE NOT NULL,value TEXT NOT NULL)",(),
    )
    .unwrap();
    let mut stmt = conn.prepare("SELECT value FROM triggervalue WHERE trigger=:trigg").unwrap();
    let data_iter = stmt
    .query_map(&[(":trigg", input.to_string().as_str())], |row| {
            Ok(row.get(0)?)
        })
        .unwrap();
    for data in data_iter {
        return Some(data.unwrap());
    }
    return None;
}
#[tauri::command]
pub fn run_backspace(length: usize)->(){
    for _ in 0..length{
        simulate::send(Key::Backspace).unwrap();
    }
}
pub fn parse_number<I>(iter: &mut I) -> Option<i32> where I: Iterator<Item = char> + Clone,{
    let mut num_str = String::new();

    // Clone the iterator to get a separate peekable iterator
    let mut peekable_iter = iter.clone().peekable();

    // Read characters until we find the closing brackets "]]"
    while let Some(c) = peekable_iter.next() {
        if c == ']' && peekable_iter.peek() == Some(&']') {
            peekable_iter.next();
            break;
        }
        num_str.push(c);
    }

    // Attempt to parse the number
    num_str.parse::<i32>().ok()
}
    pub fn run_buffer_vector(buffer_vector: &mut Vec<buffer_vector_value>, delay_vector: &mut Vec<delay_vector_value>)->(){
    let n = buffer_vector.len();
    let m = delay_vector.len();
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        if buffer_vector[i].step < delay_vector[j].step {
            let buffer = &mut buffer_vector[i].buffer;
            buffer.clone().simulate().unwrap();
            i += 1;
        }
        else {
            thread::sleep(delay_vector[j].delay);
            j += 1;
        }
    }
    while i < n {
        let buffer = &mut buffer_vector[i].buffer;
        buffer.clone().simulate().unwrap();
        i += 1;
    }
    while j < m {
        thread::sleep(delay_vector[j].delay);
        j += 1;
    }
}
#[tauri::command]
pub fn run_string(value: String)->(){
    let mut buffer_vector: Vec<buffer_vector_value>=Vec::new();
    let mut chars=value.chars().peekable();
    let mut str: String=String::new();
    let mut delay_vector: Vec<delay_vector_value>=Vec::new();
    let mut stp: i32=1;
    while let Some(c)=chars.next(){
        if c == '[' && chars.peek() == Some(&'[') {
            chars.next();
            parse_number(&mut chars);
            if let Some(num) = parse_number(&mut chars) {
                let sleep_duration = Duration::from_millis(num as u64);
                if !str.is_empty(){
                    buffer_vector.push(buffer_vector_value{
                        buffer: EventBuffer::new(),
                        step: stp
                    });
                    buffer_vector.last_mut().unwrap().buffer.type_str(str.as_str());
                    stp+=1;
                    str.clear();
                }
                delay_vector.push(delay_vector_value{
                    delay: sleep_duration,
                    step: stp
                });
                stp+=1;
                while let Some(c) = chars.next() {
                    if c == ']' && chars.peek() == Some(&']') {
                        chars.next();
                        break;
                    }
                }
            } 
            else {
                str.push('[');
                str.push('[');
            }
        }
        else if c==':' && chars.peek()==Some(&':'){
            buffer_vector.push(buffer_vector_value{
                buffer: EventBuffer::new(),
                step: stp
            });
            stp+=1;
            if !str.is_empty(){
                buffer_vector.last_mut().unwrap().buffer.type_str(str.as_str());
            }
            buffer_vector.last_mut().unwrap().buffer.send(Key::Enter);
            chars.next();
            str.clear();
        }
        else{
            str.push(c);
        }
    }
    if !str.is_empty(){
        buffer_vector.push(buffer_vector_value{
            buffer: EventBuffer::new(),
            step: stp
        });
        buffer_vector.last_mut().unwrap().buffer.type_str(str.as_str());
    }
    run_buffer_vector(&mut buffer_vector, &mut delay_vector);
}
pub fn pre_process_value(input: String)->Vec<String>{
    let mut extracted_strings = Vec::new();
    let mut string_count = HashMap::new();
    
    let start_delim = "{{";
    let end_delim = "}}";
    
    let mut current_index = 0;
    
    while let Some(start) = input[current_index..].find(start_delim) {
        let start_index = current_index + start;
        let end_index = start_index + start_delim.len();
        
        if let Some(end) = input[end_index..].find(end_delim) {
            let end_index = end_index + end;
            let extracted_string = &input[start_index + start_delim.len()..end_index];
            
            if !extracted_string.contains(start_delim) && !extracted_string.contains(end_delim) {
                if !string_count.contains_key(extracted_string) {
                    extracted_strings.push(extracted_string.to_string());
                    string_count.insert(extracted_string.to_string(), 1);
                }
            }
            
            current_index = end_index + end_delim.len();
        } else {
            break;
        }
    }
    extracted_strings
}
pub fn get_variable_values(input: String,length: usize,variables: Vec<String>,_app_handle: tauri::AppHandle)->(){
    let _docs_window = tauri::WindowBuilder::new(
        &_app_handle.clone(),
        "variables",
        tauri::WindowUrl::App("../../src/windows/variables/index.html".into()),
    )
    .title("Variables")
    .build()
    .unwrap();
    let data=payload{
        variables: variables,
        length: length,
        value: input
    };
    let docs_window_clone=_docs_window.clone();
    thread::spawn(move || {
        loop{
            docs_window_clone.emit("sending_data", data.clone()).unwrap();
        }
    });
    let id=_docs_window.clone().listen("for_id",|_event|{});
    let _docs_window_clone_forclosing=_docs_window.clone();
    _docs_window.clone().listen("close_window",move |event|{
        if let Err(e) = _docs_window_clone_forclosing.clone().close() {
            println!("Failed to close window: {:?}", e);
        }
        _docs_window_clone_forclosing.clone().unlisten(id);
    });
}
pub fn keyboard_listener(_app_handle: tauri::AppHandle)->(){
    let mut _input = String::new();
    let mut _now = Instant::now();
    let duration = Duration::from_millis(2000);
    if let Err(error) = listen(move |event| {
        match event.name {
            Some(ref key) => {
                if _input.len() == 0 {
                    _now = Instant::now();
                }
                else {
                    if _now.elapsed() >= duration {
                        _input.clear();
                        _now = Instant::now();
                    }
                    else {
                        _now = Instant::now();
                    }
                }
                let _str = key;
                _input.push_str(&_str);
                let value=check_trigger(&_input);
                match value{
                    Some(value)=>{
                        let length=_input.len();
                        thread::sleep(Duration::from_millis(500));
                        let variables: Vec<String>=pre_process_value(value.clone());
                        if variables.len()>0{
                            get_variable_values(value,length,variables,_app_handle.clone());
                        }
                        else{
                            run_backspace(length);
                            run_string(value);
                        }
                        _input.clear();
                        _now=Instant::now();
                    }
                    None=>{
                        _now=Instant::now();
                    }
                }
                if _now.elapsed() >= duration {
                    _input.clear();
                }
            }
            None => (),
        }
    }) {
        println!("Error: {:?}", error)
    }
}