#pragma once

#include "Factory.h"

#include "../../Constants.h"

class UndergroundFactory : public Factory
{

	UndergroundFactory() : Factory(constants::UndergroundFactory,
								   "structures/factory_underground.sprite",
								   StructureID::SID_UNDERGROUND_FACTORY)
	{
		maxAge(600);
		turnsToBuild(4);

		requiresCHAP(false);
		hasCrime(true);

		initFactory();
	}

	void initFactory() override
	{
		// Robot digger for now. Need to be replaced by non robot/surface goods
		// Produces luxuries, clothing, or medicine
		addProduct(ProductType::PRODUCT_CLOTHING);
		addProduct(ProductType::PRODUCT_MEDICINE);
	}

	void defineResourceInput() override
	{
		energyRequired(10);
	}
};
