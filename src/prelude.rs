//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```
extern crate stdweb;

pub use html::{
    ChangeData, Component, ComponentLink, Href, Html, InputData, Renderable, ShouldRender,
};

pub use app::App;

pub use callback::Callback;

pub use stdweb::web::event::{
    BlurEvent, ClickEvent, DoubleClickEvent, FocusEvent, IKeyboardEvent, IMouseEvent, KeyDownEvent,
    KeyPressEvent, KeyUpEvent, MouseDownEvent, MouseEnterEvent, MouseLeaveEvent, MouseMoveEvent,
    MouseOutEvent, MouseOverEvent, MouseUpEvent, MouseWheelEvent, SubmitEvent,
};
