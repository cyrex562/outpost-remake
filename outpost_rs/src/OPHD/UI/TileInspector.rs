#pragma once

#include "Core/Window.h"
#include "Core/Button.h"

#include "../Map/Tile.h"


class TileInspector: public Window
{

	TileInspector();

	void tile(Tile* t) { mTile = t; }

	void update() override;


	void onClose();

	Button btnClose;
	Tile* mTile = nullptr;
};
