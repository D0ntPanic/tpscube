#pragma once

#include <QtWidgets/QDialog>
#include <QtWidgets/QStackedWidget>
#include "bluetoothdevicewidget.h"
#include "bluetoothconnectwidget.h"
#include "bluetoothcheckwidget.h"

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

private slots:
	void deviceSelected();
	void deviceConnected();
	void connectFailed();
	void connectComplete();

public:
	BluetoothDialog();
};
