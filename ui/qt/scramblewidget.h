#pragma once

#include <QtWidgets/QLabel>
#include <QtCore/QTimer>
#include <QtCore/QThread>
#include <QtCore/QMutex>
#include <QtCore/QWaitCondition>
#include "cubecommon.h"
#include "scramble.h"
#include "bluetoothcube.h"

class QtRandomSource: public RandomSource
{
public:
	virtual int Next(int range) override;
};

class RescrambleThread: public QThread
{
	Q_OBJECT

	bool m_requestPending = false;
	CubeMoveSequence m_inScramble;
	Cube3x3 m_initialState;
	CubeMoveSequence m_outScramble;

	QMutex m_mutex;
	QWaitCondition m_cond;
	volatile bool m_running = true;

protected:
	virtual void run() override;

public:
	RescrambleThread(QObject* owner);
	void stop();

	void requestRescramble(const Cube3x3& state, const CubeMoveSequence& scramble);
	CubeMoveSequence rescramble();

signals:
	void rescrambleGenerated();
};

class ScrambleWidget: public QLabel
{
	Q_OBJECT

	CubeMoveSequence m_scramble, m_originalScramble;
	size_t m_maxMoveCount = 30;
	bool m_reserveVerticalSpace = true;

	std::shared_ptr<BluetoothCube> m_bluetoothCube;
	std::shared_ptr<BluetoothCubeClient> m_bluetoothCubeClient;
	int m_currentScrambleMove = -1;
	int m_currentScrambleSubMove = 0;
	CubeMoveSequence m_fixMoves;
	QTimer* m_bluetoothUpdateTimer;

	RescrambleThread* m_thread;

	void updateText();
	void updateScrambleStateForHalfMove(CubeMove move, CubeMove forward, CubeMove back);
	void updateScrambleStateForMove(CubeMove currentMove, CubeMove scrambleMove);

private slots:
	void updateBluetoothScramble();
	void rescrambleGenerated();

public:
	ScrambleWidget(QWidget* parent);
	~ScrambleWidget();

	void setScramble(const CubeMoveSequence& scramble);
	void invalidateScramble();
	void setFontSize(int size);
	void setMaxMoveCount(size_t count);
	void setReserveVerticalSpace(bool enable);

	void setBluetoothCube(const std::shared_ptr<BluetoothCube>& cube);

signals:
	void scrambleComplete();
};
