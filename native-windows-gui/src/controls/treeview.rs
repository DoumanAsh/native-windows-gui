use winapi::shared::minwindef::LPARAM;
use winapi::um::commctrl::HTREEITEM;
use crate::win32::window_helper as wh;
use crate::win32::base_helper::{to_utf16};
use crate::Font;
use super::ControlHandle;
use std::{mem, ptr};

const NOT_BOUND: &'static str = "TreeView is not yet bound to a winapi object";
const BAD_HANDLE: &'static str = "INTERNAL ERROR: TreeView handle is not HWND!";


/// Select the position of a new item that is about to be inserted in a TreeView
#[derive(Copy, Clone, Debug)]
pub enum TreeInsert {
    /// Inserts the item at the beginning of the list. 
    First,

    /// Inserts the item at the end of the list. 
    Last,

    /// Add the item as a root item 
    Root,

    /// Inserts the item into the list in alphabetical order
    Sort,

    /// Insert the item after the choosen item
    After(HTREEITEM)
}

/// A reference to an item in a TreeView
#[derive(Copy, Clone)]
pub struct TreeItem {
    pub handle: HTREEITEM
}

impl TreeItem {

}


/**
A tree-view control is a window that displays a hierarchical list of items
*/
#[derive(Default, Debug)]
pub struct TreeView {
    pub handle: ControlHandle
} 


impl TreeView {


    /// Insert a new item into the TreeView and return a reference to new newly added item
    pub fn insert_item<'a>(&self, new: &'a str, parent: Option<TreeItem>, position: TreeInsert) -> TreeItem {
        use winapi::um::commctrl::{TVM_INSERTITEMW, TVINSERTSTRUCTW, TVI_FIRST, TVI_LAST, TVI_ROOT, TVI_SORT, TVIF_TEXT};
        use winapi::um::commctrl::TVINSERTSTRUCTW_u;
        use winapi::um::winnt::LPWSTR;

        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let insert = match position {
            TreeInsert::First => TVI_FIRST,
            TreeInsert::Last => TVI_LAST,
            TreeInsert::Root => TVI_ROOT,
            TreeInsert::Sort => TVI_SORT,
            TreeInsert::After(i) => i
        };

        let text = to_utf16(new);

        let item = {
            let mut item: TVINSERTSTRUCTW_u = unsafe { mem::zeroed() };
            let i = unsafe { item.item_mut() };
            i.mask = TVIF_TEXT;
            i.pszText = text.as_ptr() as LPWSTR;
            item
        };

        let new_item = TVINSERTSTRUCTW {
            hParent: parent.map(|p| p.handle ).unwrap_or(ptr::null_mut()),
            hInsertAfter: insert,
            u: item
        };

        let ptr = &new_item as *const TVINSERTSTRUCTW;
        let handle = wh::send_message(handle, TVM_INSERTITEMW, 0, ptr as LPARAM) as HTREEITEM;

        TreeItem { handle }
    }


    //
    // Common methods
    //

    /// Return the font of the control
    pub fn font(&self) -> Option<Font> {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);

        let font_handle = wh::get_window_font(handle);
        if font_handle.is_null() {
            None
        } else {
            Some(Font { handle: font_handle })
        }
    }

    /// Set the font of the control
    pub fn set_font(&self, font: Option<&Font>) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_font(handle, font.map(|f| f.handle), true); }
    }

    /// Return true if the control currently has the keyboard focus
    pub fn focus(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_focus(handle) }
    }

    /// Set the keyboard focus on the button.
    pub fn set_focus(&self) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_focus(handle); }
    }

    /// Return true if the control user can interact with the control, return false otherwise
    pub fn enabled(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_enabled(handle) }
    }

    /// Enable or disable the control
    pub fn set_enabled(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_enabled(handle, v) }
    }

    /// Return true if the control is visible to the user. Will return true even if the 
    /// control is outside of the parent client view (ex: at the position (10000, 10000))
    pub fn visible(&self) -> bool {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_visibility(handle) }
    }

    /// Show or hide the control to the user
    pub fn set_visible(&self, v: bool) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_visibility(handle, v) }
    }

    /// Return the size of the button in the parent window
    pub fn size(&self) -> (u32, u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_size(handle) }
    }

    /// Set the size of the button in the parent window
    pub fn set_size(&self, x: u32, y: u32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_size(handle, x, y, false) }
    }

    /// Return the position of the button in the parent window
    pub fn position(&self) -> (i32, i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::get_window_position(handle) }
    }

    /// Set the position of the button in the parent window
    pub fn set_position(&self, x: i32, y: i32) {
        if self.handle.blank() { panic!(NOT_BOUND); }
        let handle = self.handle.hwnd().expect(BAD_HANDLE);
        unsafe { wh::set_window_position(handle, x, y) }
    }

    /// Winapi class name used during control creation
    pub fn class_name(&self) -> Option<&'static str> {
        use winapi::um::commctrl::WC_TREEVIEW;
        Some(WC_TREEVIEW)
    }

    /// Winapi base flags used during window creation
    pub fn flags(&self) -> u32 {
        use winapi::um::commctrl::{TVS_HASBUTTONS, TVS_LINESATROOT, TVS_HASLINES};
        use winapi::um::winuser::WS_VISIBLE;

        WS_VISIBLE | TVS_HASBUTTONS | TVS_LINESATROOT | TVS_HASLINES
    }

    /// Winapi flags required by the control
    pub fn forced_flags(&self) -> u32 {
        use winapi::um::winuser::{WS_CHILD, WS_BORDER};

        WS_CHILD | WS_BORDER
    }
}


impl PartialEq for TreeItem {
    fn eq(&self, other: &Self) -> bool {
        self.handle == other.handle
    }
}

impl Eq for TreeItem {}
