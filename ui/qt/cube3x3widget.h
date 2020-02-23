#pragma once

#include "cubewidget.h"
#include "cube3x3.h"
#include "bluetoothcube.h"

class Cube3x3Widget: public CubeWidget
{
	Cube3x3 m_cube;

	std::shared_ptr<BluetoothCube> m_bluetoothCube;
	std::shared_ptr<BluetoothCubeClient> m_bluetoothCubeClient;
	QTimer* m_updateTimer;

protected:
	virtual void applyMove(CubeMove move) override;

private slots:
	void updateBluetoothCube();

public:
	Cube3x3Widget();
	~Cube3x3Widget();

	const Cube3x3& cube() const { return m_cube; }
	virtual int cubeSize() const override { return 3; }
	virtual std::vector<CubeColor> cubeFaceColors() const override;
	void setCubeState(const Cube3x3& cube);

	void setBluetoothCube(const std::shared_ptr<BluetoothCube>& cube);
};
