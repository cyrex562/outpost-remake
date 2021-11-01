#pragma once

#include "Structure.h"

#include "../../Constants.h"


class University : public Structure
{

	University() : Structure(constants::University,
		"structures/university.sprite",
		StructureClass::University,
		StructureID::SID_UNIVERSITY)
	{
		maxAge(500);
		turnsToBuild(4);

		requiresCHAP(true);
		hasCrime(true);
	}

protected:
	void defineResourceInput() override
	{
		energyRequired(1);
	}
};
