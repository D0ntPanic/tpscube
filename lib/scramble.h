#pragma once

#include <string>
#include "cubecommon.h"

class RandomSource
{
public:
	virtual int Next(int range) = 0;
};

// Do not use this for scrambles, this is here for deterministic unit tests.
// THIS IS NOT A GOOD SOURCE OF RANDOMNESS.
class SimpleSeededRandomSource: public RandomSource
{
	uint32_t m_seed;
public:
	SimpleSeededRandomSource();
	SimpleSeededRandomSource(uint32_t seed);
	virtual int Next(int range) override;
};

class Scrambler
{
public:
	virtual ~Scrambler() {}
	virtual std::string GetName() = 0;
	virtual CubeMoveSequence GetScramble(RandomSource& rng) = 0;
	virtual size_t GetMaxMoveCount() = 0;
};
