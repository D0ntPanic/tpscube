#include <QtWidgets/QHBoxLayout>
#include "bluetoothdevicewidget.h"
#include "utilwidgets.h"
#include "theme.h"


BluetoothDeviceWidget::BluetoothDeviceWidget()
{
	QVBoxLayout* layout = new QVBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	layout->addWidget(new Heading("Connect to a Bluetooth Cube"));

	m_noAvailableLabel = new QLabel("No compatible devices found.");
	m_noAvailableLabel->setFont(fontOfRelativeSize(1.0f, QFont::Thin, true));
	layout->addWidget(m_noAvailableLabel);

	m_listLayout = new QVBoxLayout();
	m_listLayout->setContentsMargins(0, 0, 0, 0);
	layout->addLayout(m_listLayout);
	layout->addStretch(1);

	m_errorLabel = new QLabel("Scanning for devices...");
	m_errorLabel->setFont(fontOfRelativeSize(1.0f, QFont::Normal));
	layout->addWidget(m_errorLabel);

	QHBoxLayout* buttonLayout = new QHBoxLayout();
	buttonLayout->addStretch(1);
	m_cancelButton = new QPushButton("Cancel");
	connect(m_cancelButton, &QPushButton::clicked, this, &BluetoothDeviceWidget::cancelPushed);
	buttonLayout->addWidget(m_cancelButton);

	layout->addLayout(buttonLayout);
	setLayout(layout);

	m_discovery = new QBluetoothDeviceDiscoveryAgent(this);
	m_discovery->setLowEnergyDiscoveryTimeout(0);
	connect(m_discovery, &QBluetoothDeviceDiscoveryAgent::deviceDiscovered,
		this, &BluetoothDeviceWidget::discoveredDevice);
	connect(m_discovery, QOverload<QBluetoothDeviceDiscoveryAgent::Error>::of(
		&QBluetoothDeviceDiscoveryAgent::error), this, &BluetoothDeviceWidget::discoveryFailed);
	m_discovery->start(QBluetoothDeviceDiscoveryAgent::LowEnergyMethod);
}


BluetoothDeviceWidget::~BluetoothDeviceWidget()
{
	m_discovery->stop();
}


void BluetoothDeviceWidget::discoveredDevice(const QBluetoothDeviceInfo& device)
{
	if (m_devices.count(device.deviceUuid()) != 0)
		return;

	if (device.coreConfigurations() & QBluetoothDeviceInfo::LowEnergyCoreConfiguration)
	{
		BluetoothCubeType* cubeType = BluetoothCubeType::GetTypeForName(device.name().toStdString());
		if (cubeType)
		{
			m_devices.insert(device.deviceUuid());
			ClickableLabel* label = new ClickableLabel(device.name(), Theme::content, Theme::blue,
				[=]() {
					m_selectedDevice = device;
					m_selectedCubeType = cubeType;
					next();
				});
			label->setCursor(Qt::PointingHandCursor);
			label->setFont(fontOfRelativeSize(1.1f));
			m_listLayout->addWidget(label);
			m_noAvailableLabel->hide();
		}
	}
}


void BluetoothDeviceWidget::discoveryFailed(QBluetoothDeviceDiscoveryAgent::Error)
{
	m_errorLabel->setText(m_discovery->errorString());
}


void BluetoothDeviceWidget::cancelPushed()
{
	emit cancel();
}
