#pragma once

#include "Core/Button.h"
#include "Core/TextArea.h"
#include "Core/Window.h"

#include "StringTable.h"

#include "../Common.h"

#include "../Things/Robots/Robot.h"


class RobotInspector : public Window
{

	RobotInspector();

	void focusOnRobot(Robot*);
	const Robot* focusedRobot() const { return mRobot; }

	NAS2D::Signal<Robot*>& actionButtonClicked() { return mSignal; }

	void update() override;


	void onCancelOrders();
	void onSelfDestruct();
	void onCancel();


	Button btnCancelOrders;
	Button btnSelfDestruct;
	Button btnCancel;

	NAS2D::Rectangle<int> mContentArea;

	NAS2D::Signal<Robot*> mSignal;

	Robot* mRobot{nullptr};
};
