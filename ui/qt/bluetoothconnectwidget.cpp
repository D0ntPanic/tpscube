//#define BLUETOOTH_DEBUG
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QLabel>
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
	emit error("Device disconnected");
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
		m_serviceConnectedFunc();
		break;
	case QLowEnergyService::InvalidService:
		emit error("Accessing invalid service of device");
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


void QtBluetoothDevice::failed(QLowEnergyController::Error)
{
	emit error(m_control->errorString());
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
		emit error("Device does not have required service " + QString::fromStdString(uuid));
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
		emit error("Reading characteristic " + QString::fromStdString(uuid) + " without a connected service");
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		emit error("Device does not have required characteristic " + QString::fromStdString(uuid));
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
		emit error("Reading characteristic " + QString::fromStdString(uuid) + " without a connected service");
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		emit error("Device does not have required characteristic " + QString::fromStdString(uuid));
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
		emit error("Writing characteristic " + QString::fromStdString(uuid) + " without a connected service");
		return;
	}

	QLowEnergyCharacteristic characteristic = m_service->characteristic(
		QBluetoothUuid(QString::fromStdString(uuid)));
	if (!characteristic.isValid())
	{
		emit error("Device does not have required characteristic " + QString::fromStdString(uuid));
		return;
	}
	m_writeCharacteristicDoneFunc = doneFunc;
	m_service->writeCharacteristic(characteristic, QByteArray((const char*)&data[0], data.size()));
}


void QtBluetoothDevice::Error(const string& msg)
{
#ifdef BLUETOOTH_DEBUG
	printf("ERROR: %s\n", msg.c_str());
#endif
	emit error(QString::fromStdString(msg));
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
	deviceObj->connectToDevice();
}
