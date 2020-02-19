#pragma once

#include <QtWidgets/QWidget>
#include "bluetoothcube.h"
#include "cube3x3widget.h"
#include "utilwidgets.h"

class BluetoothCheckWidget: public QWidget
{
	Q_OBJECT

	std::shared_ptr<BluetoothCube> m_cube;
	Heading* m_heading;
	Cube3x3Widget* m_cubeWidget;

private slots:
	void correctPushed();
	void incorrectPushed();

public:
	BluetoothCheckWidget();

	void setCube(const std::shared_ptr<BluetoothCube>& cube);

signals:
	void correct();
	void incorrect();
};
