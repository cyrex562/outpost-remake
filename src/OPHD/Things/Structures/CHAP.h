#pragma once

#include "Structure.h"

#include "../../Constants.h"


class CHAP : public Structure
{

	CHAP() : Structure(constants::Chap, "structures/chap.sprite", StructureClass::LifeSupport, StructureID::SID_CHAP)
	{
		maxAge(600);
		turnsToBuild(5);

		requiresCHAP(false);
	}

protected:
	void defineResourceInput() override
	{
		resourcesIn({2, 0, 1, 1});
		energyRequired(10);
	}
};
