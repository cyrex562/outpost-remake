#pragma once

#include "Structure.h"

#include "../../Constants.h"


class AirShaft : public Structure
{

	AirShaft() : Structure(constants::AirShaft, "structures/air_shaft.sprite",
		constants::StructureStateOperational,
		StructureClass::Tube,
		StructureID::SID_AIR_SHAFT)
	{
		connectorDirection(ConnectorDir::CONNECTOR_VERTICAL);

		requiresCHAP(false);
		state(StructureState::Operational);
		integrityDecayRate(0);
	}

	void ug()
	{
		sprite().play(constants::StructureStateOperationalUg);
		mIsUnderground = true;
	}

	void forced_state_change(StructureState, DisabledReason, IdleReason) override
	{
		if (mIsUnderground)
		{
			return;
		}
	}


	bool mIsUnderground = false;
};
