#pragma once

#include "Core/Window.h"
#include "Core/Button.h"


class Warehouse;


/**
* Implements a Factory Production dialog interface.
*/
class WarehouseInspector : public Window
{

	WarehouseInspector();

	void warehouse(Warehouse* w);

	void hide() override;
	void update() override;


	void onClose();

	Warehouse* mWarehouse = nullptr;
	Button btnClose;
};
