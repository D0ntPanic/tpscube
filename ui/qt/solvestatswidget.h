#pragma once

#include <QtWidgets/QWidget>
#include <QtWidgets/QLabel>
#include <QtWidgets/QGridLayout>
#include "history.h"
#include "solvebarwidget.h"

class SolveStatsWidget: public QWidget
{
	Solve m_solve;
	float m_scale = 1.0f;
	bool m_showSolveBar = true;

	QGridLayout* m_splitLayout;
	SolveBarWidget* m_solveBar;
	QLabel* m_crossLabel;
	QLabel* m_crossTime;
	QLabel* m_f2lLabel;
	QLabel* m_f2lTime;
	QLabel* m_ollLabel;
	QLabel* m_ollTime;
	QLabel* m_pllLabel;
	QLabel* m_pllTime;
	QLabel* m_movesLabel;
	QLabel* m_moveCount;
	QLabel* m_idleLabel;
	QLabel* m_idleTime;
	QLabel* m_etpsLabel;
	QLabel* m_etps;
	QLabel* m_tpsLabel;
	QLabel* m_tps;

	void updateStats();

public:
	SolveStatsWidget();
	void setSolve(const Solve& solve);
	void setScale(float scale);
	void setSolveBarEnabled(bool enabled);
	void invalidateSolve();
	bool hasValidStats();
};
