#pragma once

#include "cubewidget.h"
#include "cube3x3.h"

class Cube3x3Widget: public CubeWidget
{
	Cube3x3 m_cube;

public:
	Cube3x3Widget();

	Cube3x3& cube() { return m_cube; }
	const Cube3x3& cube() const { return m_cube; }
	virtual int cubeSize() const override { return 3; }
	virtual std::vector<CubeColor> cubeFaceColors() const override;
};
