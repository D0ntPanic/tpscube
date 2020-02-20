#pragma once

#include <QtWidgets/QDialog>
#include <QtWidgets/QStackedWidget>
#include "bluetoothdevicewidget.h"
#include "bluetoothconnectwidget.h"
#include "bluetoothcheckwidget.h"
#include "bluetoothresetwidget.h"

class BluetoothDialog: public QDialog
{
	Q_OBJECT

	QStackedWidget* m_stackedWidget;
	BluetoothDeviceWidget* m_devices;
	int m_devicesIndex;
	BluetoothConnectWidget* m_connect;
	int m_connectIndex;
	BluetoothCheckWidget* m_check;
	int m_checkIndex;
	BluetoothResetWidget* m_reset;
	int m_resetIndex;
	std::shared_ptr<BluetoothCube> m_cube;

private slots:
	void deviceSelected();
	void deviceConnected();
	void connectFailed();
	void connectComplete();
	void stateCorrect();
	void stateIncorrect();
	void resetComplete();

public:
	BluetoothDialog();

	const std::shared_ptr<BluetoothCube>& cube() const { return m_cube; }
};
