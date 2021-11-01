#pragma once

#include "Core/Window.h"
#include "Core/Button.h"


class GameOverDialog : public Window
{

	using ClickSignal = NAS2D::Signal<>;


	GameOverDialog();

	ClickSignal::Source& returnToMainMenu() { return mSignal; }

	void update() override;


	void onClose();

	const NAS2D::Image& mHeader;

	Button btnClose;

	ClickSignal mSignal;
};
