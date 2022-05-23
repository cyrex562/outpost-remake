#pragma once

#include "Structure.h"

#include "../../Constants.h"

class Laboratory : public Structure
{

	Laboratory() : Structure(constants::Laboratory,
							 "structures/laboratory_underground.sprite",
							 StructureClass::Laboratory,
							 StructureID::SID_LABORATORY)
	{
		maxAge(500);
		turnsToBuild(4);

		requiresCHAP(false);
		hasCrime(true);
	}

	void defineResourceInput() override
	{
		energyRequired(1);
	}
};
