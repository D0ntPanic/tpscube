#pragma once

#include <QtWidgets/QLabel>
#include <QtCore/QTimer>
#include <chrono>
#include "bluetoothcube.h"

enum TimerState
{
	TIMER_STOPPED,
	TIMER_READY_TO_START,
	TIMER_RUNNING,
	TIMER_BLUETOOTH_READY
};

class TimerWidget: public QLabel
{
	Q_OBJECT

	TimerState m_state = TIMER_STOPPED;
	std::chrono::time_point<std::chrono::steady_clock> m_startTime, m_endTime, m_prepareTime;
	int m_fontSize = 80;
	QTimer* m_updateTimer;
	bool m_enabled = true;

	std::shared_ptr<BluetoothCube> m_bluetoothCube;
	std::shared_ptr<BluetoothCubeClient> m_bluetoothCubeClient;
	uint64_t m_bluetoothStartTimestamp, m_bluetoothLastTimestamp;
	QTimer* m_bluetoothUpdateTimer;
	bool m_bluetoothTimeOverride = false;
	int m_bluetoothTimeValue;
	TimedCubeMoveSequence m_solveMoves;

	void updateText();

private slots:
	void updateBluetoothSolve();

public:
	TimerWidget(QWidget* parent);
	~TimerWidget();

	void buttonDown();
	void buttonUp();

	bool running() const { return m_state == TIMER_RUNNING; }
	int value();

	void setFontSize(int size);

	void disable();
	void enable();

	void setBluetoothCube(const std::shared_ptr<BluetoothCube>& cube);
	void readyForBluetoothSolve();

	const TimedCubeMoveSequence& solveMoves() const { return m_solveMoves; }

signals:
	void started();
	void completed();
	void aboutToStart();
	void reset();
};
