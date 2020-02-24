#pragma once

#include <QtWidgets/QWidget>
#include "bluetoothcube.h"
#include "cube3x3widget.h"
#include "utilwidgets.h"

class BluetoothResetWidget: public QWidget
{
	Q_OBJECT

	std::shared_ptr<BluetoothCube> m_cube;
	std::shared_ptr<BluetoothCubeClient> m_cubeClient;
	Heading* m_heading;
	Cube3x3Widget* m_cubeWidget;

private slots:
	void donePushed();
	void cancelPushed();

public:
	BluetoothResetWidget();
	~BluetoothResetWidget();

	void setCube(const std::shared_ptr<BluetoothCube>& cube);

signals:
	void done();
	void cancel();
};
