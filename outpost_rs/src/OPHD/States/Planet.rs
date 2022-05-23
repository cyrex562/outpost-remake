#pragma once

#include <NAS2D/Signal/Signal.h>
#include <NAS2D/Timer.h>
#include <NAS2D/Math/Point.h>
#include <NAS2D/Resource/Image.h>

#include <cmath>
#include <string>
#include <unordered_map>

class Planet
{

	enum class PlanetType
	{
		None,

		Mercury,
		Mars,
		Ganymede,

		Count
	};

	enum class Hostility
	{
		None,

		Low,
		Medium,
		High
	};

	struct Attributes
	{
		PlanetType type = PlanetType::None;
		std::string imagePath;
		Hostility hostility = Hostility::None;
		int maxDepth = 0;
		int maxMines = 0;
		std::string mapImagePath;
		std::string tilesetPath;
		std::string name;

		/* Mean distance from star in astronomical units (AU) */
		float meanSolarDistance = 0;

		std::string description;
	};

	using MouseSignal = NAS2D::Signal<>;

	Planet(const Attributes &attributes);
	~Planet();

	NAS2D::Point<int> position() const { return mPosition; }
	void position(const NAS2D::Point<int> &point) { mPosition = point; }
	void position(int x, int y) { mPosition = {x, y}; }

	const Attributes &attributes() const { return mAttributes; }

	bool mouseHovering() const { return mMouseInArea; }

	MouseSignal::Source &mouseEnter() { return mMouseEnterSignal; }
	MouseSignal::Source &mouseExit() { return mMouseExitSignal; }

	void update();

	bool pointInCircle(NAS2D::Point<int> point);
	void onMouseMove(int x, int y, int rX, int rY);

	Planet() = delete;
	Planet(const Planet &) = delete;
	Planet &operator=(const Planet &) = delete;

	int mTick = 0;

	Attributes mAttributes;

	const NAS2D::Image mImage;
	NAS2D::Point<int> mPosition;

	MouseSignal mMouseEnterSignal;
	MouseSignal mMouseExitSignal;

	bool mMouseInArea = false;

	NAS2D::Timer mTimer;
};

std::vector<Planet::Attributes> parsePlanetAttributes();
