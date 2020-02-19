#include <QtWidgets/QVBoxLayout>
#include "bluetoothdialog.h"


BluetoothDialog::BluetoothDialog()
{
	QVBoxLayout* layout = new QVBoxLayout();
	m_stackedWidget = new QStackedWidget();

	m_devices = new BluetoothDeviceWidget();
	m_devicesIndex = m_stackedWidget->addWidget(m_devices);
	connect(m_devices, &BluetoothDeviceWidget::cancel, this, &BluetoothDialog::reject);
	connect(m_devices, &BluetoothDeviceWidget::next, this, &BluetoothDialog::deviceSelected);

	m_connect = new BluetoothConnectWidget();
	m_connectIndex = m_stackedWidget->addWidget(m_connect);
	connect(m_connect, &BluetoothConnectWidget::cancel, this, &BluetoothDialog::reject);
	connect(m_connect, &BluetoothConnectWidget::failed, this, &BluetoothDialog::connectFailed);
	connect(m_connect, &BluetoothConnectWidget::next, this, &BluetoothDialog::connectComplete);

	m_check = new BluetoothCheckWidget();
	m_checkIndex = m_stackedWidget->addWidget(m_check);

	m_stackedWidget->setCurrentIndex(m_devicesIndex);

	layout->addWidget(m_stackedWidget);
	setLayout(layout);
	setMinimumSize(300, 350);
}


void BluetoothDialog::deviceSelected()
{
	m_connect->connectToDevice(m_devices->selectedDevice(), m_devices->selectedCubeType());
	m_stackedWidget->setCurrentIndex(m_connectIndex);
}


void BluetoothDialog::deviceConnected()
{
}


void BluetoothDialog::connectFailed()
{
	m_stackedWidget->setCurrentIndex(m_devicesIndex);
}


void BluetoothDialog::connectComplete()
{
	m_check->setCube(m_connect->cube());
	m_stackedWidget->setCurrentIndex(m_checkIndex);
}
