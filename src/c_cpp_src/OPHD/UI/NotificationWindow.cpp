#include "NotificationWindow.h"

#include "../Cache.h"
#include "../Constants.h"

#include <NAS2D/Utility.h>
#include <NAS2D/Renderer/Renderer.h>


using namespace NAS2D;


NotificationWindow::NotificationWindow():
	mIcons{imageCache.load("ui/icons.png")}
{
	size({300, 220});

	add(btnOkay, {245, 195});
	btnOkay.size({50, 20});

	add(btnTakeMeThere, {10, 195});
	btnTakeMeThere.size({125, 20});
	btnTakeMeThere.hide();

	add(mMessageArea, {5, 65});
	mMessageArea.size({size().x - 10, 125});
	mMessageArea.font(constants::FONT_PRIMARY, constants::FontPrimaryNormal);
}


void NotificationWindow::notification(const NotificationArea::Notification& notification)
{
	mNotification = notification;
	title(StringFromNotificationType(mNotification.type));
	mMessageArea.text(mNotification.message);

	title(mNotification.brief);

	mTakeMeThereVisible = mNotification.position != Point<int>{-1, -1}; //\fixme magic value
}


void NotificationWindow::btnOkayClicked()
{
	hide();
}


void NotificationWindow::btnTakeMeThereClicked()
{
	mTakeMeThereClicked(mNotification.position, mNotification.depth);
	hide();
}


void NotificationWindow::update()
{
	if (!visible()) { return; }

	Window::update();

	btnTakeMeThere.visible(mTakeMeThereVisible); // bit of a hack

	auto& renderer = Utility<Renderer>::get();

	Point<float> iconLocation = position() + Vector{10, 30};

	renderer.drawSubImage(mIcons, iconLocation, {128, 64, 32, 32}, ColorFromNotification(mNotification.type));
	renderer.drawSubImage(mIcons, iconLocation, IconRectFromNotificationType(mNotification.type), Color::Normal);
}
