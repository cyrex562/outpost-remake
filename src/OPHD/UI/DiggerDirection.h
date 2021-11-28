#pragma once

#include "Core/Window.h"
#include "Core/Button.h"
#include "../Common.h"

class Tile;

class DiggerDirection : public Window
{

	using Signal = NAS2D::Signal<Direction, Tile *>;

	DiggerDirection();

	void update() override;

	Signal::Source &directionSelected() { return mSignal; }

	void setParameters(Tile *tile);

	void selectDown();

	void downOnlyEnabled();
	void cardinalOnlyEnabled();
	void allEnabled();

	void onCancel();

	void onDiggerDown();
	void onDiggerNorth();
	void onDiggerSouth();
	void onDiggerEast();
	void onDiggerWest();

	Button btnDown;
	Button btnNorth;
	Button btnEast;
	Button btnSouth;
	Button btnWest;
	Button btnCancel;

	Signal mSignal;

	Tile *mTile = nullptr;
};
