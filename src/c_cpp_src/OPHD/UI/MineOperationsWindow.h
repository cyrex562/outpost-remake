#pragma once

#include "Core/Window.h"
#include "Core/Button.h"
#include "Core/CheckBox.h"

#include <NAS2D/Renderer/RectangleSkin.h>


class MineFacility;


/**
 * Implements the Mine Facility Operations Window
 */
class MineOperationsWindow final : public Window
{
public:
	MineOperationsWindow();

	void mineFacility(MineFacility* facility);
	MineFacility* mineFacility() { return mFacility; }

	void updateTruckAvailability();

	void update() override;
	void hide() override;

private:
	void onCheckBoxCommonMetalsChange();
	void onCheckBoxCommonMineralsChange();
	void onCheckBoxRareMetalsChange();
	void onCheckBoxRareMineralsChange();

	void onOkay();
	void onExtendShaft();
	void onIdle();

	void onAssignTruck();
	void onUnassignTruck();

	const NAS2D::Font& mFont;
	const NAS2D::Font& mFontBold;

	MineFacility* mFacility = nullptr;

	const NAS2D::Image& mUiIcon;
	const NAS2D::Image& mIcons;

	NAS2D::RectangleSkin mPanel;

	CheckBox chkCommonMetals;
	CheckBox chkCommonMinerals;
	CheckBox chkRareMetals;
	CheckBox chkRareMinerals;

	Button btnIdle;
	Button btnExtendShaft;
	Button btnOkay;

	Button btnAssignTruck;
	Button btnUnassignTruck;

	int mAvailableTrucks = 0;
};
