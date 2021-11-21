use sdl2::event::Event;

//TODO: extern SDL_Window* underlyingWindow;
pub const MAX_MESSAGE_PROCESSING: isize = 100;

#[derive(Clone,Debug,PartialOrd, PartialEq)]
pub struct EventHandler {

}

impl EventHandler {
    pub fn new() -> Self {
        Self {}
    }
}

pub enum EventType {
    // triggered when the app gains or loses focus
    ActivateEvent,
    // triggered when the app's window is hidden or exposed
    WindowHiddenEvent,
    // triggered when the app's window is exposed
    WindowExposedEvent,
    // triggered when the application's window is minimized
    WindowMinimizedEvent,
    // triggered when the app's window is maximized
    WindowMaximizedEvent,
    // triggered when the app's window is restored to its original size and location
    WindowRestoredEvent,
    // triggered when the window is resized,
    WindowResizeEvent,
    // triggered when the mouse enters the application's window
    WindowMouseEnterEvent,
    // triggered when the mouse leaves the application's window
    WindowMouseLeaveEvent,
    JoystickAxisMotionEvent,
    JoystickBallMotionEvent,
    JoystickButtonUpEvent,
    JoystickButtonDownEvent,
    JoystickHatMotionEvent,
    KeyDownEvent,
    KeyUpEvent,
    TextInputEvent,
    MouseButtonDownEvent,
    MouseButtonUpEvent,
    MouseDoubleClickEvent,
    MouseMotionEvent,
    MouseWheelEvent,
    QuitEvent,
}

/**
 * Updates the event pump and calls any associated callbacks.
 */
pub fn pump()
{
    let event: Event;
    let count: i64 = 0;

    while ((SDL_PollEvent(&event) != 0) && (count < MAX_MESSAGE_PROCESSING))
    {
        match event {
            Event::MouseMotion => {
                // mMouseMotionEvent(event.motion.x, event.motion.y, event.motion.xrel, event.motion.yrel);
            },
            Event::KeyDown => {
                // mKeyDownEvent(static_cast<KeyCode>(event.key.keysym.sym), static_cast<KeyModifier>(event.key.keysym.mod), event.key.repeat != 0 ? true : false);
            },
            Event::KeyUp => {
                // mKeyUpEvent(static_cast<KeyCode>(event.key.keysym.sym), static_cast<KeyModifier>(event.key.keysym.mod));
            break;
            }


        }
        switch (event.type)
        {



        case SDL_KEYUP:
            mKeyUpEvent(static_cast<KeyCode>(event.key.keysym.sym), static_cast<KeyModifier>(event.key.keysym.mod));
            break;

        case SDL_TEXTINPUT:
            mTextInput(event.text.text);
            break;

        case SDL_MOUSEBUTTONDOWN:
            if (event.button.clicks == 2)
            {
                mMouseDoubleClick(static_cast<MouseButton>(event.button.button), event.button.x, event.button.y);
            }

            mMouseButtonDownEvent(static_cast<MouseButton>(event.button.button), event.button.x, event.button.y);
            break;

        case SDL_MOUSEBUTTONUP:
            mMouseButtonUpEvent(static_cast<MouseButton>(event.button.button), event.button.x, event.button.y);
            break;

        case SDL_MOUSEWHEEL:
            mMouseWheelEvent(event.wheel.x, event.wheel.y);
            break;

        case SDL_JOYAXISMOTION:
            mJoystickAxisMotionEvent(event.jaxis.which, event.jaxis.axis, event.jaxis.value);
            break;

        case SDL_JOYBALLMOTION:
            mJoystickBallMotionEvent(event.jball.which, event.jball.ball, event.jball.xrel, event.jball.yrel);
            break;

        case SDL_JOYHATMOTION:
            mJoystickHatMotionEvent(event.jhat.which, event.jhat.hat, event.jhat.value);
            break;

        case SDL_JOYBUTTONDOWN:
            mJoystickButtonDownEvent(event.jbutton.which, event.jbutton.button);
            break;

        case SDL_JOYBUTTONUP:
            mJoystickButtonUpEvent(event.jbutton.which, event.jbutton.button);
            break;

        case SDL_WINDOWEVENT:
            // Not completely happy with this but meh, it works.
            if (event.window.event == SDL_WINDOWEVENT_FOCUS_GAINED) { mActivateEvent(true); }
            else if (event.window.event == SDL_WINDOWEVENT_FOCUS_LOST) { mActivateEvent(false); }
            else if (event.window.event == SDL_WINDOWEVENT_SHOWN) { mWindowHiddenEvent(false); }
            else if (event.window.event == SDL_WINDOWEVENT_HIDDEN) { mWindowHiddenEvent(true); }
            else if (event.window.event == SDL_WINDOWEVENT_EXPOSED) { mWindowExposedEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_MINIMIZED) { mWindowMinimizedEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_MAXIMIZED) { mWindowMaximizedEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_RESTORED) { mWindowRestoredEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_ENTER) { mWindowMouseEnterEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_LEAVE) { mWindowMouseLeaveEvent(); }
            else if (event.window.event == SDL_WINDOWEVENT_RESIZED) { mWindowResizedEvent({event.window.data1, event.window.data2}); }
            break;

        case SDL_QUIT:
            mQuitEvent();
            break;

        default:
            // Ignore any cases not handled.
            break;
        }
        count++;
    }
}


/**
 * Turns on/off text input mode.
 *
 * \param _b Boolean value. True to turn on text input, false to turn it off.
 */
void EventHandler::textInputMode(bool _b)
{
    if (_b) { SDL_StartTextInput(); }
    else { SDL_StopTextInput(); }
}


/**
 * Queries text input mode.
 */
bool EventHandler::textInputMode()
{
    return SDL_IsTextInputActive() == SDL_TRUE;
}


/**
 * Decodes a KeyModifier and determines if the Shift keymod is applied.
 *
 * \param mod	Modifier value to decode.
 */
bool EventHandler::shift(KeyModifier mod) const
{
    return KeyModifier::None != (mod & (KeyModifier::Shift | KeyModifier::Caps));
}


/**
 * Decodes a KeyModifier and determines if the Shift keymod is applied.
 *
 * \param mod	Modifier value to decode.
 */
bool EventHandler::alt(KeyModifier mod) const
{
    return KeyModifier::None != (mod & KeyModifier::Alt);
}


/**
 * Decodes a KeyModifier and determines if the Numlock keymod is applied.
 *
 * \param mod	Modifier value to decode.
 */
bool EventHandler::numlock(KeyModifier mod) const
{
    return KeyModifier::None != (mod & KeyModifier::Num);
}


/**
 * Decodes a KeyModifier and determines if the Control keymod is applied.
 *
 * \param mod	Modifier value to decode.
 */
bool EventHandler::control(KeyModifier mod) const
{
    return KeyModifier::None != (mod & KeyModifier::Ctrl);
}


/**
 * Queries state of the Shift key modifier.
 */
bool EventHandler::query_shift() const
{
    using underlying = std::underlying_type_t<KeyModifier>;
    return KeyModifier::None != static_cast<KeyModifier>(SDL_GetModState() & static_cast<underlying>(KeyModifier::Shift));
}


/**
 * Queries state of the Shift key modifier.
 */
bool EventHandler::query_numlock() const
{
    using underlying = std::underlying_type_t<KeyModifier>;
    return KeyModifier::None != static_cast<KeyModifier>(SDL_GetModState() & static_cast<underlying>(KeyModifier::Num));
}


/**
 * Queries state of the Shift key modifier.
 */
bool EventHandler::query_control() const
{
    using underlying = std::underlying_type_t<KeyModifier>;
    return KeyModifier::None != static_cast<KeyModifier>(SDL_GetModState() & static_cast<underlying>(KeyModifier::Ctrl));
}


/**
 * Disconnects all connected signal handlers.
 */
void EventHandler::disconnectAll()
{
    mActivateEvent.clear();
    mJoystickAxisMotionEvent.clear();
    mJoystickBallMotionEvent.clear();
    mJoystickButtonUpEvent.clear();
    mJoystickButtonDownEvent.clear();
    mJoystickHatMotionEvent.clear();
    mKeyUpEvent.clear();
    mKeyDownEvent.clear();
    mTextInput.clear();
    mMouseButtonUpEvent.clear();
    mMouseButtonDownEvent.clear();
    mMouseMotionEvent.clear();
    mMouseWheelEvent.clear();
    mQuitEvent.clear();
}


namespace NAS2D
{
    /**
     * Posts a quit event to the event system.
     */
    void postQuitEvent()
    {
        SDL_Event event;
        event.type = SDL_QUIT;
        SDL_PushEvent(&event);
    }
}
