#pragma once

#include <QtWidgets/QWidget>
#include "history.h"
#include "scramblewidget.h"

class SolveWidget: public QWidget
{
	Solve m_solve;
	ScrambleWidget* m_scramble;
	QLabel* m_timer;

public:
	SolveWidget(const Solve& solve);

	QString solveDetailsText();
	static QString solveTimeText(const Solve& solve);
};
