#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGridLayout>
#include <QtCore/QDateTime>
#include "solvewidget.h"
#include "historymode.h"
#include "utilwidgets.h"
#include "theme.h"
#include "sessionwidget.h"
#include "solvebarwidget.h"

using namespace std;


SolveWidget::SolveWidget(const Solve& solve, bool fullDetails): m_solve(solve)
{
	QVBoxLayout* layout = new QVBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	Heading* dateLabel = new Heading("Solved " + HistoryMode::stringForDate(solve.created));
	layout->addWidget(dateLabel);

	m_scramble = new ScrambleWidget(this);
	m_scramble->setReserveVerticalSpace(false);
	m_scramble->setScramble(m_solve.scramble);
	m_scramble->setFontSize(relativeFontSize(1.25f));
	layout->addWidget(m_scramble);

	if (fullDetails)
	{
		layout->addSpacing(8);

		m_cube = new Cube3x3Widget();
		m_cube->applyImmediate(m_solve.scramble);
		layout->addWidget(m_cube, 1);
	}

	layout->addSpacing(8);

	QString penaltyStr;
	if (m_solve.ok && (m_solve.penalty > 0))
		penaltyStr = QString::asprintf("  (+%d)", (int)(m_solve.penalty / 1000));

	QString timeText;
	if (m_solve.ok)
	{
		int hs = (m_solve.time + 5) / 10;
		int minutes = hs / 6000;
		int seconds = (hs / 100) % 60;
		hs %= 100;

		if (minutes > 0)
		{
			timeText = QString::asprintf("<span style='font-size:%fpt'>%d:%02d</span>"
				"<span style='font-size:%fpt'>.%02d%s</span>", relativeFontSize(2.0f),
				minutes, seconds, relativeFontSize(1.75f), hs, penaltyStr.toStdString().c_str());
		}
		else
		{
			timeText = QString::asprintf("<span style='font-size:%fpt'>%d</span>"
				"<span style='font-size:%fpt'>.%02d%s</span>", relativeFontSize(2.0f),
				seconds, relativeFontSize(1.75f), hs, penaltyStr.toStdString().c_str());
		}
	}
	else
	{
		timeText = QString::asprintf("<span style='font-size:%fpt'>DNF</span>",
			relativeFontSize(2.0f));
	}

	m_timer = new QLabel(timeText);
	m_timer->setAlignment(Qt::AlignVCenter | Qt::AlignCenter);
	QPalette pal(m_timer->palette());
	if (m_solve.ok)
		pal.setColor(QPalette::WindowText, Theme::green);
	else
		pal.setColor(QPalette::WindowText, Theme::red);
	m_timer->setPalette(pal);
	layout->addWidget(m_timer);

	if (solve.ok && solve.crossTime && solve.f2lPairTimes[3] && solve.ollFinishTime)
	{
		SolveBarWidget* bar = new SolveBarWidget(solve);
		layout->addWidget(bar);

		QGridLayout* splitLayout = new QGridLayout();
		splitLayout->setHorizontalSpacing(16);
		splitLayout->setVerticalSpacing(0);

		QLabel* crossLabel = new QLabel("Cross");
		crossLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
		crossLabel->setAlignment(Qt::AlignCenter);
		crossLabel->setToolTip("Time spent in the cross phase");
		splitLayout->addWidget(crossLabel, 0, 1);
		QLabel* crossTime = new QLabel(SessionWidget::stringForTime(solve.crossTime));
		crossTime->setAlignment(Qt::AlignCenter);
		crossTime->setToolTip("Time spent in the cross phase");
		splitLayout->addWidget(crossTime, 1, 1);
		QLabel* f2lLabel = new QLabel("F2L");
		f2lLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
		f2lLabel->setAlignment(Qt::AlignCenter);
		f2lLabel->setToolTip("Time spent in the first two layers phase");
		splitLayout->addWidget(f2lLabel, 0, 2);
		QLabel* f2lTime = new QLabel(SessionWidget::stringForTime(solve.f2lPairTimes[3] - solve.crossTime));
		f2lTime->setAlignment(Qt::AlignCenter);
		f2lTime->setToolTip("Time spent in the first two layers phase");
		splitLayout->addWidget(f2lTime, 1, 2);
		QLabel* ollLabel = new QLabel("OLL");
		ollLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
		ollLabel->setAlignment(Qt::AlignCenter);
		ollLabel->setToolTip("Time spent in the orient last layer phase");
		splitLayout->addWidget(ollLabel, 0, 3);
		QLabel* ollTime = new QLabel(SessionWidget::stringForTime(solve.ollFinishTime - solve.f2lPairTimes[3]));
		ollTime->setAlignment(Qt::AlignCenter);
		ollTime->setToolTip("Time spent in the orient last layer phase");
		splitLayout->addWidget(ollTime, 1, 3);
		QLabel* pllLabel = new QLabel("PLL");
		pllLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
		pllLabel->setAlignment(Qt::AlignCenter);
		pllLabel->setToolTip("Time spent in the permute last layer phase");
		splitLayout->addWidget(pllLabel, 0, 4);
		QLabel* pllTime = new QLabel(SessionWidget::stringForTime((solve.time - solve.penalty) - solve.ollFinishTime));
		pllTime->setAlignment(Qt::AlignCenter);
		pllTime->setToolTip("Time spent in the permute last layer phase");
		splitLayout->addWidget(pllTime, 1, 4);
		int columns = 4;

		if (solve.solveMoves.moves.size() != 0)
		{
			DetailedSplitTimes splits = solve.GenerateDetailedSplitTimes();
			int idle = (splits.cross.firstMoveTime - splits.cross.phaseStartTime) +
				(splits.f2lPair[0].firstMoveTime - splits.f2lPair[0].phaseStartTime) +
				(splits.f2lPair[1].firstMoveTime - splits.f2lPair[1].phaseStartTime) +
				(splits.f2lPair[2].firstMoveTime - splits.f2lPair[2].phaseStartTime) +
				(splits.f2lPair[3].firstMoveTime - splits.f2lPair[3].phaseStartTime) +
				(splits.ollCross.firstMoveTime - splits.ollCross.phaseStartTime) +
				(splits.ollFinish.firstMoveTime - splits.ollFinish.phaseStartTime) +
				(splits.pllCorner.firstMoveTime - splits.pllCorner.phaseStartTime) +
				(splits.pllFinish.firstMoveTime - splits.pllFinish.phaseStartTime);

			size_t moves = solve.solveMoves.GetOuterTurnCount();
			splitLayout->setColumnMinimumWidth(5, 16);
			QLabel* movesLabel = new QLabel("Moves");
			movesLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			movesLabel->setAlignment(Qt::AlignCenter);
			movesLabel->setToolTip("Number of moves (outer turn metric)");
			splitLayout->addWidget(movesLabel, 0, 6);
			QLabel* moveCount = new QLabel(QString::asprintf("<span style='font-size:%fpt'>%d</span>",
				relativeFontSize(1.0f), (int)moves));
			moveCount->setAlignment(Qt::AlignCenter);
			moveCount->setToolTip("Number of moves (outer turn metric)");
			splitLayout->addWidget(moveCount, 1, 6);
			QLabel* idleLabel = new QLabel("Idle");
			idleLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			idleLabel->setAlignment(Qt::AlignCenter);
			idleLabel->setToolTip("Time spent not performing moves (recognition time)");
			splitLayout->addWidget(idleLabel, 0, 7);
			QLabel* idleTime = new QLabel(SessionWidget::stringForTime(idle));
			idleTime->setAlignment(Qt::AlignCenter);
			idleTime->setToolTip("Time spent not performing moves (recognition time)");
			splitLayout->addWidget(idleTime, 1, 7);
			QLabel* tpsLabel = new QLabel("TPS");
			tpsLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			tpsLabel->setAlignment(Qt::AlignCenter);
			tpsLabel->setToolTip("Turns per second (excluding idle time)");
			splitLayout->addWidget(tpsLabel, 0, 8);
			QLabel* tps = new QLabel(QString::asprintf("<span style='font-size:%fpt'>%.2f</span>",
				relativeFontSize(1.0f), (float)moves / ((float)(solve.time - (idle + solve.penalty)) / 1000.0f)));
			tps->setAlignment(Qt::AlignCenter);
			tps->setToolTip("Turns per second (excluding idle time)");
			splitLayout->addWidget(tps, 1, 8);
			columns = 8;
		}

		splitLayout->setColumnStretch(0, 1);
		splitLayout->setColumnStretch(columns + 1, 1);
		layout->addLayout(splitLayout);
	}

	setLayout(layout);
}


QString SolveWidget::solveDetailsText()
{
	QString result = solveTimeText(m_solve);
	result += "   ";
	result += QString::fromStdString(m_solve.scramble.ToString());
	result += "   @";
	result += QDateTime::fromSecsSinceEpoch(m_solve.created).toString(Qt::DateFormat::TextDate);
	return result;
}


QString SolveWidget::solveTimeText(const Solve& solve)
{
	if (solve.ok)
	{
		int hs = (solve.time + 5) / 10;
		int minutes = hs / 6000;
		int seconds = (hs / 100) % 60;
		hs %= 100;

		if (minutes > 0)
			return QString::asprintf("%d:%02d.%02d", minutes, seconds, hs);
		else
			return QString::asprintf("%d.%02d", seconds, hs);
	}
	return "DNF";
}
