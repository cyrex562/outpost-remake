#pragma once

#include "Common.h"
#include "Constants.h"

#include <NAS2D/Math/Point.h>
#include <NAS2D/Xml/XmlElement.h>

#include <bitset>

class Mine
{
public:
	enum OreType
	{
		ORE_COMMON_METALS,
		ORE_COMMON_MINERALS,
		ORE_RARE_METALS,
		ORE_RARE_MINERALS,
	};

	using MineVein = std::array<int, 4>;
	using MineVeins = std::vector<MineVein>;

public:
	Mine();
	Mine(MineProductionRate rate);

	bool active() const;
	void active(bool newActive);

	bool exhausted() const;
	void checkExhausted();

	MineProductionRate productionRate() const { return mProductionRate; }

	int depth() const;
	void increaseDepth();

	int commonMetalsAvailable() const;
	int commonMineralsAvailable() const;
	int rareMetalsAvailable() const;
	int rareMineralsAvailable() const;

	int oreAvailable(size_t) const;
	int oreTotalYield(size_t) const;

	bool miningCommonMetals() const;
	bool miningCommonMinerals() const;
	bool miningRareMetals() const;
	bool miningRareMinerals() const;

	void miningCommonMetals(bool value);
	void miningCommonMinerals(bool value);
	void miningRareMetals(bool value);
	void miningRareMinerals(bool value);

	int pull(OreType type, int quantity);

public:
	NAS2D::Xml::XmlElement* serialize(NAS2D::Point<int> location);
	void deserialize(NAS2D::Xml::XmlElement* element);

private:
	Mine(const Mine&) = delete;
	Mine& operator=(const Mine&) = delete;

private:
	MineVeins mVeins; /**< Ore veins */
	MineProductionRate mProductionRate = MineProductionRate::Low; /**< Mine's production rate. */

	/**
	 * Flags indicating several states for the mine:
	 * 
	 * [0] : Mine Common Metal Ore
	 * [1] : Mine Common Mineral Ore
	 * [2] : Mine Rare Metal Ore
	 * [3] : Mine Rare Mineral Ore
	 * [4] : Mine is active
	 * [5] : Mine is exhausted
	 */
	std::bitset<6> mFlags; /**< Set of flags. */
};
