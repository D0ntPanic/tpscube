#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QScrollArea>
#include <QtWidgets/QScrollBar>
#include <vector>
#include "graphmode.h"

using namespace std;


GraphMode::GraphMode(QWidget *parent) : QWidget(parent)
{
	setBackgroundRole(QPalette::Base);
	setAutoFillBackground(true);

	QHBoxLayout *layout = new QHBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	QScrollArea *leftAreaScroll = new QScrollArea();
	leftAreaScroll->setWidgetResizable(true);
	leftAreaScroll->setFrameStyle(QFrame::NoFrame);

	QWidget *leftArea = new QWidget();
	leftArea->setContentsMargins(0, 0, 0, 0);
	leftArea->setBackgroundRole(QPalette::AlternateBase);
	leftArea->setAutoFillBackground(true);

	QVBoxLayout *leftAreaLayout = new QVBoxLayout();

	leftAreaLayout->addWidget(new Heading("Phase"));

	m_entireSolve = new ModeLabel("Entire solve", [this]() { showEntireSolve(); });
	m_entireSolve->sizeToLargest();
	m_entireSolve->setToolTip("Graph statistics for the entire solve");
	leftAreaLayout->addWidget(m_entireSolve);
	m_solveBreakdown = new ModeLabel("Solve breakdown", [this]() { showSolveBreakdown(); });
	m_solveBreakdown->sizeToLargest();
	m_solveBreakdown->setToolTip("Show a stacked graph for all phases of the solve");
	leftAreaLayout->addWidget(m_solveBreakdown);
	m_cross = new ModeLabel("Cross", [this]() { showCross(); });
	m_cross->sizeToLargest();
	m_cross->setToolTip("Graph statistics for the cross phase of the solve");
	leftAreaLayout->addWidget(m_cross);
	m_f2l = new ModeLabel("F2L", [this]() { showF2L(); });
	m_f2l->sizeToLargest();
	m_f2l->setToolTip("Graph statistics for the first two layers phase of the solve");
	leftAreaLayout->addWidget(m_f2l);
	m_oll = new ModeLabel("OLL", [this]() { showOLL(); });
	m_oll->sizeToLargest();
	m_oll->setToolTip("Graph statistics for the orient last layer phase of the solve");
	leftAreaLayout->addWidget(m_oll);
	m_pll = new ModeLabel("PLL", [this]() { showPLL(); });
	m_pll->sizeToLargest();
	m_pll->setToolTip("Graph statistics for the permute last layer phase of the solve");
	leftAreaLayout->addWidget(m_pll);
	m_entireSolve->setActive(true);

	leftAreaLayout->addWidget(new Heading("Statistic"));

	m_time = new ModeLabel("Time", [this]() { showTime(); });
	m_time->sizeToLargest();
	m_time->setToolTip("Show solve time for the selected phase(s)");
	leftAreaLayout->addWidget(m_time);
	m_moves = new ModeLabel("Moves", [this]() { showMoves(); });
	m_moves->sizeToLargest();
	m_moves->setToolTip("Show move count for the selected phase(s)");
	leftAreaLayout->addWidget(m_moves);
	m_idle = new ModeLabel("Idle", [this]() { showIdle(); });
	m_idle->sizeToLargest();
	m_idle->setToolTip("Show time not spent performing moves (recognition time) for the selected phase(s)");
	leftAreaLayout->addWidget(m_idle);
	m_etps = new ModeLabel("eTPS", [this]() { showETPS(); });
	m_etps->sizeToLargest();
	m_etps->setToolTip("Show execution turns per second (excludes idle time) for the selected phase(s)");
	leftAreaLayout->addWidget(m_etps);
	m_tps = new ModeLabel("TPS", [this]() { showTPS(); });
	m_tps->sizeToLargest();
	m_tps->setToolTip("Show turns per second (includes idle time) for the selected phase(s)");
	leftAreaLayout->addWidget(m_tps);
	m_time->setActive(true);

	leftAreaLayout->addWidget(new Heading("Time period"));

	m_entireHistory = new ModeLabel("Entire history", [this]() { showEntireHistory(); });
	m_entireHistory->sizeToLargest();
	m_entireHistory->setToolTip("Show all recorded solves");
	leftAreaLayout->addWidget(m_entireHistory);
	m_thisYear = new ModeLabel("This year", [this]() { showThisYear(); });
	m_thisYear->sizeToLargest();
	m_thisYear->setToolTip("Show solves from the last year");
	leftAreaLayout->addWidget(m_thisYear);
	m_thisQuarter = new ModeLabel("This quarter", [this]() { showThisQuarter(); });
	m_thisQuarter->sizeToLargest();
	m_thisQuarter->setToolTip("Show solves from the last three months");
	leftAreaLayout->addWidget(m_thisQuarter);
	m_thisMonth = new ModeLabel("This month", [this]() { showThisMonth(); });
	m_thisMonth->sizeToLargest();
	m_thisMonth->setToolTip("Show solves from the last month");
	leftAreaLayout->addWidget(m_thisMonth);
	m_thisWeek = new ModeLabel("This week", [this]() { showThisWeek(); });
	m_thisWeek->sizeToLargest();
	m_thisWeek->setToolTip("Show solves from the last week");
	leftAreaLayout->addWidget(m_thisWeek);
	m_entireHistory->setActive(true);

	leftAreaLayout->addStretch(1);
	leftArea->setLayout(leftAreaLayout);

	leftAreaScroll->setWidget(leftArea);
	leftAreaScroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
	leftAreaScroll->verticalScrollBar()->setStyleSheet("QScrollBar { width:0px; }");
	layout->addWidget(leftAreaScroll);

	QVBoxLayout *rightAreaLayout = new QVBoxLayout();
	m_graph = new GraphWidget();
	rightAreaLayout->addWidget(m_graph, 1);
	layout->addLayout(rightAreaLayout, 1);

	setLayout(layout);
}


void GraphMode::showEntireSolve()
{
	m_entireSolve->setActive(true);
	m_solveBreakdown->setActive(false);
	m_cross->setActive(false);
	m_f2l->setActive(false);
	m_oll->setActive(false);
	m_pll->setActive(false);
	m_phase = GRAPHPHASE_ALL;
	updateGraph();
}


void GraphMode::showSolveBreakdown()
{
	m_entireSolve->setActive(false);
	m_solveBreakdown->setActive(true);
	m_cross->setActive(false);
	m_f2l->setActive(false);
	m_oll->setActive(false);
	m_pll->setActive(false);
	m_phase = GRAPHPHASE_BREAKDOWN;
	updateGraph();
}


void GraphMode::showCross()
{
	m_entireSolve->setActive(false);
	m_solveBreakdown->setActive(false);
	m_cross->setActive(true);
	m_f2l->setActive(false);
	m_oll->setActive(false);
	m_pll->setActive(false);
	m_phase = GRAPHPHASE_CROSS;
	updateGraph();
}


void GraphMode::showF2L()
{
	m_entireSolve->setActive(false);
	m_solveBreakdown->setActive(false);
	m_cross->setActive(false);
	m_f2l->setActive(true);
	m_oll->setActive(false);
	m_pll->setActive(false);
	m_phase = GRAPHPHASE_F2L;
	updateGraph();
}


void GraphMode::showOLL()
{
	m_entireSolve->setActive(false);
	m_solveBreakdown->setActive(false);
	m_cross->setActive(false);
	m_f2l->setActive(false);
	m_oll->setActive(true);
	m_pll->setActive(false);
	m_phase = GRAPHPHASE_OLL;
	updateGraph();
}


void GraphMode::showPLL()
{
	m_entireSolve->setActive(false);
	m_solveBreakdown->setActive(false);
	m_cross->setActive(false);
	m_f2l->setActive(false);
	m_oll->setActive(false);
	m_pll->setActive(true);
	m_phase = GRAPHPHASE_PLL;
	updateGraph();
}


void GraphMode::showTime()
{
	m_time->setActive(true);
	m_moves->setActive(false);
	m_idle->setActive(false);
	m_etps->setActive(false);
	m_tps->setActive(false);
	m_stat = GRAPHSTAT_TIME;
	updateGraph();
}


void GraphMode::showMoves()
{
	m_time->setActive(false);
	m_moves->setActive(true);
	m_idle->setActive(false);
	m_etps->setActive(false);
	m_tps->setActive(false);
	m_stat = GRAPHSTAT_MOVES;
	updateGraph();
}


void GraphMode::showIdle()
{
	m_time->setActive(false);
	m_moves->setActive(false);
	m_idle->setActive(true);
	m_etps->setActive(false);
	m_tps->setActive(false);
	m_stat = GRAPHSTAT_IDLE;
	updateGraph();
}


void GraphMode::showETPS()
{
	m_time->setActive(false);
	m_moves->setActive(false);
	m_idle->setActive(false);
	m_etps->setActive(true);
	m_tps->setActive(false);
	m_stat = GRAPHSTAT_ETPS;
	updateGraph();
}


void GraphMode::showTPS()
{
	m_time->setActive(false);
	m_moves->setActive(false);
	m_idle->setActive(false);
	m_etps->setActive(false);
	m_tps->setActive(true);
	m_stat = GRAPHSTAT_TPS;
	updateGraph();
}


void GraphMode::showEntireHistory()
{
	m_entireHistory->setActive(true);
	m_thisYear->setActive(false);
	m_thisQuarter->setActive(false);
	m_thisMonth->setActive(false);
	m_thisWeek->setActive(false);
	m_timePeriod = -1;
	updateGraph();
}


void GraphMode::showThisYear()
{
	m_entireHistory->setActive(false);
	m_thisYear->setActive(true);
	m_thisQuarter->setActive(false);
	m_thisMonth->setActive(false);
	m_thisWeek->setActive(false);
	m_timePeriod = 365 * 24 * 3600;
	updateGraph();
}


void GraphMode::showThisQuarter()
{
	m_entireHistory->setActive(false);
	m_thisYear->setActive(false);
	m_thisQuarter->setActive(true);
	m_thisMonth->setActive(false);
	m_thisWeek->setActive(false);
	m_timePeriod = 90 * 24 * 3600;
	updateGraph();
}


void GraphMode::showThisMonth()
{
	m_entireHistory->setActive(false);
	m_thisYear->setActive(false);
	m_thisQuarter->setActive(false);
	m_thisMonth->setActive(true);
	m_thisWeek->setActive(false);
	m_timePeriod = 30 * 24 * 3600;
	updateGraph();
}


void GraphMode::showThisWeek()
{
	m_entireHistory->setActive(false);
	m_thisYear->setActive(false);
	m_thisQuarter->setActive(false);
	m_thisMonth->setActive(false);
	m_thisWeek->setActive(true);
	m_timePeriod = 7 * 24 * 3600;
	updateGraph();
}


float GraphMode::valueForPhase(float* values, GraphPhase phase)
{
	switch (phase)
	{
	case GRAPHPHASE_CROSS:
		return values[0];
	case GRAPHPHASE_F2L:
		return values[1];
	case GRAPHPHASE_OLL:
		return values[2];
	case GRAPHPHASE_PLL:
		return values[3];
	default:
		return values[0] + values[1] + values[2] + values[3];
	}
}


void GraphMode::timeForAllPhases(const Solve& solve, float* values)
{
	values[0] = (float)solve.crossTime / 1000.0f;
	values[1] = (float)(solve.f2lPairTimes[3] - solve.crossTime) / 1000.0f;
	values[2] = (float)(solve.ollFinishTime - solve.f2lPairTimes[3]) / 1000.0f;
	values[3] = (float)((solve.time - solve.penalty) - solve.ollFinishTime) / 1000.0f;
}


float GraphMode::timeForPhase(const Solve& solve, GraphPhase phase)
{
	float values[4];
	timeForAllPhases(solve, values);
	return valueForPhase(values, phase);
}


void GraphMode::movesForAllPhases(const Solve& solve, float* values)
{
	DetailedSplitTimes details = solve.GenerateDetailedSplitTimes();
	values[0] = (float)details.cross.moveCount;
	values[1] = (float)(details.f2lPair[0].moveCount + details.f2lPair[1].moveCount +
		details.f2lPair[2].moveCount + details.f2lPair[3].moveCount);
	values[2] = (float)(details.ollCross.moveCount + details.ollFinish.moveCount);
	values[3] = (float)(details.pllCorner.moveCount + details.pllFinish.moveCount);
}


float GraphMode::movesForPhase(const Solve& solve, GraphPhase phase)
{
	float values[4];
	movesForAllPhases(solve, values);
	return valueForPhase(values, phase);
}


void GraphMode::idleForAllPhases(const Solve& solve, float* values)
{
	DetailedSplitTimes details = solve.GenerateDetailedSplitTimes();
	values[0] = (float)(details.cross.firstMoveTime - details.cross.phaseStartTime) / 1000.0f;
	values[1] = (float)((details.f2lPair[0].firstMoveTime - details.f2lPair[0].phaseStartTime) +
		(details.f2lPair[1].firstMoveTime - details.f2lPair[1].phaseStartTime) +
		(details.f2lPair[2].firstMoveTime - details.f2lPair[2].phaseStartTime) +
		(details.f2lPair[3].firstMoveTime - details.f2lPair[3].phaseStartTime)) / 1000.0f;
	values[2] = (float)((details.ollCross.firstMoveTime - details.ollCross.phaseStartTime) +
		(details.ollFinish.firstMoveTime - details.ollFinish.phaseStartTime)) / 1000.0f;
	values[3] = (float)((details.pllCorner.firstMoveTime - details.pllCorner.phaseStartTime) +
		(details.pllFinish.firstMoveTime - details.pllFinish.phaseStartTime)) / 1000.0f;
}


float GraphMode::idleForPhase(const Solve& solve, GraphPhase phase)
{
	float values[4];
	idleForAllPhases(solve, values);
	return valueForPhase(values, phase);
}


void GraphMode::etpsForAllPhases(const Solve& solve, float* values)
{
	DetailedSplitTimes details = solve.GenerateDetailedSplitTimes();
	uint32_t crossTime = details.cross.finishTime - details.cross.firstMoveTime;
	uint32_t f2lTime = (details.f2lPair[0].finishTime - details.f2lPair[0].firstMoveTime) +
		(details.f2lPair[1].finishTime - details.f2lPair[1].firstMoveTime) +
		(details.f2lPair[2].finishTime - details.f2lPair[2].firstMoveTime) +
		(details.f2lPair[3].finishTime - details.f2lPair[3].firstMoveTime);
	uint32_t ollTime = (details.ollCross.finishTime - details.ollCross.firstMoveTime) +
		(details.ollFinish.finishTime - details.ollFinish.firstMoveTime);
	uint32_t pllTime = (details.pllCorner.finishTime - details.pllCorner.firstMoveTime) +
		(details.pllFinish.finishTime - details.pllFinish.firstMoveTime);
	size_t crossMoves = (details.cross.moveCount ? (details.cross.moveCount - 1) : 0);
	size_t f2lMoves = (details.f2lPair[0].moveCount ? (details.f2lPair[0].moveCount - 1) : 0) +
		(details.f2lPair[1].moveCount ? (details.f2lPair[1].moveCount - 1) : 0) +
		(details.f2lPair[2].moveCount ? (details.f2lPair[2].moveCount - 1) : 0) +
		(details.f2lPair[3].moveCount ? (details.f2lPair[3].moveCount - 1) : 0);
	size_t ollMoves = (details.ollCross.moveCount ? (details.ollCross.moveCount - 1) : 0) +
		(details.ollFinish.moveCount ? (details.ollFinish.moveCount - 1) : 0);
	size_t pllMoves = (details.pllCorner.moveCount ? (details.pllCorner.moveCount - 1) : 0) +
		(details.pllFinish.moveCount ? (details.pllFinish.moveCount - 1) : 0);
	if (crossTime == 0)
		values[0] = 0;
	else
		values[0] = (float)crossMoves / ((float)crossTime / 1000.0f);
	if (f2lTime == 0)
		values[1] = 0;
	else
		values[1] = (float)f2lMoves / ((float)f2lTime / 1000.0f);
	if (ollTime == 0)
		values[2] = 0;
	else
		values[2] = (float)ollMoves / ((float)ollTime / 1000.0f);
	if (pllTime == 0)
		values[3] = 0;
	else
		values[3] = (float)pllMoves / ((float)pllTime / 1000.0f);
}


float GraphMode::etpsForPhase(const Solve& solve, GraphPhase phase)
{
	float values[4];
	etpsForAllPhases(solve, values);
	return valueForPhase(values, phase);
}


void GraphMode::tpsForAllPhases(const Solve& solve, float* values)
{
	DetailedSplitTimes details = solve.GenerateDetailedSplitTimes();
	uint32_t crossTime = details.cross.finishTime - details.cross.phaseStartTime;
	uint32_t f2lTime = (details.f2lPair[0].finishTime - details.f2lPair[0].phaseStartTime) +
		(details.f2lPair[1].finishTime - details.f2lPair[1].phaseStartTime) +
		(details.f2lPair[2].finishTime - details.f2lPair[2].phaseStartTime) +
		(details.f2lPair[3].finishTime - details.f2lPair[3].phaseStartTime);
	uint32_t ollTime = (details.ollCross.finishTime - details.ollCross.phaseStartTime) +
		(details.ollFinish.finishTime - details.ollFinish.phaseStartTime);
	uint32_t pllTime = (details.pllCorner.finishTime - details.pllCorner.phaseStartTime) +
		(details.pllFinish.finishTime - details.pllFinish.phaseStartTime);
	size_t crossMoves = (details.cross.moveCount ? (details.cross.moveCount - 1) : 0);
	size_t f2lMoves = (details.f2lPair[0].moveCount ? (details.f2lPair[0].moveCount - 1) : 0) +
		(details.f2lPair[1].moveCount ? (details.f2lPair[1].moveCount - 1) : 0) +
		(details.f2lPair[2].moveCount ? (details.f2lPair[2].moveCount - 1) : 0) +
		(details.f2lPair[3].moveCount ? (details.f2lPair[3].moveCount - 1) : 0);
	size_t ollMoves = (details.ollCross.moveCount ? (details.ollCross.moveCount - 1) : 0) +
		(details.ollFinish.moveCount ? (details.ollFinish.moveCount - 1) : 0);
	size_t pllMoves = (details.pllCorner.moveCount ? (details.pllCorner.moveCount - 1) : 0) +
		(details.pllFinish.moveCount ? (details.pllFinish.moveCount - 1) : 0);
	if (crossTime == 0)
		values[0] = 0;
	else
		values[0] = (float)crossMoves / ((float)crossTime / 1000.0f);
	if (f2lTime == 0)
		values[1] = 0;
	else
		values[1] = (float)f2lMoves / ((float)f2lTime / 1000.0f);
	if (ollTime == 0)
		values[2] = 0;
	else
		values[2] = (float)ollMoves / ((float)ollTime / 1000.0f);
	if (pllTime == 0)
		values[3] = 0;
	else
		values[3] = (float)pllMoves / ((float)pllTime / 1000.0f);
}


float GraphMode::tpsForPhase(const Solve& solve, GraphPhase phase)
{
	float values[4];
	tpsForAllPhases(solve, values);
	return valueForPhase(values, phase);
}


void GraphMode::updateGraph()
{
	time_t minTime = 0;
	if (m_timePeriod != -1)
		minTime = time(NULL) - m_timePeriod;

	bool splitsRequired = false;
	bool fullMovesRequired = false;
	int solvesWithoutSplits = 0;
	int solvesWithoutMoves = 0;
	if (m_phase != GRAPHPHASE_ALL)
		splitsRequired = true;
	if (m_stat != GRAPHSTAT_TIME)
		fullMovesRequired = true;

	vector<GraphPlot> plots;
	for (auto &i : History::instance.sessions)
	{
		for (auto &j : i->solves)
		{
			if (!j.ok)
				continue;
			if (j.created < minTime)
				continue;

			if (!j.crossTime || !j.f2lPairTimes[3] || !j.ollFinishTime)
			{
				solvesWithoutSplits++;
				if (splitsRequired)
					continue;
			}

			if (j.solveMoves.moves.size() == 0)
			{
				solvesWithoutMoves++;
				if (fullMovesRequired)
					continue;
			}

			GraphPlot plot;
			bool valid = true;
			plot.date = j.created;
			switch (m_stat)
			{
			case GRAPHSTAT_TIME:
				if (m_phase == GRAPHPHASE_ALL)
					plot.value[0] = (float)j.time / 1000.0f;
				else if (m_phase == GRAPHPHASE_BREAKDOWN)
					timeForAllPhases(j, plot.value);
				else
					plot.value[0] = timeForPhase(j, m_phase);
				break;
			case GRAPHSTAT_MOVES:
				if (m_phase == GRAPHPHASE_ALL)
					plot.value[0] = (float)j.solveMoves.GetOuterTurnCount();
				else if (m_phase == GRAPHPHASE_BREAKDOWN)
					movesForAllPhases(j, plot.value);
				else
					plot.value[0] = movesForPhase(j, m_phase);
				break;
			case GRAPHSTAT_IDLE:
				if (m_phase == GRAPHPHASE_BREAKDOWN)
					idleForAllPhases(j, plot.value);
				else
					plot.value[0] = idleForPhase(j, m_phase);
				break;
			case GRAPHSTAT_ETPS:
				if (m_phase == GRAPHPHASE_BREAKDOWN)
				{
					etpsForAllPhases(j, plot.value);
					if ((plot.value[0] == 0) || (plot.value[1] == 0) ||
						(plot.value[2] == 0) || (plot.value[3] == 0))
					{
						valid = false;
						break;
					}
					plot.value[0] /= 4.0f;
					plot.value[1] /= 4.0f;
					plot.value[2] /= 4.0f;
					plot.value[3] /= 4.0f;
				}
				else
				{
					plot.value[0] = etpsForPhase(j, m_phase);
				}
				break;
			case GRAPHSTAT_TPS:
				if (m_phase == GRAPHPHASE_ALL)
				{
					size_t moves = j.solveMoves.GetOuterTurnCount();
					if ((j.time - j.penalty) == 0)
					{
						valid = false;
						break;
					}
					plot.value[0] = (float)(moves - 1) / ((float)(j.time - j.penalty) / 1000.0f);
				}
				else if (m_phase == GRAPHPHASE_BREAKDOWN)
				{
					tpsForAllPhases(j, plot.value);
					if ((plot.value[0] == 0) || (plot.value[1] == 0) ||
						(plot.value[2] == 0) || (plot.value[3] == 0))
					{
						valid = false;
						break;
					}
					plot.value[0] /= 4.0f;
					plot.value[1] /= 4.0f;
					plot.value[2] /= 4.0f;
					plot.value[3] /= 4.0f;
				}
				else
				{
					plot.value[0] = tpsForPhase(j, m_phase);
				}
				break;
			default:
				plot.value[0] = 0;
				break;
			}

			if (!valid)
				continue;
			plots.push_back(plot);
		}
	}

	if (splitsRequired && (solvesWithoutSplits >= 2))
		m_graph->setMessage("This graph requires solves with split timing.");
	else if (fullMovesRequired && (solvesWithoutMoves >= 2))
		m_graph->setMessage("This graph requires solves with a Bluetooth cube.");
	else
		m_graph->setMessage("Not enough data to display this graph.");

	if (m_phase == GRAPHPHASE_BREAKDOWN)
		m_graph->setPlots(plots, 4);
	else
		m_graph->setPlots(plots, 1);
}
