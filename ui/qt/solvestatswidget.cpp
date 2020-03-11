#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include "solvestatswidget.h"
#include "sessionwidget.h"


SolveStatsWidget::SolveStatsWidget()
{
	QVBoxLayout* layout = new QVBoxLayout();
	m_solveBar = new SolveBarWidget();
	layout->addWidget(m_solveBar);

	m_splitLayout = new QGridLayout();
	m_splitLayout->setHorizontalSpacing(16);
	m_splitLayout->setVerticalSpacing(0);

	m_crossLabel = new QLabel("Cross");
	m_crossLabel->setAlignment(Qt::AlignCenter);
	m_crossLabel->setToolTip("Time spent in the cross phase");
	m_splitLayout->addWidget(m_crossLabel, 0, 0);

	m_crossTime = new QLabel();
	m_crossTime->setAlignment(Qt::AlignCenter);
	m_crossTime->setToolTip("Time spent in the cross phase");
	m_splitLayout->addWidget(m_crossTime, 1, 0);

	m_f2lLabel = new QLabel("F2L");
	m_f2lLabel->setAlignment(Qt::AlignCenter);
	m_f2lLabel->setToolTip("Time spent in the first two layers phase");
	m_splitLayout->addWidget(m_f2lLabel, 0, 1);

	m_f2lTime = new QLabel();
	m_f2lTime->setAlignment(Qt::AlignCenter);
	m_f2lTime->setToolTip("Time spent in the first two layers phase");
	m_splitLayout->addWidget(m_f2lTime, 1, 1);

	m_ollLabel = new QLabel("OLL");
	m_ollLabel->setAlignment(Qt::AlignCenter);
	m_ollLabel->setToolTip("Time spent in the orient last layer phase");
	m_splitLayout->addWidget(m_ollLabel, 0, 2);

	m_ollTime = new QLabel();
	m_ollTime->setAlignment(Qt::AlignCenter);
	m_ollTime->setToolTip("Time spent in the orient last layer phase");
	m_splitLayout->addWidget(m_ollTime, 1, 2);

	m_pllLabel = new QLabel("PLL");
	m_pllLabel->setAlignment(Qt::AlignCenter);
	m_pllLabel->setToolTip("Time spent in the permute last layer phase");
	m_splitLayout->addWidget(m_pllLabel, 0, 3);

	m_pllTime = new QLabel();
	m_pllTime->setAlignment(Qt::AlignCenter);
	m_pllTime->setToolTip("Time spent in the permute last layer phase");
	m_splitLayout->addWidget(m_pllTime, 1, 3);

	m_movesLabel = new QLabel("Moves");
	m_movesLabel->setAlignment(Qt::AlignCenter);
	m_movesLabel->setToolTip("Number of moves (outer turn metric)");
	m_splitLayout->addWidget(m_movesLabel, 0, 5);

	m_moveCount = new QLabel();
	m_moveCount->setAlignment(Qt::AlignCenter);
	m_moveCount->setToolTip("Number of moves (outer turn metric)");
	m_splitLayout->addWidget(m_moveCount, 1, 5);

	m_idleLabel = new QLabel("Idle");
	m_idleLabel->setAlignment(Qt::AlignCenter);
	m_idleLabel->setToolTip("Time spent not performing moves (recognition time)");
	m_splitLayout->addWidget(m_idleLabel, 0, 6);

	m_idleTime = new QLabel();
	m_idleTime->setAlignment(Qt::AlignCenter);
	m_idleTime->setToolTip("Time spent not performing moves (recognition time)");
	m_splitLayout->addWidget(m_idleTime, 1, 6);

	m_etpsLabel = new QLabel("eTPS");
	m_etpsLabel->setAlignment(Qt::AlignCenter);
	m_etpsLabel->setToolTip("Execution turns per second (excludes idle time)");
	m_splitLayout->addWidget(m_etpsLabel, 0, 7);

	m_etps = new QLabel();
	m_etps->setAlignment(Qt::AlignCenter);
	m_etps->setToolTip("Execution turns per second (excludes idle time)");
	m_splitLayout->addWidget(m_etps, 1, 7);

	m_tpsLabel = new QLabel("TPS");
	m_tpsLabel->setAlignment(Qt::AlignCenter);
	m_tpsLabel->setToolTip("Turns per second (includes idle time)");
	m_splitLayout->addWidget(m_tpsLabel, 0, 8);

	m_tps = new QLabel();
	m_tps->setAlignment(Qt::AlignCenter);
	m_tps->setToolTip("Turns per second (includes idle time)");
	m_splitLayout->addWidget(m_tps, 1, 8);

	QHBoxLayout* centerLayout = new QHBoxLayout();
	centerLayout->setContentsMargins(0, 0, 0, 0);
	centerLayout->addStretch(1);
	centerLayout->addLayout(m_splitLayout);
	centerLayout->addStretch(1);
	layout->addLayout(centerLayout);
	setLayout(layout);

	updateStats();
}


void SolveStatsWidget::setSolve(const Solve& solve)
{
	m_solve = solve;
	updateStats();
}


void SolveStatsWidget::setScale(float scale)
{
	m_scale = scale;
	updateStats();
}


void SolveStatsWidget::setSolveBarEnabled(bool enabled)
{
	m_showSolveBar = enabled;
}


void SolveStatsWidget::updateStats()
{
	m_splitLayout->setHorizontalSpacing((int)(16.0f * m_scale));

	if (m_solve.ok && m_solve.crossTime && m_solve.f2lPairTimes[3] && m_solve.ollFinishTime)
	{
		m_solveBar->setSolve(m_solve);
		m_solveBar->setVisible(m_showSolveBar);

		m_crossLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
		m_crossLabel->show();
		m_crossTime->setText(SessionWidget::stringForTime(m_solve.crossTime, m_scale));
		m_crossTime->show();
		m_f2lLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
		m_f2lLabel->show();
		m_f2lTime->setText(SessionWidget::stringForTime(m_solve.f2lPairTimes[3] - m_solve.crossTime, m_scale));
		m_f2lTime->show();
		m_ollLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
		m_ollLabel->show();
		m_ollTime->setText(SessionWidget::stringForTime(m_solve.ollFinishTime - m_solve.f2lPairTimes[3], m_scale));
		m_ollTime->show();
		m_pllLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
		m_pllLabel->show();
		m_pllTime->setText(SessionWidget::stringForTime((m_solve.time - m_solve.penalty) -
			m_solve.ollFinishTime, m_scale));
		m_pllTime->show();

		if (m_solve.solveMoves.moves.size() != 0)
		{
			DetailedSplitTimes splits = m_solve.GenerateDetailedSplitTimes();
			m_splitLayout->setColumnMinimumWidth(4, (int)(16.0f * m_scale));
			m_movesLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
			m_movesLabel->show();
			m_moveCount->setText(QString::asprintf("<span style='font-size:%fpt'>%d</span>",
				relativeFontSize(m_scale), (int)splits.moveCount));
			m_moveCount->show();
			m_idleLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
			m_idleLabel->show();
			m_idleTime->setText(SessionWidget::stringForTime(splits.idleTime, m_scale));
			m_idleTime->show();
			m_etpsLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
			m_etpsLabel->show();
			m_etps->setText(QString::asprintf("<span style='font-size:%fpt'>%.2f</span>",
				relativeFontSize(m_scale), splits.etps));
			m_etps->show();
			m_tpsLabel->setFont(fontOfRelativeSize(m_scale * 0.8f, QFont::Thin));
			m_tpsLabel->show();
			m_tps->setText(QString::asprintf("<span style='font-size:%fpt'>%.2f</span>",
				relativeFontSize(m_scale), splits.tps));
			m_tps->show();
		}
		else
		{
			m_splitLayout->setColumnMinimumWidth(4, 0);
			m_movesLabel->hide();
			m_moveCount->hide();
			m_idleLabel->hide();
			m_idleTime->hide();
			m_etpsLabel->hide();
			m_etps->hide();
			m_tpsLabel->hide();
			m_tps->hide();
		}
	}
	else
	{
		m_splitLayout->setColumnMinimumWidth(4, 0);
		m_solveBar->hide();
		m_crossLabel->hide();
		m_crossTime->hide();
		m_f2lLabel->hide();
		m_f2lTime->hide();
		m_ollLabel->hide();
		m_ollTime->hide();
		m_pllLabel->hide();
		m_pllTime->hide();
		m_movesLabel->hide();
		m_moveCount->hide();
		m_idleLabel->hide();
		m_idleTime->hide();
		m_etpsLabel->hide();
		m_etps->hide();
		m_tpsLabel->hide();
		m_tps->hide();
	}
}


void SolveStatsWidget::invalidateSolve()
{
	m_solve.ok = false;
	updateStats();
}


bool SolveStatsWidget::hasValidStats()
{
	return m_solve.ok && m_solve.crossTime && m_solve.f2lPairTimes[3] && m_solve.ollFinishTime;
}
