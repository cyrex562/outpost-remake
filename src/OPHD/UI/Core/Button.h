#pragma once

#include "TextControl.h"

#include <NAS2D/Signal/Signal.h>
#include <NAS2D/EventHandler.h>
#include <NAS2D/Resource/Image.h>
#include <NAS2D/Resource/Font.h>
#include <NAS2D/Renderer/RectangleSkin.h>

#include <string>


class Button: public TextControl
{

	enum class Type
	{
		BUTTON_NORMAL,
		BUTTON_TOGGLE
	};

	using ClickSignal = NAS2D::Signal<>;

	Button(std::string newText = "");
	Button(std::string newText, ClickSignal::DelegateType clickHandler);
	~Button() override;

	void type(Type type);

	void toggle(bool toggle);
	bool toggled() const;

	void fontSize(unsigned int pointSize);

	void image(const std::string& path);
	bool hasImage() const;

	ClickSignal::Source& click() { return mSignal; }

	void update() override;

protected:
	virtual void onMouseDown(NAS2D::EventHandler::MouseButton button, int x, int y);
	virtual void onMouseUp(NAS2D::EventHandler::MouseButton button, int x, int y);
	virtual void onMouseMove(int x, int y, int dX, int dY);


	enum class State
	{
		Normal,
		Pressed
	};

	void draw() override;

	State mState = State::Normal; /**< Current state of the Button. */
	Type mType = Type::BUTTON_NORMAL; /**< Modifies Button behavior. */

	const NAS2D::Image* mImage = nullptr; /**< Image to draw centered on the Button. */

	NAS2D::RectangleSkin mSkinNormal;
	NAS2D::RectangleSkin mSkinHover;
	NAS2D::RectangleSkin mSkinPressed;

	const NAS2D::Font* mFont = nullptr; /**< Buttons can have different font sizes. */

	ClickSignal mSignal; /**< Object to notify when the Button is activated. */

	bool mMouseHover = false; /**< Mouse is within the bounds of the Button. */
};
