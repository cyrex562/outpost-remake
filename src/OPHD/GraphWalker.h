#pragma once

#include "Common.h"

#include "Map/TileMap.h"

#include <NAS2D/Math/Point.h>


/**
 * GraphWalker does a basic depth-first connection check
 *			on a TileMap given a starting point.
 */
class GraphWalker
{

	GraphWalker(const MapCoordinate&, TileMap&, TileList&);
	~GraphWalker() = default;


	GraphWalker() = delete;
	GraphWalker(GraphWalker&) = delete;
	GraphWalker& operator=(const GraphWalker&) = delete;


	void walkGraph();
	void check(const MapCoordinate& fromPosition, Direction direction);


	TileMap& mTileMap;
	Tile& mThisTile;
	TileList& mTileList;

	MapCoordinate mPosition{};
};
