#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QMessageBox>
#include <QtGui/QPainter>
#include <QtGui/QPicture>
#include "topbar.h"
#include "theme.h"

using namespace std;


ModeLabel::ModeLabel(const QString& text, const function<void()>& func):
	QLabel(text), m_onClick(func)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, Theme::disabled);
	setPalette(pal);
	setFont(fontOfRelativeSize(1.0f, QFont::Light));
	setCursor(Qt::PointingHandCursor);
}


void ModeLabel::mousePressEvent(QMouseEvent*)
{
	m_onClick();
}


void ModeLabel::enterEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::content);
	setPalette(pal);
}


void ModeLabel::leaveEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::disabled);
	setPalette(pal);
}


void ModeLabel::setActive(bool active)
{
	m_active = active;
	setFont(fontOfRelativeSize(1.0f, active ? QFont::Bold : QFont::Light));
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::disabled);
	setPalette(pal);
}


TopBar::TopBar(QWidget* parent): QWidget(parent)
{
	setBackgroundRole(QPalette::Window);
	setAutoFillBackground(true);

	QHBoxLayout* layout = new QHBoxLayout();
	m_timerMode = new ModeLabel("Timer", [this]() { timerModeClicked(); });
	layout->addWidget(m_timerMode);
	layout->addSpacing(12);
	m_historyMode = new ModeLabel("History", [this]() { historyModeClicked(); });
	layout->addWidget(m_historyMode);
	layout->addSpacing(12);
	m_graphMode = new ModeLabel("Graphs", [this]() { graphModeClicked(); });
	layout->addWidget(m_graphMode);
	layout->addSpacing(12);
	m_algorithmMode = new ModeLabel("Algorithms", [this]() { algorithmModeClicked(); });
	layout->addWidget(m_algorithmMode);
	layout->addSpacing(12);
	m_timerMode->setActive(true);
	m_graphMode->setVisible(false);
	m_algorithmMode->setVisible(false);

	layout->addStretch(1);

	QImage disconnectedBluetoothImage(":/images/bluetooth_deselect.png");
	QPainter disconnectedPainter(&m_disconnectedBluetoothIcon);
	disconnectedPainter.setRenderHint(QPainter::SmoothPixmapTransform);
	disconnectedPainter.drawImage(QRect(0, 0, 20, 20), disconnectedBluetoothImage);

	QImage connectedBluetoothImage(":/images/bluetooth.png");
	QPainter connectedPainter(&m_connectedBluetoothIcon);
	connectedPainter.setRenderHint(QPainter::SmoothPixmapTransform);
	connectedPainter.drawImage(QRect(0, 0, 20, 20), connectedBluetoothImage);

	QImage hoverBluetoothImage(":/images/bluetooth_hover.png");
	QPainter hoverPainter(&m_hoverBluetoothIcon);
	hoverPainter.setRenderHint(QPainter::SmoothPixmapTransform);
	hoverPainter.drawImage(QRect(0, 0, 20, 20), hoverBluetoothImage);

	m_bluetooth = new ClickableLabel("", Theme::content, Theme::blue,
		[this]() { bluetoothClicked(); });
	m_bluetooth->setPictures(m_disconnectedBluetoothIcon, m_hoverBluetoothIcon);
	m_bluetooth->setCursor(Qt::PointingHandCursor);
	m_bluetooth->setToolTip("Connect to a Bluetooth cube");
	layout->addWidget(m_bluetooth);

	m_bluetoothName = new QLabel();
	m_bluetoothName->setFont(fontOfRelativeSize(1.0f, QFont::Thin));
	m_bluetoothName->hide();
	layout->addWidget(m_bluetoothName);

	m_bluetoothUpdateTimer = new QTimer(this);
	m_bluetoothUpdateTimer->setSingleShot(false);
	m_bluetoothUpdateTimer->setInterval(5000);
	connect(m_bluetoothUpdateTimer, &QTimer::timeout, this, &TopBar::bluetoothUpdate);

	setLayout(layout);
}


void TopBar::setBluetoothCube(const shared_ptr<BluetoothCube>& cube)
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
	{
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
		m_bluetoothCubeClient.reset();
	}

	m_bluetoothCube = cube;

	if (m_bluetoothCube)
	{
		m_bluetoothCubeClient = make_shared<BluetoothCubeClient>();
		string name = m_bluetoothCube->GetDevice()->GetName();
		m_bluetoothCubeClient->SetErrorCallback([=](const string& msg) {
			QMessageBox::critical(this, QString::fromStdString(name),
				QString::fromStdString(msg));
			emit disconnectFromBluetoothCube();
		});
		m_bluetoothCube->AddClient(m_bluetoothCubeClient);

		m_bluetooth->setPictures(m_connectedBluetoothIcon, m_hoverBluetoothIcon);
		m_bluetooth->setToolTip("Disconnect from the Bluetooth cube");
		if (m_bluetoothCube->GetBatteryState().charging)
		{
			m_bluetoothName->setText(QString::fromStdString(m_bluetoothCube->GetDevice()->GetName()) +
				QString::asprintf(" (%d%%, charging)", m_bluetoothCube->GetBatteryState().percent));
		}
		else
		{
			m_bluetoothName->setText(QString::fromStdString(m_bluetoothCube->GetDevice()->GetName()) +
				QString::asprintf(" (%d%%)", m_bluetoothCube->GetBatteryState().percent));
		}
		m_bluetoothName->show();
		m_bluetoothUpdateTimer->start();
	}
	else
	{
		m_bluetooth->setPictures(m_disconnectedBluetoothIcon, m_hoverBluetoothIcon);
		m_bluetooth->setToolTip("Connect to a Bluetooth cube");
		m_bluetoothName->hide();
		m_bluetoothUpdateTimer->stop();
	}
}


void TopBar::bluetoothUpdate()
{
	if (m_bluetoothCube)
	{
		if (m_bluetoothCube->GetBatteryState().charging)
		{
			m_bluetoothName->setText(QString::fromStdString(m_bluetoothCube->GetDevice()->GetName()) +
				QString::asprintf(" (%d%%, charging)", m_bluetoothCube->GetBatteryState().percent));
		}
		else
		{
			m_bluetoothName->setText(QString::fromStdString(m_bluetoothCube->GetDevice()->GetName()) +
				QString::asprintf(" (%d%%)", m_bluetoothCube->GetBatteryState().percent));
		}
	}
}


void TopBar::timerModeClicked()
{
	m_timerMode->setActive(true);
	m_historyMode->setActive(false);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(false);
	emit showTimer();
}


void TopBar::historyModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(true);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(false);
	emit showHistory();
}


void TopBar::graphModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(false);
	m_graphMode->setActive(true);
	m_algorithmMode->setActive(false);
	emit showGraphs();
}


void TopBar::algorithmModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(false);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(true);
	emit showAlgorithms();
}


void TopBar::bluetoothClicked()
{
	if (m_bluetoothCube)
		emit disconnectFromBluetoothCube();
	else
		emit connectToBluetoothCube();
}
