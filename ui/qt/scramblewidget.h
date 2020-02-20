#pragma once

#include <QtWidgets/QLabel>
#include <QtCore/QTimer>
#include "cubecommon.h"
#include "scramble.h"
#include "bluetoothcube.h"

class QtRandomSource: public RandomSource
{
public:
	virtual int Next(int range) override;
};

class ScrambleWidget: public QLabel
{
	Q_OBJECT

	CubeMoveSequence m_scramble;
	size_t m_maxMoveCount = 30;
	bool m_reserveVerticalSpace = true;

	std::shared_ptr<BluetoothCube> m_bluetoothCube;
	std::shared_ptr<BluetoothCubeClient> m_bluetoothCubeClient;
	int m_currentScrambleMove = -1;
	int m_currentScrambleSubMove = 0;
	CubeMoveSequence m_fixMoves;
	QTimer* m_bluetoothUpdateTimer;

	void updateText();
	void updateScrambleStateForHalfMove(CubeMove move, CubeMove forward, CubeMove back);
	void updateScrambleStateForMove(CubeMove currentMove, CubeMove scrambleMove);

private slots:
	void updateBluetoothScramble();

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
