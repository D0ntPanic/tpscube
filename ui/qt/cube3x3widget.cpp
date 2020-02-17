#include <QtGui/QMouseEvent>
#include <math.h>
#include "cube3x3widget.h"
#include "theme.h"

using namespace std;


Cube3x3Widget::Cube3x3Widget()
{
}


vector<CubeColor> Cube3x3Widget::cubeFaceColors() const
{
	Cube3x3Faces faces(m_cube);
	vector<CubeColor> result;
	result.resize(6 * 3 * 3);
	for (int i = 0; i < 6; i++)
		for (uint8_t y = 0; y < 3; y++)
			for (uint8_t x = 0; x < 3; x++)
				result[(i * 9) + (y * 3) + x] = faces.GetColor((CubeFace)i, y, x);
	return result;
}
