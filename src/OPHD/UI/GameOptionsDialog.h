#pragma once

#include "Core/Window.h"
#include "Core/Button.h"


class GameOptionsDialog : public Window
{

	using ClickSignal = NAS2D::Signal<>;

	GameOptionsDialog();
	~GameOptionsDialog() override;

	void update() override;

	ClickSignal::Source& SaveGame() { return mSignalSave; }
	ClickSignal::Source& LoadGame() { return mSignalLoad; }
	ClickSignal::Source& returnToGame() { return mSignalReturn; }
	ClickSignal::Source& returnToMainMenu() { return mSignalClose; }


	void onLoad();
	void onSave();
	void onReturn();
	void onClose();

	void onEnableChange() override;

	Button btnSave;
	Button btnLoad;
	Button btnReturn;
	Button btnClose;

	ClickSignal mSignalSave;
	ClickSignal mSignalLoad;
	ClickSignal mSignalReturn;
	ClickSignal mSignalClose;
};
