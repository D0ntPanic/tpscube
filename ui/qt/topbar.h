#pragma once

#include <QtWidgets/QLabel>
#include "utilwidgets.h"
#include "bluetoothcube.h"

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
	bool isConnectedToBluetoothCube() const;

signals:
	void showTimer();
	void showHistory();
	void showGraphs();
	void showAlgorithms();
	void showSettings();
	void connectToBluetoothCube();
	void disconnectFromBluetoothCube();
	void bluetoothCubeError(QString name, QString msg);
};
