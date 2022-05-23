#pragma once

#include "Core/Window.h"
#include "Core/Button.h"


class MajorEventAnnouncement : public Window
{


	enum AnnouncementType
	{
		ANNOUNCEMENT_COLONY_SHIP_CRASH,
		ANNOUNCEMENT_COLONY_SHIP_CRASH_WITH_COLONISTS
	};

	MajorEventAnnouncement();

	void announcement(AnnouncementType a);

	void update() override;


	void onClose();

	const NAS2D::Image& mHeader;
	std::string mMessage;
	Button btnClose{"Okay", {this, &MajorEventAnnouncement::onClose}};
};
