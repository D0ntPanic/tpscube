#include <QtGui/QMouseEvent>
#include <math.h>
#include "cube3x3widget.h"
#include "theme.h"

using namespace std;


Cube3x3Widget::Cube3x3Widget()
{
	m_updateTimer = new QTimer(this);
	m_updateTimer->setSingleShot(false);
	m_updateTimer->setInterval(50);
	connect(m_updateTimer, &QTimer::timeout, this, &Cube3x3Widget::updateBluetoothCube);
}


Cube3x3Widget::~Cube3x3Widget()
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
}


void Cube3x3Widget::applyMove(CubeMove move)
{
	m_cube.Move(move);
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


void Cube3x3Widget::setCubeState(const Cube3x3& cube)
{
	m_cube = cube;

	while (!m_movementQueue.empty())
		m_movementQueue.pop();
	m_movementActive = false;
	m_cubeNeedsUpdate = true;
	m_animationTimer->stop();
}


void Cube3x3Widget::setBluetoothCube(const shared_ptr<BluetoothCube>& cube)
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
	{
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
		m_bluetoothCubeClient.reset();
	}

	m_bluetoothCube = cube;

	if (m_bluetoothCube)
	{
		setCubeState(cube->GetCubeState());
		m_bluetoothCubeClient = make_shared<BluetoothCubeClient>();
		m_bluetoothCube->AddClient(m_bluetoothCubeClient);
		m_updateTimer->start();
	}
	else
	{
		m_updateTimer->stop();
	}
}


void Cube3x3Widget::updateBluetoothCube()
{
	if ((!m_bluetoothCube) || (!m_bluetoothCubeClient))
		return;

	TimedCubeMoveSequence timedMoves = m_bluetoothCubeClient->GetLatestMoves();
	if (timedMoves.moves.size() > 0)
	{
		CubeMoveSequence moves;
		for (auto& i : timedMoves.moves)
			moves.moves.push_back(i.move);
		apply(moves, 4, true);
	}

	m_bluetoothCube->Update();
}
