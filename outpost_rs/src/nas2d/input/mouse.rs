use sdl2::raw_window_handle::SDL_bool::{SDL_TRUE, SDL_FALSE};
use sdl2_sys::{SDL_SetWindowGrab, SDL_WarpMouseInWindow, SDL_SetRelativeMouseMode};

pub fn grab_mouse(underlying_window: &mut SDL_Window) {
    SDL_SetWindowGrab(underlying_window, SDL_TRUE)
}

pub fn release_mouse(underlying_window: &mut SDL_Window) {
    SDL_SetWindowGrab(underlying_window, SDL_FALSE)
}

pub fn warp_mouse(underlying_window: &mut SDL_Window, x: i64, y: i64) {
    if underlying_window {
        SDL_WarpMouseInWindow(underlying_window, x, y);
        // TODO
        // mMouseMotionEvent.emit(x, y, 0, 0);
    }
}

pub fn mouse_relative_mode(rel: bool) {
    if rel {
        SDL_SetRelativeMouseMode(SDL_TRUE)
    } else {
        SDL_SetRelativeMouseMode(SDL_FALSE)
    }
}



