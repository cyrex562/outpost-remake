#pragma once

#include "ReportInterface.h"

#include "../Core/Button.h"
#include "../Core/CheckBox.h"
#include "../Core/ComboBox.h"
#include "../Core/TextArea.h"
#include "../StructureListBox.h"
#include "../../Common.h"

#include <NAS2D/Math/Rectangle.h>


namespace NAS2D
{
	class Font;
	class Image;
}

class MineFacility;


class MineReport : public ReportInterface
{

	MineReport();

	void selectStructure(Structure*) override;
	void refresh() override;

	void fillLists() override;
	void clearSelected() override;

	void update() override;


	void onShowAll();
	void onShowActive();
	void onShowIdle();
	void onShowTappedOut();
	void onShowDisabled();

	void onIdle();
	void onDigNewLevel();
	void onTakeMeThere();

	void onAddTruck();
	void onRemoveTruck();

	void onCheckBoxCommonMetalsChange();
	void onCheckBoxCommonMineralsChange();
	void onCheckBoxRareMetalsChange();
	void onCheckBoxRareMineralsChange();

	void filterButtonClicked();

	void onMineFacilitySelectionChange();

	void updateManagementButtonsVisiblity();

	void onResize() override;
	void onVisibilityChange(bool visible) override;

	void drawMineFacilityPane(const NAS2D::Point<int>&);
	void drawOreProductionPane(const NAS2D::Point<int>&);
	void drawTruckMangementPane(const NAS2D::Point<int>&);
	void drawTruckHaulInfo(const NAS2D::Point<int>&);

	const NAS2D::Font& font;
	const NAS2D::Font& fontBold;
	const NAS2D::Font& fontMedium;
	const NAS2D::Font& fontMediumBold;
	const NAS2D::Font& fontBigBold;

	const NAS2D::Image& mineFacility;
	const NAS2D::Image& uiIcons;

	Button btnShowAll;
	Button btnShowActive;
	Button btnShowIdle;
	Button btnShowTappedOut;
	Button btnShowDisabled;

	Button btnIdle;
	Button btnDigNewLevel;
	Button btnTakeMeThere;

	Button btnAddTruck;
	Button btnRemoveTruck;

	CheckBox chkCommonMetals;
	CheckBox chkCommonMinerals;
	CheckBox chkRareMetals;
	CheckBox chkRareMinerals;

	StructureListBox lstMineFacilities;

	Structure* mSelectedFacility{nullptr};

	int mAvailableTrucks{0};
};
