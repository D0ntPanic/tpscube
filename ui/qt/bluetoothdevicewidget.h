#pragma once

#include <QtBluetooth/QBluetoothDeviceDiscoveryAgent>
#include <QtBluetooth/QBluetoothDeviceInfo>
#include <QtBluetooth/QBluetoothUuid>
#include <QtWidgets/QWidget>
#include <QtWidgets/QLabel>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QPushButton>
#include <set>
#include "bluetoothcube.h"

class BluetoothDeviceWidget: public QWidget
{
	Q_OBJECT

	QBluetoothDeviceDiscoveryAgent* m_discovery;
	std::set<QBluetoothUuid> m_devices;
	QBluetoothDeviceInfo m_selectedDevice;
	BluetoothCubeType* m_selectedCubeType;

	QLabel* m_noAvailableLabel;
	QLabel* m_errorLabel;
	QVBoxLayout* m_listLayout;
	QPushButton* m_nextButton;
	QPushButton* m_cancelButton;

private slots:
	void discoveredDevice(const QBluetoothDeviceInfo& device);
	void discoveryFailed(QBluetoothDeviceDiscoveryAgent::Error error);
	void cancelPushed();

public:
	BluetoothDeviceWidget();
	~BluetoothDeviceWidget();

	const QBluetoothDeviceInfo& selectedDevice() const { return m_selectedDevice; }
	BluetoothCubeType* selectedCubeType() const { return m_selectedCubeType; }

signals:
	void next();
	void cancel();
};
