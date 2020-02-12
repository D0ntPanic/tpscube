#pragma once

#include <QtWidgets/QLabel>
#include <QtCore/QTimer>
#include <chrono>

enum TimerState
{
	TIMER_STOPPED,
	TIMER_READY_TO_START,
	TIMER_RUNNING
};

class TimerWidget: public QLabel
{
	Q_OBJECT

	TimerState m_state = TIMER_STOPPED;
	std::chrono::time_point<std::chrono::steady_clock> m_startTime, m_endTime, m_prepareTime;
	int m_fontSize = 80;
	QTimer* m_updateTimer;
	bool m_enabled = true;

	void updateText();

public:
	TimerWidget(QWidget* parent);

	void buttonDown();
	void buttonUp();

	bool running() const { return m_state == TIMER_RUNNING; }
	int value();

	void setFontSize(int size);

	void disable();
	void enable();

signals:
	void started();
	void completed();
	void aboutToStart();
	void reset();
};
