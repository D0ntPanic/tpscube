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

	ClickableLabel* m_averageOf5;
	ClickableLabel* m_averageOf12;
	QLabel* m_sessionAverage;
	ClickableLabel* m_bestSolveLabel;
	Solve m_bestSolve;
	ClickableLabel* m_bestAverageOf5;
	int m_bestAverageOf5Index = -1;
	QLabel* m_noSolves;

	QGridLayout* m_solveLayout;
	QList<SolveLabels> m_solveLabels;

	void showSolve(int row);
	void showSolveTooltip(int row);
	void showLastAvgOf5();
	void showLastAvgOf5Tooltip();
	void showLastAvgOf12();
	void showLastAvgOf12Tooltip();
	void showBestSolve();
	void showBestSolveTooltip();
	void showBestAvgOf5();
	void showBestAvgOf5Tooltip();
	void options(int row);
	void remove(int row);

private slots:
	void resetSession();

public:
	SessionWidget(QWidget* parent);

	void updateHistory();

	static QString stringForTime(int ms, float scale = 1.0f);
	static QString stringForSolveTime(const Solve& solve, float scale = 1.0f);
};
