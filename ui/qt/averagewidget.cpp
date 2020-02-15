#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGridLayout>
#include <QtCore/QDateTime>
#include "averagewidget.h"
#include "solvedialog.h"
#include "sessionwidget.h"
#include "historymode.h"
#include "utilwidgets.h"
#include "theme.h"

using namespace std;


AverageWidget::AverageWidget(const vector<Solve>& solves, bool fullDetails): m_solves(solves)
{
	QVBoxLayout* layout = new QVBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	Heading* headingLabel = new Heading(QString::asprintf("Average of %d", (int)solves.size()));
	layout->addWidget(headingLabel);

	vector<int> solveTimes;
	int highest = -1;
	int lowest = -1;
	for (size_t i = 0; i < m_solves.size(); i++)
	{
		if (m_solves[i].ok)
		{
			if (lowest == -1)
				lowest = (int)i;
			if (highest == -1)
				highest = (int)i;
			if ((m_solves[i].time < m_solves[lowest].time))
				lowest = (int)i;
			if ((m_solves[i].time > m_solves[highest].time))
				highest = (int)i;
			solveTimes.push_back(m_solves[i].time);
		}
		else
		{
			highest = (int)i;
			solveTimes.push_back(-1);
		}
	}
	m_lowest = (size_t)lowest;
	m_highest = (size_t)highest;

	int avg = Session::avgOf(solveTimes);

	QString timeText;
	if (avg != -1)
	{
		int hs = (avg + 5) / 10;
		int minutes = hs / 6000;
		int seconds = (hs / 100) % 60;
		hs %= 100;

		if (minutes > 0)
		{
			timeText = QString::asprintf("<span style='font-size:%fpt'>%d:%02d</span>"
				"<span style='font-size:%fpt'>.%02d</span>", relativeFontSize(2.0f),
				minutes, seconds, relativeFontSize(1.75f), hs);
		}
		else
		{
			timeText = QString::asprintf("<span style='font-size:%fpt'>%d</span>"
				"<span style='font-size:%fpt'>.%02d</span>", relativeFontSize(2.0f),
				seconds, relativeFontSize(1.75f), hs);
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
	if (avg != -1)
		pal.setColor(QPalette::WindowText, Theme::green);
	else
		pal.setColor(QPalette::WindowText, Theme::red);
	m_timer->setPalette(pal);
	layout->addWidget(m_timer);

	QGridLayout* timeLayout = new QGridLayout();
	timeLayout->setContentsMargins(0, 0, 0, 0);
	timeLayout->setColumnStretch(0, 0);
	timeLayout->setColumnStretch(1, 0);
	timeLayout->setColumnStretch(2, 0);
	timeLayout->setColumnStretch(3, 0);
	timeLayout->setColumnStretch(4, 0);
	timeLayout->setSpacing(0);

	for (size_t i = 0; i < m_solves.size(); i++)
	{
		Solve solve = m_solves[i];

		ThinLabel* num = new ThinLabel(QString::asprintf("%d.  ", (int)i + 1));
		QPalette pal = palette();
		pal.setColor(QPalette::WindowText, Theme::disabled);
		num->setPalette(pal);
		timeLayout->addWidget(num, (int)i, 0, Qt::AlignRight);

		ClickableLabel* timeLabel;
		function<void()> showSolve = [=]() {
			SolveDialog* dlg = new SolveDialog(solve);
			dlg->show();
		};
		if (solve.ok)
		{
			timeLabel = new ClickableLabel(SessionWidget::stringForTime(solve.time),
				Theme::content, Theme::blue, showSolve);
		}
		else
		{
			timeLabel = new ClickableLabel("DNF", Theme::red, Theme::blue, showSolve);
		}
		if ((i == m_highest) || (i == m_lowest))
		{
			timeLabel->setText("(" + timeLabel->text() + ")");
			timeLabel->setColors(Theme::selectionLight, Theme::blue);
		}
		timeLabel->setCursor(Qt::PointingHandCursor);
		timeLayout->addWidget(timeLabel, (int)i, 1, Qt::AlignRight);

		QLabel* penalty;
		if (solve.ok && solve.penalty)
			penalty = new QLabel(QString::asprintf(" [+%d]", solve.penalty / 1000));
		else
			penalty = new QLabel("");
		pal.setColor(QPalette::WindowText, Theme::red);
		penalty->setPalette(pal);
		timeLayout->addWidget(penalty, (int)i, 2, Qt::AlignLeft);

		if (fullDetails)
		{
			ClickableLabel* scramble = new ClickableLabel("   " +
				QString::fromStdString(solve.scramble.ToString()) + "   ",
				Theme::content, Theme::blue, showSolve);
			scramble->setFont(fontOfRelativeSize(1.0f, QFont::Thin));
			scramble->setCursor(Qt::PointingHandCursor);
			timeLayout->addWidget(scramble, (int)i, 3, Qt::AlignLeft);
		}

		ClickableLabel* timeOfSolve = new ClickableLabel("   " + HistoryMode::shortStringForDate(solve.created),
			Theme::content, Theme::blue, showSolve);
		timeOfSolve->setFont(fontOfRelativeSize(1.0f, QFont::Thin));
		timeOfSolve->setCursor(Qt::PointingHandCursor);
		timeLayout->addWidget(timeOfSolve, (int)i, fullDetails ? 4 : 3, Qt::AlignLeft);
	}

	layout->addLayout(timeLayout);
	setLayout(layout);
}


QString AverageWidget::averageDetailsText()
{
	vector<int> solveTimes;
	for (auto& i : m_solves)
	{
		if (i.ok)
			solveTimes.push_back(i.time);
		else
			solveTimes.push_back(-1);
	}

	int avg = Session::avgOf(solveTimes);

	QString result = QString::asprintf("Average of %d: ", (int)m_solves.size());
	if (avg != -1)
	{
		int hs = (avg + 5) / 10;
		int minutes = hs / 6000;
		int seconds = (hs / 100) % 60;
		hs %= 100;

		if (minutes > 0)
			result += QString::asprintf("%d:%02d.%02d\n", minutes, seconds, hs);
		else
			result += QString::asprintf("%d.%02d\n", seconds, hs);
	}
	else
	{
		result += "DNF\n";
	}

	size_t countWidth = QString::asprintf("%d", (int)m_solves.size() + 1).size();
	size_t scrambleWidth = 0;
	size_t solveWidth = 0;
	size_t penaltyWidth = 0;
	for (auto& i : m_solves)
	{
		size_t curScrambleWidth = i.scramble.ToString().size();
		if (curScrambleWidth > scrambleWidth)
			scrambleWidth = curScrambleWidth;

		size_t curSolveWidth = SolveWidget::solveTimeText(i).size() + 2;
		if (curSolveWidth > solveWidth)
			solveWidth = curSolveWidth;

		size_t curPenaltyWidth = 0;
		if (i.ok && i.penalty)
			curPenaltyWidth = QString::asprintf(" [+%d]", i.penalty / 1000).size();
		if (curPenaltyWidth > penaltyWidth)
			penaltyWidth = curPenaltyWidth;
	}

	for (size_t i = 0; i < m_solves.size(); i++)
	{
		result += QString::asprintf("%*d. ", (int)countWidth, (int)i + 1);

		QString solveTimeStr = SolveWidget::solveTimeText(m_solves[i]);
		if ((i == m_highest) || (i == m_lowest))
			solveTimeStr = "(" + solveTimeStr + ")";
		QString penaltyStr = "";
		if (m_solves[i].ok && m_solves[i].penalty)
			penaltyStr = QString::asprintf(" [+%d]", m_solves[i].penalty / 1000);
		result += QString::asprintf("%*s%*s", (int)solveWidth, solveTimeStr.toStdString().c_str(),
			(int)penaltyWidth, penaltyStr.toStdString().c_str());

		result += QString::asprintf("   %-*s   ", (int)scrambleWidth, m_solves[i].scramble.ToString().c_str());
		result += "@" + QDateTime::fromSecsSinceEpoch(m_solves[i].created).toString(Qt::DateFormat::TextDate);
		result += "\n";
	}
	return result;
}
