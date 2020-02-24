//#define BLUETOOTH_DEBUG
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QLabel>
#include <QtWidgets/QMessageBox>
#include "bluetoothconnectwidget.h"
#include "theme.h"

using namespace std;


QtBluetoothDevice::QtBluetoothDevice(const QBluetoothDeviceInfo& device): m_device(device)
{
	m_control = QLowEnergyController::createCentral(device, this);
	connect(m_control, &QLowEnergyController::serviceDiscovered, this, &QtBluetoothDevice::serviceDiscovered);
	connect(m_control, &QLowEnergyController::discoveryFinished, this, &QtBluetoothDevice::discoveryFinished);
	connect(m_control, &QLowEnergyController::connected, this, &QtBluetoothDevice::connected);
	connect(m_control, &QLowEnergyController::disconnected, this, &QtBluetoothDevice::disconnected);
	connect(m_control, QOverload<QLowEnergyController::Error>::of(
		&QLowEnergyController::error), this, &QtBluetoothDevice::failed);
}


void QtBluetoothDevice::connectToDevice()
{
	m_control->connectToDevice();
}


void QtBluetoothDevice::connected()
{
	if (m_servicesDiscovered)
		Connect();
	else
		m_control->discoverServices();
}


void QtBluetoothDevice::disconnected()
{
	Error("Device disconnected");
}


void QtBluetoothDevice::serviceDiscovered(const QBluetoothUuid& service)
{
	m_services.insert(service);
}


void QtBluetoothDevice::discoveryFinished()
{
	m_servicesDiscovered = true;
	Connect();
}


void QtBluetoothDevice::serviceStateChanged(QLowEnergyService::ServiceState state)
{
	switch (state)
	{
	case QLowEnergyService::ServiceDiscovered:
#ifdef BLUETOOTH_DEBUG
		for (auto& i : m_service->characteristics())
		{
			printf("Characteristic %s (%s) %x\n", i.uuid().toString().toStdString().c_str(),
				i.name().toStdString().c_str(), (int)i.properties());
			for (auto& j : i.descriptors())
			{
				printf("  Descriptor %s (%s) %x\n", j.uuid().toString().toStdString().c_str(),
					j.name().toStdString().c_str(), j.type());
			}
		}
#endif
		m_serviceConnectedFunc();
		break;
	case QLowEnergyService::InvalidService:
		Error("Accessing invalid service of device");
		break;
	default:
		break;
	}
}


void QtBluetoothDevice::characteristicRead(const QLowEnergyCharacteristic&, const QByteArray& value)
{
	vector<uint8_t> valueVector;
	valueVector.insert(valueVector.begin(), (const uint8_t*)value.data(),
		(const uint8_t*)(value.data() + value.size()));
	if (m_readCharacteristicEncoded)
		valueVector = m_decodeFunc(valueVector);
#ifdef BLUETOOTH_DEBUG
	printf("    ");
	for (size_t i = 0; i < valueVector.size(); i++)
	{
		if ((i != 0) && ((i & 15) == 0))
			printf("\n    ");
		printf("%.2x ", valueVector[i]);
	}
	printf("\n");
#endif
	m_readCharacteristicResultFunc(valueVector);
}


void QtBluetoothDevice::characteristicWritten(const QLowEnergyCharacteristic&, const QByteArray& value)
{
#ifdef BLUETOOTH_DEBUG
	vector<uint8_t> valueVector;
	valueVector.insert(valueVector.begin(), (const uint8_t*)value.data(),
		(const uint8_t*)(value.data() + value.size()));
	printf("    ");
	for (size_t i = 0; i < valueVector.size(); i++)
	{
		if ((i != 0) && ((i & 15) == 0))
			printf("\n    ");
		printf("%.2x ", valueVector[i]);
	}
	printf("\n");
#else
	(void)value;
#endif
	m_writeCharacteristicDoneFunc();
}


void QtBluetoothDevice::characteristicChanged(const QLowEnergyCharacteristic& characteristic, const QByteArray& value)
{
	vector<uint8_t> valueVector;
	valueVector.insert(valueVector.begin(), (const uint8_t*)value.data(),
		(const uint8_t*)(value.data() + value.size()));
	if (m_decodeFunc)
		valueVector = m_decodeFunc(valueVector);
	printf("Characteristic %s changed:\n", characteristic.uuid().toString().toStdString().c_str());
	for (size_t i = 0; i < valueVector.size(); i++)
	{
		if ((i != 0) && ((i & 15) == 0))
			printf("\n    ");
		printf("%.2x ", valueVector[i]);
	}
	printf("\n");
}


void QtBluetoothDevice::descriptorWritten(const QLowEnergyDescriptor&, const QByteArray&)
{
	m_writeDescriptorDoneFunc();
}


void QtBluetoothDevice::failed(QLowEnergyController::Error)
{
	Error(m_control->errorString().toStdString());
}


string QtBluetoothDevice::GetName()
{
	return m_device.name().toStdString();
}


void QtBluetoothDevice::ConnectToService(const string& uuid,
	const function<void()>& serviceConnectedFunc)
{
	QBluetoothUuid service(QString::fromStdString(uuid));
	if (m_services.count(service) == 0)
	{
		Error(("Device does not have required service " + QString::fromStdString(uuid)).toStdString());
		return;
	}

#ifdef BLUETOOTH_DEBUG
	printf("Connecting to service: %s\n", uuid.c_str());
#endif
	if (m_service)
		delete m_service;
	m_service = m_control->createServiceObject(service, this);
	m_serviceConnectedFunc = serviceConnectedFunc;
	connect(m_service, &QLowEnergyService::stateChanged, this, &QtBluetoothDevice::serviceStateChanged);
	connect(m_service, &QLowEnergyService::characteristicRead, this, &QtBluetoothDevice::characteristicRead);
	connect(m_service, &QLowEnergyService::characteristicWritten, this, &QtBluetoothDevice::characteristicWritten);
	connect(m_service, &QLowEnergyService::characteristicChanged, this, &QtBluetoothDevice::characteristicChanged);
	connect(m_service, &QLowEnergyService::descriptorWritten, this, &QtBluetoothDevice::descriptorWritten);
	m_service->discoverDetails();
}


void QtBluetoothDevice::ReadCharacteristic(const string& uuid,
	const function<void(const vector<uint8_t>& data)>& resultFunc)
{
#ifdef BLUETOOTH_DEBUG
	printf("Reading characteristic: %s\n", uuid.c_str());
#endif
	if (!m_service)
	{
		Error(("Reading characteristic " + QString::fromStdString(uuid) +
			" without a connected service").toStdString());
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		Error(("Device does not have required characteristic " +
			QString::fromStdString(uuid)).toStdString());
		return;
	}
	m_readCharacteristicResultFunc = resultFunc;
	m_readCharacteristicEncoded = false;
	m_service->readCharacteristic(characteristic);
}


void QtBluetoothDevice::ReadEncodedCharacteristic(const string& uuid,
	const function<void(const vector<uint8_t>& data)>& resultFunc)
{
#ifdef BLUETOOTH_DEBUG
	printf("Reading encoded characteristic: %s\n", uuid.c_str());
#endif
	if (!m_service)
	{
		Error(("Reading characteristic " + QString::fromStdString(uuid) +
			" without a connected service").toStdString());
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		Error(("Device does not have required characteristic " +
			QString::fromStdString(uuid)).toStdString());
		return;
	}
	m_readCharacteristicResultFunc = resultFunc;
	m_readCharacteristicEncoded = true;
	m_service->readCharacteristic(characteristic);
}


void QtBluetoothDevice::SetDecoder(const function<vector<uint8_t>(const vector<uint8_t>&)>& decodeFunc)
{
	m_decodeFunc = decodeFunc;
}


void QtBluetoothDevice::WriteCharacteristic(const string& uuid, const std::vector<uint8_t>& data,
	const function<void()>& doneFunc)
{
#ifdef BLUETOOTH_DEBUG
	printf("Writing characteristic: %s\n", uuid.c_str());
#endif
	if (!m_service)
	{
		Error(("Writing characteristic " + QString::fromStdString(uuid) +
			" without a connected service").toStdString());
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		Error(("Device does not have required characteristic " +
			QString::fromStdString(uuid)).toStdString());
		return;
	}
	m_writeCharacteristicDoneFunc = doneFunc;
	m_service->writeCharacteristic(characteristic, QByteArray((const char*)&data[0], data.size()));
}


void QtBluetoothDevice::EnableNotifications(const string& uuid, const function<void()>& doneFunc)
{
#ifdef BLUETOOTH_DEBUG
	printf("Enabling notifications for characteristic: %s\n", uuid.c_str());
#endif
	if (!m_service)
	{
		Error(("Enabling notifications for characteristic " +
			QString::fromStdString(uuid) + " without a connected service").toStdString());
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		Error(("Device does not have required characteristic " +
			QString::fromStdString(uuid)).toStdString());
		return;
	}

	QLowEnergyDescriptor notification = characteristic.descriptor(QBluetoothUuid::ClientCharacteristicConfiguration);
	if (!notification.isValid())
	{
		Error(("Device does not support notifications for characteristic " +
			QString::fromStdString(uuid)).toStdString());
		return;
	}

	static char enable[2] = {1, 0};
	m_writeDescriptorDoneFunc = doneFunc;
	m_service->writeDescriptor(notification, QByteArray(enable, sizeof(enable)));
}


void QtBluetoothDevice::DebugMessage(const string& msg)
{
#ifdef BLUETOOTH_DEBUG
	puts(msg.c_str());
#else
	(void)msg;
#endif
}


BluetoothConnectWidget::BluetoothConnectWidget()
{
	QVBoxLayout* layout = new QVBoxLayout();
	m_label = new QLabel("Connecting...");
	m_label->setFont(fontOfRelativeSize(1.2f, QFont::Thin, true));
	m_label->setAlignment(Qt::AlignVCenter | Qt::AlignCenter);
	layout->addStretch(1);
	layout->addWidget(m_label);
	layout->addStretch(1);

	QHBoxLayout* buttonLayout = new QHBoxLayout();
	buttonLayout->addStretch(1);
	QPushButton* cancelButton = new QPushButton("Cancel");
	connect(cancelButton, &QPushButton::clicked, this, &BluetoothConnectWidget::cancelPushed);
	buttonLayout->addWidget(cancelButton);
	layout->addLayout(buttonLayout);
	setLayout(layout);
}


BluetoothConnectWidget::~BluetoothConnectWidget()
{
	if (m_cube && m_cubeClient)
		m_cube->RemoveClient(m_cubeClient);
}


void BluetoothConnectWidget::cancelPushed()
{
	emit cancel();
}


void BluetoothConnectWidget::connectToDevice(const QBluetoothDeviceInfo& device, BluetoothCubeType* cubeType)
{
	m_label->setText("Connecting to " + device.name() + "...");
	QtBluetoothDevice* deviceObj = new QtBluetoothDevice(device);
	m_cube = cubeType->Create(deviceObj);
	m_cube->SetReadyCallback([this]() { emit next(); });
	m_cubeClient = make_shared<BluetoothCubeClient>();
	QString name = device.name();
	m_cubeClient->SetErrorCallback([=](const string& msg) {
		QMessageBox::critical(this, name, QString::fromStdString(msg));
		emit cancel();
	});
	m_cube->AddClient(m_cubeClient);
	deviceObj->connectToDevice();
}
