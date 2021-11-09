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
}



/**
 * Joystic axis events generate three values:
 * Device ID, Axis ID and Axis Position.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int deviceId, int axisId, int pos);
 * \endcode
 *
 * \arg \c deviceId Joystick ID which this event was generated from.
 * \arg \c axisId Axis ID. Axis 0 is x, Axis 1 is Y.
 * Some joysticks use additional axis as buttons.
 * \arg \c pos Current position of the axis.
 */
EventHandler::JoystickAxisMotionEventSource& EventHandler::joystickAxisMotion()
{
	return mJoystickAxisMotionEvent;
}


/**
 * Joystick trackball events generate four values:
 * Device ID, Ball ID, X motion change and Y motion change.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int deviceId, int ballId, int xChange, int yChange);
 * \endcode
 *
 * \arg \c deviceId Joystick ID which this event was generated from.
 * \arg \c ballId Trackball ID.
 * \arg \c xChange Change in relative position of the X position.
 * \arg \c yChange Change in relative position of the Y position.
 */
EventHandler::JoystickBallMotionEventSource& EventHandler::joystickBallMotion()
{
	return mJoystickBallMotionEvent;
}


/**
 * Joystick button events generate two values:
 * Device ID, and Button ID
 * Button events occure when a joystick button is pressed or released.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int deviceId, int buttonId);
 * \endcode
 *
 * \arg \c deviceId	Joystick ID which this event was generated from.
 * \arg \c buttonId	Button ID which the event was generated from.
 */
EventHandler::JoystickButtonEventSource& EventHandler::joystickButtonUp()
{
	return mJoystickButtonUpEvent;
}


/**
 * Joystick button events generate two values:
 * Device ID, and Button ID
 * Button events occure when a joystick button is pressed or released.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int deviceId, int buttonId);
 * \endcode
 *
 * \arg \c deviceId	Joystick ID which this event was generated from.
 * \arg \c buttonId	Button ID which the event was generated from.
 */
EventHandler::JoystickButtonEventSource& EventHandler::joystickButtonDown()
{
	return mJoystickButtonDownEvent;
}


/**
 * Joystick hat motion events generate three values:
 * Device ID, Hat ID, and Hat Position.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int deviceId, int hatId, int pos);
 * \endcode
 *
 * \arg \c deviceId	Joystick ID which this event was generated from.
 * \arg \c hatId	Hat ID.
 * \arg \c pos		Current position of the hat.
 */
EventHandler::JoystickHatMotionEventSource& EventHandler::joystickHatMotion()
{
	return mJoystickHatMotionEvent;
}


/**
 * Keyboard key down events generate three values:
 * Keyboard scan code, key modifier, and a repeat flag.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(KeyCode key, KeyModifier mod, bool repeat);
 * \endcode
 *
 * \arg \c key		KeyCode representing a key on the keyboard.
 * \arg \c mod		Keyboard modifier.
 * \arg \c repeat	Indicates that this event is a repeat and not an initial key event.
 */
EventHandler::KeyDownEventSource& EventHandler::keyDown()
{
	return mKeyDownEvent;
}


/**
 * Keyboard key up events generate three values:
 * Keyboard scan code, key modifier, and a repeat flag.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(KeyCode key, KeyModifier mod);
 * \endcode
 *
 * \arg \c key		KeyCode representing a key on the keyboard.
 * \arg \c mod		Keyboard modifier.
 */
EventHandler::KeyUpEventSource& EventHandler::keyUp()
{
	return mKeyUpEvent;
}


/**
 * Fires whenever text input events occur.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(const std::string&);
 * \endcode
 */
EventHandler::TextInputEventSource& EventHandler::textInput()
{
	return mTextInput;
}


/**
 * Mouse button events generate three values:
 * MouseButton, x, and y.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(MouseButton button, int x, int y);
 * \endcode
 *
 * \arg \c button: MouseButton value indicating which button is pressed.
 * \arg \c x: X position of the mouse button event.
 * \arg \c y: Y position of the mouse button event.
 */
EventHandler::MouseButtonEventSource& EventHandler::mouseButtonDown()
{
	return mMouseButtonDownEvent;
}


/**
 * Mouse button events generate three values:
 * MouseButton, x, and y.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(MouseButton button, int x, int y);
 * \endcode
 *
 * \arg \c button: MouseButton value indicating which button is pressed.
 * \arg \c x: X position of the mouse button event.
 * \arg \c y: Y position of the mouse button event.
 */
EventHandler::MouseButtonEventSource& EventHandler::mouseButtonUp()
{
	return mMouseButtonUpEvent;
}


/**
 * Mouse button events generate three values:
 * MouseButton, x, and y.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(MouseButton button, int x, int y);
 * \endcode
 *
 * \arg \c button: MouseButton value indicating which button is pressed.
 * \arg \c x: X position of the mouse button event.
 * \arg \c y: Y position of the mouse button event.
 */
EventHandler::MouseButtonEventSource& EventHandler::mouseDoubleClick()
{
	return mMouseDoubleClick;
}


/**
 * Mouse motion events generate four values:
 * x, y, relative x, and relative y values.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int x, int y, int relX, int relY);
 * \endcode
 *
 * \arg \c x: Absolute x position of the mouse.
 * \arg \c y: Absolute y position of the mouse.
 * \arg \c relX: X position of the mouse relative to its last position.
 * \arg \c relY: Y position of the mouse relative to its last position.
 */
EventHandler::MouseMotionEventSource& EventHandler::mouseMotion()
{
	return mMouseMotionEvent;
}


/**
 * Mouse wheel events generate two values:
 * x and y motion values.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(int x, int y);
 * \endcode
 *
 * \arg \c x: Change along the X-Axis.
 * \arg \c y: Change along the Y-Axis.
 *
 * \note The value given in the axis parameters is the value
 * of a single 'click' of the mouse wheel. This may be
 * more than one (on Windows this value is typical 120,
 * not 1).
 */
EventHandler::MouseWheelEventSource& EventHandler::mouseWheel()
{
	return mMouseWheelEvent;
}


/**
 * Triggered whenever a Quit message is posted.
 *
 * To connect an event handler to this event, call the 'connect()'
 * function as follows:
 *
 * \code
 * connect(this, &Object::function);
 * \endcode
 *
 * \code
 * void function(void);
 * \endcode
 */
EventHandler::QuitEventSource& EventHandler::quit()
{
	return mQuitEvent;
}


/**
 * Grabs exclusive mouse input.
 */
void EventHandler::grabMouse()
{
	if (underlyingWindow) { SDL_SetWindowGrab(underlyingWindow, SDL_TRUE); }
}


/**
 * Releases exclusive mouse input.
 */
void EventHandler::releaseMouse()
{
	if (underlyingWindow) { SDL_SetWindowGrab(underlyingWindow, SDL_FALSE); }
}


/**
 * Sets the mouse pointer to a specified location within the application window.
 *
 * \param x X-Coordinate.
 * \param y Y-Coordinate.
 *
 * \note	Coordinates can only be set within the bounds of the application's
 *			window. Coordinates will be clamped for all other values.
 */
void EventHandler::warpMouse(int x, int y)
{
	if (underlyingWindow)
	{
		SDL_WarpMouseInWindow(underlyingWindow, x, y);
		mMouseMotionEvent.emit(x, y, 0, 0);
	}
}


/**
 * Sets mouse updates to relative or absolute mostion.
 *
 * \param	rel		True for relative mode. False for absolute mode.
 */
void EventHandler::mouseRelativeMode(bool rel)
{
	rel ? SDL_SetRelativeMouseMode(SDL_TRUE) : SDL_SetRelativeMouseMode(SDL_FALSE);
}


/**
 * Updates the event pump and calls any associated callbacks.
 */
void EventHandler::pump()
{
	SDL_Event event;

	int count = 0;
	while ((SDL_PollEvent(&event) != 0) && (count < MAX_MESSAGE_PROCESSING))
	{
		switch (event.type)
		{
		case SDL_MOUSEMOTION:
			mMouseMotionEvent(event.motion.x, event.motion.y, event.motion.xrel, event.motion.yrel);
			break;

		case SDL_KEYDOWN:
			mKeyDownEvent(static_cast<KeyCode>(event.key.keysym.sym), static_cast<KeyModifier>(event.key.keysym.mod), event.key.repeat != 0 ? true : false);
			break;

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
