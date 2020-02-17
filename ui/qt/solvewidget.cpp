#include <QtWidgets/QVBoxLayout>
#include <QtCore/QDateTime>
#include "solvewidget.h"
#include "historymode.h"
#include "utilwidgets.h"
#include "theme.h"

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
		m_cube->cube().Apply(m_solve.scramble);
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
