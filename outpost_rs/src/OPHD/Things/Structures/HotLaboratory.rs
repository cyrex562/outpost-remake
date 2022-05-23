#pragma once

#include "Structure.h"

#include "../../Constants.h"

class HotLaboratory : public Structure
{

	HotLaboratory() : Structure(constants::HotLaboratory,
								"structures/labo_surface.sprite",
								StructureClass::Laboratory,
								StructureID::SID_HOT_LABORATORY)
	{
		maxAge(500);
		turnsToBuild(5);
		integrityDecayRate(2);

		requiresCHAP(false);
	}

	void defineResourceInput() override
	{
		energyRequired(3);
	}
};
