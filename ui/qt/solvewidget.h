#pragma once

#include <QtWidgets/QWidget>
#include "history.h"
#include "scramblewidget.h"
#include "cube3x3widget.h"

class SolveWidget: public QWidget
{
	Solve m_solve;
	ScrambleWidget* m_scramble;
	QLabel* m_timer;
	Cube3x3Widget* m_cube;

public:
	SolveWidget(const Solve& solve, bool fullDetails = false);

	QString solveDetailsText();
	static QString solveTimeText(const Solve& solve);
};
