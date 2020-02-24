#pragma once

#include <QtBluetooth/QBluetoothDeviceInfo>
#include <QtBluetooth/QLowEnergyController>
#include <QtBluetooth/QLowEnergyService>
#include <QtBluetooth/QLowEnergyCharacteristic>
#include <QtWidgets/QWidget>
#include <QtWidgets/QLabel>
#include <set>
#include "bluetoothcube.h"

class QtBluetoothDevice: public QObject, public BluetoothDevice
{
	Q_OBJECT

	QBluetoothDeviceInfo m_device;
	QLowEnergyController* m_control;

	bool m_servicesDiscovered = false;
	std::set<QBluetoothUuid> m_services;

	QLowEnergyService* m_service = nullptr;
	std::function<void()> m_serviceConnectedFunc;
	std::function<void(const std::vector<uint8_t>& value)> m_readCharacteristicResultFunc;
	std::function<std::vector<uint8_t>(const std::vector<uint8_t>&)> m_decodeFunc;
	bool m_readCharacteristicEncoded;
	std::function<void()> m_writeCharacteristicDoneFunc;
	std::function<void()> m_writeDescriptorDoneFunc;

private slots:
	void connected();
	void disconnected();
	void serviceDiscovered(const QBluetoothUuid& service);
	void discoveryFinished();
	void serviceStateChanged(QLowEnergyService::ServiceState state);
	void characteristicRead(const QLowEnergyCharacteristic& characteristic, const QByteArray& value);
	void characteristicWritten(const QLowEnergyCharacteristic& characteristic, const QByteArray& value);
	void characteristicChanged(const QLowEnergyCharacteristic& characteristic, const QByteArray& value);
	void descriptorWritten(const QLowEnergyDescriptor& descriptor, const QByteArray& value);
	void failed(QLowEnergyController::Error error);

public:
	QtBluetoothDevice(const QBluetoothDeviceInfo& device);
	void connectToDevice();

	virtual std::string GetName() override;
	virtual void ConnectToService(const std::string& uuid,
		const std::function<void()>& serviceConnectedFunc) override;
	virtual void ReadCharacteristic(const std::string& uuid,
		const std::function<void(const std::vector<uint8_t>& data)>& resultFunc) override;
	virtual void ReadEncodedCharacteristic(const std::string& uuid,
		const std::function<void(const std::vector<uint8_t>& data)>& resultFunc) override;
	virtual void SetDecoder(const std::function<std::vector<uint8_t>(const std::vector<uint8_t>&)>& decodeFunc) override;
	virtual void WriteCharacteristic(const std::string& uuid, const std::vector<uint8_t>& data,
		const std::function<void()>& doneFunc) override;
	virtual void EnableNotifications(const std::string& uuid, const std::function<void()>& doneFunc) override;
	virtual void DebugMessage(const std::string& msg) override;
};

class BluetoothConnectWidget: public QWidget
{
	Q_OBJECT

	QLabel* m_label;
	std::shared_ptr<BluetoothCube> m_cube;
	std::shared_ptr<BluetoothCubeClient> m_cubeClient;

private slots:
	void cancelPushed();

public:
	BluetoothConnectWidget();
	~BluetoothConnectWidget();
	void connectToDevice(const QBluetoothDeviceInfo& device, BluetoothCubeType* cubeType);

	std::shared_ptr<BluetoothCube> cube() const { return m_cube; }

signals:
	void next();
	void failed();
	void cancel();
};
