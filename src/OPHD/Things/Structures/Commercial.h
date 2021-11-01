#pragma once

#include "Structure.h"

#include "../../Constants.h"


class Commercial : public Structure
{

	Commercial() : Structure(constants::Commercial,
		"structures/commercial.sprite",
		StructureClass::Commercial,
		StructureID::SID_COMMERCIAL)
	{
		maxAge(500);
		turnsToBuild(3);

		requiresCHAP(true);
		hasCrime(true);
	}

protected:
	void defineResourceInput() override
	{
		energyRequired(2);
	}
};
