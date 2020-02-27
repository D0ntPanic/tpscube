#pragma once

#include <QtWidgets/QWidget>
#include "utilwidgets.h"
#include "graphwidget.h"
#include "history.h"

enum GraphPhase
{
	GRAPHPHASE_ALL,
	GRAPHPHASE_BREAKDOWN,
	GRAPHPHASE_CROSS,
	GRAPHPHASE_F2L,
	GRAPHPHASE_OLL,
	GRAPHPHASE_PLL
};

enum GraphStatistic
{
	GRAPHSTAT_TIME,
	GRAPHSTAT_MOVES,
	GRAPHSTAT_IDLE,
	GRAPHSTAT_ETPS,
	GRAPHSTAT_TPS
};

class GraphMode: public QWidget
{
	ModeLabel* m_entireSolve;
	ModeLabel* m_solveBreakdown;
	ModeLabel* m_cross;
	ModeLabel* m_f2l;
	ModeLabel* m_oll;
	ModeLabel* m_pll;

	ModeLabel* m_time;
	ModeLabel* m_moves;
	ModeLabel* m_idle;
	ModeLabel* m_etps;
	ModeLabel* m_tps;

	ModeLabel* m_entireHistory;
	ModeLabel* m_thisYear;
	ModeLabel* m_thisQuarter;
	ModeLabel* m_thisMonth;
	ModeLabel* m_thisWeek;

	GraphPhase m_phase = GRAPHPHASE_ALL;
	GraphStatistic m_stat = GRAPHSTAT_TIME;
	int64_t m_timePeriod = -1;

	GraphWidget* m_graph;

	void showEntireSolve();
	void showSolveBreakdown();
	void showCross();
	void showF2L();
	void showOLL();
	void showPLL();

	void showTime();
	void showMoves();
	void showIdle();
	void showETPS();
	void showTPS();

	void showEntireHistory();
	void showThisYear();
	void showThisQuarter();
	void showThisMonth();
	void showThisWeek();

	float valueForPhase(float* values, GraphPhase phase);
	void timeForAllPhases(const Solve& solve, float* values);
	float timeForPhase(const Solve& solve, GraphPhase phase);
	void movesForAllPhases(const Solve& solve, float* values);
	float movesForPhase(const Solve& solve, GraphPhase phase);
	void idleForAllPhases(const Solve& solve, float* values);
	float idleForPhase(const Solve& solve, GraphPhase phase);
	void etpsForAllPhases(const Solve& solve, float* values);
	float etpsForPhase(const Solve& solve, GraphPhase phase);
	void tpsForAllPhases(const Solve& solve, float* values);
	float tpsForPhase(const Solve& solve, GraphPhase phase);

public:
	GraphMode(QWidget* parent);

	void updateGraph();
};
