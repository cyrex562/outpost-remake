#pragma once

#include "../Constants.h"
#include "../UI/UI.h"
#include "../UI/FileIo.h"

#include <NAS2D/State.h>
#include <NAS2D/EventHandler.h>
#include <NAS2D/Resource/Image.h>

#include <array>

/**
 * Implements the main menu screen.
 */
class MainMenuState : public NAS2D::State
{

	MainMenuState();
	~MainMenuState() override;

	void initialize() override;
	State *update() override;

	void onKeyDown(NAS2D::EventHandler::KeyCode key, NAS2D::EventHandler::KeyModifier mod, bool repeat);
	void onWindowResized(NAS2D::Vector<int> newSize);
	void onFadeComplete();

	void positionButtons();
	void disableButtons();
	void enableButtons();

	void onNewGame();
	void onContinueGame();
	void onOptions();
	void onHelp();
	void onQuit();

	void onFileIoAction(const std::string &filePath, FileIo::FileOperation fileOp);

	const NAS2D::Image mBgImage;

	FileIo mFileIoDialog; /**< File IO window. */

	std::array<Button, 4> buttons;

	Label lblVersion;
	NAS2D::State *mReturnState = this;
};
