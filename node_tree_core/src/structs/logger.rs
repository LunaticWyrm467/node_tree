//===================================================================================================================================================================================//
//
//  /$$                                                        
// | $$                                                        
// | $$        /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$   /$$$$$$ 
// | $$       /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$ /$$__  $$
// | $$      | $$  \ $$| $$  \ $$| $$  \ $$| $$$$$$$$| $$  \__/
// | $$      | $$  | $$| $$  | $$| $$  | $$| $$_____/| $$      
// | $$$$$$$$|  $$$$$$/|  $$$$$$$|  $$$$$$$|  $$$$$$$| $$      
// |________/ \______/  \____  $$ \____  $$ \_______/|__/      
//                      /$$  \ $$ /$$  \ $$                    
//                     |  $$$$$$/|  $$$$$$/                    
//                      \______/  \______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467 and others.
//? 
//? All code is licensed under the MIT license.
//? Feel free to reproduce, modify, and do whatever.
//?

//!
//! The Logger manages system-wide messages and the creation of logs, both regular and crash/panic
//! logs.
//! 

use std::time::SystemTime;

use chrono::{ DateTime, Utc };


use super::node_tree_base::NodeIdentity;
use crate::prelude::{ RID, NodeTreeBase };
use crate::utils::functions::draw_tree;


/*
 * Enum
 *      Types
 */


/// Used to dictate the logger's verbosity level.
#[derive(Debug, Clone)]
pub enum LoggerVerbosity {
    All,
    NoDebug,
    OnlyIssues,
    OnlyPanics
}


/// Used to pass the system that called the log to the logger for proper formatting.
#[derive(Debug, Clone)]
pub enum SystemCall {
    Named(String),
    NodePath(String)
}

impl SystemCall {

    /// Returns the calling system in a properly formatted string.
    pub fn format(&self) -> String {
        match self {
            Self::Named(str)    => str.to_string(),
            Self::NodePath(str) => format!("[{}]", str)
        }
    }

    /// Gets the underlying system without formating.
    pub fn to_str(&self) -> &str {
        match self {
            Self::Named(str)    => str,
            Self::NodePath(str) => str
        }
    }
}


/// Used to pass messages of certain types to the logger.
#[derive(Debug, Clone)]
pub enum Log<'a> {
    Debug(&'a str),
    Info(&'a str),
    Warn(&'a str),
    Panic(&'a str)
}

impl <'a >Log<'a> {
    
    /// Used to get the name associated to the Log's level.
    pub fn get_lv(&self) -> String {
        match self {
            Log::Debug(_) => "DEBUG".to_string(),
            Log::Info(_)  => "INFO".to_string(),
            Log::Warn(_)  => "WARN".to_string(),
            Log::Panic(_) => "PANIC!".to_string()
        }
    }

    /// Gets the message associated to the Log.
    pub fn get_msg(&self) -> &'a str {
        match self {
            Log::Debug(str) => str,
            Log::Info(str)  => str,
            Log::Warn(str)  => str,
            Log::Panic(str) => str
        }
    }

    /// Gets the colour code associated with the log's level.
    pub fn get_colour(&self) -> String {
        match self {
            Log::Debug(_) => "\u{001b}[30m".to_string(),   // Black/Dark Grey
            Log::Info(_)  => "\u{001b}[37m".to_string(),   // White
            Log::Warn(_)  => "\u{001b}[33m".to_string(),   // Yellow
            Log::Panic(_) => "\u{001b}[31m".to_string()    // Red
        }
    }

    /// Returns if this is a debug message.
    pub fn is_debug(&self) -> bool {
        match self {
            Log::Debug(_) => true,
            _             => false
        }
    }

    /// Returns if this is a log about some sort of issue, such as a warning or panic (crash).
    pub fn is_problematic(&self) -> bool {
        match self {
            Log::Warn(_) | Log::Panic(_) => true,
            _                            => false
        }
    }

    /// Returns if this is a panic (crash) log.
    pub fn is_panic(&self) -> bool {
        match self {
            Log::Panic(_) => true,
            _             => false
        }
    }
}


/*
 * Logger
 *      Struct
 */


#[derive(Debug, Clone)]
pub struct Logger {
    log:          String,
    verbosity_lv: LoggerVerbosity,
    crash_header: String,
    crash_footer: String
}

impl Logger {
    
    /// Creates a new Logger instance.
    pub fn new(verbosity_lv: LoggerVerbosity) -> Self {
        let mut logger: Logger = Logger {
            log:          String::new(),
            verbosity_lv,
            crash_header: "Unfortunately the program has crashed. Please contact the development team with the following crash report as well as the attachment of the log posted during the time of the crash.".to_string(),
            crash_footer: "Goodbye World! (Program Exited)".to_string()
        };
        
        logger.post_manual(SystemCall::Named("SysLogger".to_string()), Log::Debug("System logger has initialized. Hello World!"));
        logger
    }

    /// Sets the default crash header message.
    pub fn set_default_header_on_panic(&mut self, msg: &str) {
        self.crash_header = msg.to_string();
    }
    
    /// Sets the default crash footer message.
    pub fn set_default_footer_on_panic(&mut self, msg: &str) {
        self.crash_footer = msg.to_string();
    }

    /// Posts a new message to the log using the `NodeTreeBase` as a reference.
    /// This will return whether the NodeTree should quit or not.
    /// # Safety
    /// This is marked unsafe because there is no way to validate that the passed in pointer to the
    /// NodeTree is valid.
    pub unsafe fn post(&mut self, calling: RID, log: Log, node_tree: *mut NodeTreeBase) -> bool {
        match &self.verbosity_lv {
            LoggerVerbosity::All        => {},
            LoggerVerbosity::NoDebug    => if log.is_debug()        { return false; },
            LoggerVerbosity::OnlyIssues => if !log.is_problematic() { return false; },
            LoggerVerbosity::OnlyPanics => if !log.is_panic()       { return false; }
        }
        
        let node_tree: &NodeTreeBase = &*node_tree;
        let system:    SystemCall    = {
            match node_tree.get_node_identity(calling) {
                Some(NodeIdentity::NodePath)         => SystemCall::NodePath(unsafe { node_tree.get_node(calling).unwrap_unchecked() }.get_absolute_path().to_string()),
                Some(NodeIdentity::UniqueName(name)) => SystemCall::Named(name),
                None                                 => unimplemented!()
            }
        };

        let colour: String = log.get_colour();
        let panic:  bool   = log.is_panic();
        let time:   String = self.post_manual(system, log);

        if panic {
            let node_tree_visual: String = draw_tree(node_tree, calling, 6, 6);
            println!("
{}{}

\u{001b}[0m{}{}
Time of Crash: {}
Exit Code: {}

{}\u{001b}[0m", colour, self.crash_header, node_tree_visual, colour, time, 1, self.crash_footer);
            
            self.log += &format!("
{}

{}
Time of Crash: {}
Exit Code: {}

{}", self.crash_header, node_tree_visual, time, 1, self.crash_footer);
        }
        
        panic
    }

    /// Posts a new message to the log, without printing a crash report if there is an Error.
    /// Returns the time of the posted message
    pub fn post_manual(&mut self, system: SystemCall, log: Log) -> String {
        let time: String = DateTime::<Utc>::from(SystemTime::now()).format("%d/%m/%Y %T").to_string();
        match &self.verbosity_lv {
            LoggerVerbosity::All        => {},
            LoggerVerbosity::NoDebug    => if log.is_debug()        { return time; },
            LoggerVerbosity::OnlyIssues => if !log.is_problematic() { return time; },
            LoggerVerbosity::OnlyPanics => if !log.is_panic()       { return time; }
        }
        
        println!(
            "{}<{} UTC> | {} | {} | {}\u{001b}[0m",
            log.get_colour(),
            time,
            system.format(),
            log.get_lv(),
            log.get_msg()
        );
        
        self.log += &format!(
            "<{} UTC> | {} | {} | {}\n",
            time,
            system.format(),
            log.get_lv(),
            log.get_msg()
        );

        time
    }

    /// Gets the log as a string.
    pub fn to_str(&self) -> &str {
        &self.log
    }
}


/*
 * Logger
 *      Macros
 */


/// A simple macro which is compatible with Rust's format syntax used in macros like `print!`,
/// `println!`, and `format!`.
/// Prints debug info to the logger.
///
/// # Note
/// The first argument should be `self`.
#[macro_export]
macro_rules! debug {
    ($self:ident, $fmt_str:literal) => {{
        $self.post(Log::Debug(&format!($fmt_str)))
    }};

    ($self:ident, $fmt_str:literal, $($args:expr),*) => {{
        $self.post(Log::Debug(&format!($fmt_str, $($args),*)))
    }};
}

/// A simple macro which is compatible with Rust's format syntax used in macros like `print!`,
/// `println!`, and `format!`.
/// Prints info to the logger.
///
/// # Note
/// The first argument should be `self`.
#[macro_export]
macro_rules! info {
    ($self:ident, $fmt_str:literal) => {{
        $self.post(Log::Info(&format!($fmt_str)))
    }};

    ($self:ident, $fmt_str:literal, $($args:expr),*) => {{
        $self.post(Log::Info(&format!($fmt_str, $($args),*)))
    }};
}

/// A simple macro which is compatible with Rust's format syntax used in macros like `print!`,
/// `println!`, and `format!`.
/// Prints a warning to the logger.
///
/// # Note
/// The first argument should be `self`.
#[macro_export]
macro_rules! warn {
    ($self:ident, $fmt_str:literal) => {{
        $self.post(Log::Warn(&format!($fmt_str)))
    }};

    ($self:ident, $fmt_str:literal, $($args:expr),*) => {{
        $self.post(Log::Warn(&format!($fmt_str, $($args),*)))
    }};
}

/// A simple macro which is compatible with Rust's format syntax used in macros like `print!`,
/// `println!`, and `format!`.
/// Prints a panic to the logger and causes a crash.
///
/// # Note
/// The first argument should be `self`.
#[macro_export]
macro_rules! error {
    ($self:ident, $fmt_str:literal) => {{
        $self.post(Log::Panic(&format!($fmt_str)))
    }};

    ($self:ident, $fmt_str:literal, $($args:expr),*) => {{
        $self.post(Log::Panic(&format!($fmt_str, $($args),*)))
    }};
}
