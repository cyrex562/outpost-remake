#pragma once

#include "Structure.h"

#include "../../Mine.h"

/**
 * Implements the Mine Facility.
 */
class MineFacility : public Structure
{

	using ExtensionCompleteSignal = NAS2D::Signal<MineFacility *>;

	MineFacility(Mine *mine);

	void mine(Mine *mine) { mMine = mine; }
	void maxDepth(int depth) { mMaxDepth = depth; }

	bool extending() const;
	bool canExtend() const;
	void extend();

	int digTimeRemaining() const;

	int assignedTrucks() const { return mAssignedTrucks; }
	int maxTruckCount() const { return mMaxTruckCount; }

	void addTruck() { mAssignedTrucks = std::clamp(mAssignedTrucks + 1, 1, mMaxTruckCount); }
	void removeTruck() { mAssignedTrucks = std::clamp(mAssignedTrucks - 1, 1, mMaxTruckCount); }

	/**
	 * Gets a pointer to the mine the MineFacility manages.
	 */
	Mine *mine() { return mMine; }

	ExtensionCompleteSignal::Source &extensionComplete() { return mExtensionComplete; }

	friend class MapViewState;

	void assignedTrucks(int count) { mAssignedTrucks = count; }
	void digTimeRemaining(int count) { mDigTurnsRemaining = count; }

	void think() override;

	MineFacility() = delete;
	MineFacility(const MineFacility &) = delete;
	MineFacility &operator=(const MineFacility &) = delete;

	void activated() override;

	int mMaxDepth = 0;			/**< Maximum digging depth. */
	int mDigTurnsRemaining = 0; /**< Turns remaining before extension is complete. */
	int mAssignedTrucks = 1;	/**< All mine facilities are built with at least one truck. */
	int mMaxTruckCount = 10;

	Mine *mMine = nullptr; /**< Mine that this facility manages. */

	ExtensionCompleteSignal mExtensionComplete; /**< Called whenever an extension is completed. */
};
