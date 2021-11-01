#pragma once

#include "Structure.h"

#include "../../Constants.h"


class Nursery : public Structure
{

	Nursery() : Structure(constants::Nursery,
		"structures/nursery_01.sprite",
		StructureClass::Nursery,
		StructureID::SID_NURSERY)
	{
		maxAge(500);
		turnsToBuild(4);

		requiresCHAP(true);
		hasCrime(true);
	}

protected:
	void defineResourceInput() override
	{
		energyRequired(2);
	}
};
