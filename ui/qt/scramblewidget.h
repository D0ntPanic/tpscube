#pragma once

#include <QtWidgets/QLabel>
#include "cubecommon.h"
#include "scramble.h"

class QtRandomSource: public RandomSource
{
public:
	virtual int Next(int range) override;
};

class ScrambleWidget: public QLabel
{
	CubeMoveSequence m_scramble;
	size_t m_maxMoveCount = 30;

	void updateText();

public:
	ScrambleWidget(QWidget* parent);

	void setScramble(const CubeMoveSequence& scramble);
	void invalidateScramble();
	void setFontSize(int size);
	void setMaxMoveCount(size_t count);
};
