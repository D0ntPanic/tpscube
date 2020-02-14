#pragma once

#include <QtWidgets/QWidget>

#include "history.h"
#include "scramblewidget.h"
#include "timerwidget.h"

class SolveWidget: public QWidget
{
	Solve m_solve;
	ScrambleWidget* m_scramble;
	QLabel* m_timer;

public:
	SolveWidget(QWidget* parent, const Solve& solve);

	QString solveDetailsText();
};
