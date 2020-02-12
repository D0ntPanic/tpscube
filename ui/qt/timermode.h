#pragma once

#include <QtWidgets/QWidget>
#include <QtWidgets/QVBoxLayout>
#include <QtCore/QThread>
#include <QtCore/QMutex>
#include <QtCore/QWaitCondition>
#include "sessionwidget.h"
#include "scramblewidget.h"
#include "timerwidget.h"
#include "history.h"

class ScrambleThread: public QThread
{
	Q_OBJECT

	bool m_requestPending = false;
	std::shared_ptr<Scrambler> m_scrambler;
	CubeMoveSequence m_result;

	QMutex m_mutex;
	QWaitCondition m_cond;
	volatile bool m_running = true;

protected:
	virtual void run() override;

public:
	ScrambleThread(QObject* owner);
	void stop();

	void requestScramble(const std::shared_ptr<Scrambler>& scrambler);
	CubeMoveSequence scramble();

signals:
	void scrambleGenerated();
};

class TimerMode: public QWidget
{
	Q_OBJECT

	QVBoxLayout* m_rightAreaLayout;

	SessionWidget* m_session;

	ScrambleWidget* m_scrambleWidget;
	int m_scrambleStretch;
	TimerWidget* m_timer;

	SolveType m_solveType = SOLVE_3X3X3;
	std::shared_ptr<Scrambler> m_scrambler;
	bool m_scrambleValid = false;
	CubeMoveSequence m_currentScramble;
	bool m_pendingScrambleValid = false;
	CubeMoveSequence m_pendingScramble;
	ScrambleThread* m_scrambleThread;

	void newScramble();
	void updateFontSizes();

private slots:
	void solveStarting();
	void solveStopping();
	void solveComplete();
	void scrambleGenerated();

protected:
	virtual void resizeEvent(QResizeEvent* event);

public:
	TimerMode(QWidget* parent);
	~TimerMode();

	void buttonDown();
	void buttonUp();

	bool running() const;

	void updateHistory();

signals:
	void timerStarting();
	void timerStopping();
};
