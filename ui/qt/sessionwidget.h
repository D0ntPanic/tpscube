#pragma once

#include <QtWidgets/QWidget>
#include <QtWidgets/QLabel>
#include <QtWidgets/QGridLayout>
#include "history.h"
#include "utilwidgets.h"

struct SolveLabels
{
	QLabel* num;
	ClickableLabel* time;
	QLabel* penalty;
	ClickableLabel* options;
	ClickableLabel* remove;
};

class SessionWidget: public QWidget
{
	Q_OBJECT

	QLabel* m_averageOf5;
	QLabel* m_averageOf12;
	QLabel* m_sessionAverage;
	QLabel* m_bestSolve;
	QLabel* m_bestAverageOf5;
	QLabel* m_noSolves;

	QGridLayout* m_solveLayout;
	QList<SolveLabels> m_solveLabels;

	void showSolve(int row);
	void options(int row);
	void remove(int row);

private slots:
	void resetSession();

public:
	SessionWidget(QWidget* parent);

	void updateHistory();

	static QString stringForTime(int ms);
	static QString stringForSolveTime(const Solve& solve);
};
