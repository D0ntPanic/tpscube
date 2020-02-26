#pragma once

#include <QtWidgets/QLabel>
#include "utilwidgets.h"
#include "bluetoothcube.h"

class ModeLabel: public QLabel
{
	bool m_active = false;
	std::function<void()> m_onClick;

protected:
	virtual void mousePressEvent(QMouseEvent* event);
	virtual void enterEvent(QEvent* event);
	virtual void leaveEvent(QEvent* event);

public:
	ModeLabel(const QString& text, const std::function<void()>& func);
	void setActive(bool active);
};

class TopBar: public QWidget
{
	Q_OBJECT

	ModeLabel* m_timerMode;
	ModeLabel* m_historyMode;
	ModeLabel* m_graphMode;
	ModeLabel* m_algorithmMode;
	ModeLabel* m_settingsMode;
	ClickableLabel* m_bluetooth;
	QPicture m_disconnectedBluetoothIcon;
	QPicture m_connectedBluetoothIcon;
	QPicture m_hoverBluetoothIcon;
	QLabel* m_bluetoothName;
	QTimer* m_bluetoothUpdateTimer;

	std::shared_ptr<BluetoothCube> m_bluetoothCube;
	std::shared_ptr<BluetoothCubeClient> m_bluetoothCubeClient;

	void timerModeClicked();
	void historyModeClicked();
	void graphModeClicked();
	void algorithmModeClicked();
	void settingsModeClicked();
	void bluetoothClicked();

private slots:
	void bluetoothUpdate();

public:
	TopBar(QWidget* parent);

	void setBluetoothCube(const std::shared_ptr<BluetoothCube>& cube);

signals:
	void showTimer();
	void showHistory();
	void showGraphs();
	void showAlgorithms();
	void showSettings();
	void connectToBluetoothCube();
	void disconnectFromBluetoothCube();
};
