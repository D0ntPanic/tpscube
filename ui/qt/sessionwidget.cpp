#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QApplication>
#include <QtWidgets/QScrollArea>
#include <QtWidgets/QScrollBar>
#include <QtWidgets/QMessageBox>
#include <QtWidgets/QMenu>
#include <QtGui/QFontMetrics>
#include <algorithm>
#include "sessionwidget.h"

using namespace std;


SessionWidget::SessionWidget(QWidget* parent): QWidget(parent)
{
	setBackgroundRole(QPalette::AlternateBase);
	setAutoFillBackground(true);

	QVBoxLayout* layout = new QVBoxLayout();
	layout->addWidget(new Heading("Session"));

	QGridLayout* currentSessionLayout = new QGridLayout();
	currentSessionLayout->addWidget(new ThinLabel("Last ao5: "), 1, 0);
	m_averageOf5 = new QLabel();
	currentSessionLayout->addWidget(m_averageOf5, 1, 1, Qt::AlignRight);
	currentSessionLayout->addWidget(new ThinLabel("Last ao12: "), 2, 0);
	m_averageOf12 = new QLabel();
	currentSessionLayout->addWidget(m_averageOf12, 2, 1, Qt::AlignRight);
	currentSessionLayout->addWidget(new ThinLabel("Session avg: "), 3, 0);
	m_sessionAverage = new QLabel();
	currentSessionLayout->addWidget(m_sessionAverage, 3, 1, Qt::AlignRight);
	currentSessionLayout->addWidget(new ThinLabel("Best solve: "), 4, 0);
	m_bestSolve = new QLabel();
	currentSessionLayout->addWidget(m_bestSolve, 4, 1, Qt::AlignRight);
	currentSessionLayout->addWidget(new ThinLabel("Best ao5: "), 5, 0);
	m_bestAverageOf5 = new QLabel();
	currentSessionLayout->addWidget(m_bestAverageOf5, 5, 1, Qt::AlignRight);
	currentSessionLayout->setColumnStretch(0, 0);
	currentSessionLayout->setColumnStretch(1, 1);
	QFontMetrics metrics(QApplication::font());
	currentSessionLayout->setColumnMinimumWidth(1, metrics.boundingRect("0:00.00").width());
	currentSessionLayout->setSpacing(0);
	layout->addLayout(currentSessionLayout);
	QHBoxLayout* resetLayout = new QHBoxLayout();
	resetLayout->addStretch(1);
	ClickableLabel* resetButton = new ClickableLabel("↺ New Session", Theme::disabled, Theme::red,
		[this]() { resetSession(); });
	resetButton->setFont(fontOfRelativeSize(0.9f, QFont::Light));
	resetButton->setToolTip("Reset current session. Solves in the current session will be saved in the solve history.");
	resetLayout->addWidget(resetButton);
	layout->addLayout(resetLayout);

	layout->addWidget(new Heading("Solves"));

	m_noSolves = new QLabel("No solves in this session");
	m_noSolves->setFont(fontOfRelativeSize(0.9f, QFont::Light, true));
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, Theme::disabled);
	m_noSolves->setPalette(pal);
	layout->addWidget(m_noSolves);

	QScrollArea* solveScroll = new QScrollArea();
	solveScroll->setWidgetResizable(true);
	solveScroll->setFrameStyle(QFrame::NoFrame);
	QWidget* solveWidget = new QWidget();
	solveWidget->setContentsMargins(0, 0, 0, 0);
	QVBoxLayout* solveWidgetLayout = new QVBoxLayout();
	solveWidgetLayout->setContentsMargins(0, 0, 0, 0);
	m_solveLayout = new QGridLayout();
	m_solveLayout->setContentsMargins(0, 0, 0, 0);
	m_solveLayout->setColumnStretch(0, 0);
	m_solveLayout->setColumnStretch(1, 1);
	m_solveLayout->setColumnMinimumWidth(0, metrics.boundingRect("000.").width());
	m_solveLayout->setColumnMinimumWidth(1, metrics.boundingRect("0:00.00").width());
	m_solveLayout->setColumnMinimumWidth(2, metrics.boundingRect(" (+2)").width());
	m_solveLayout->setColumnMinimumWidth(3, metrics.boundingRect(" ≡  ").width());
	m_solveLayout->setColumnMinimumWidth(4, metrics.boundingRect(" ×  ").width());
	m_solveLayout->setSpacing(0);
	solveWidgetLayout->addLayout(m_solveLayout);
	solveWidgetLayout->addStretch(1);
	solveWidget->setLayout(solveWidgetLayout);
	solveScroll->setWidget(solveWidget);
	solveScroll->setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
	solveScroll->verticalScrollBar()->setStyleSheet("QScrollBar { width:0px; }");
	layout->addWidget(solveScroll, 1);

	setLayout(layout);
	updateHistory();
}


QString SessionWidget::stringForTime(int ms)
{
	if (ms == -1)
		return "-";

	int hs = (ms + 5) / 10;
	int minutes = hs / 6000;
	int seconds = (hs / 100) % 60;
	hs %= 100;

	if (minutes > 0)
	{
		return QString::asprintf("<span style='font-size:%fpt'>%d:%02d</span>"
			"<span style='font-size:%fpt'>.%02d</span>", relativeFontSize(1.0f),
			minutes, seconds, relativeFontSize(0.75f), hs);
	}
	else
	{
		return QString::asprintf("<span style='font-size:%fpt'>%d</span>"
			"<span style='font-size:%fpt'>.%02d</span>", relativeFontSize(1.0f),
			seconds, relativeFontSize(0.75f), hs);
	}
}


QString SessionWidget::stringForSolveTime(const Solve& solve)
{
	if (!solve.ok)
		return "DNF";
	return stringForTime(solve.time);
}


void SessionWidget::options(int row)
{
	int solveIndex = (int)History::instance.activeSession->solves.size() - (row + 1);
	if (solveIndex < 0)
		return;

	Solve& solve = History::instance.activeSession->solves[solveIndex];

	QMenu menu;
	QAction* solveOK = new QAction("Solve OK");
	solveOK->setCheckable(true);
	solveOK->setChecked(solve.ok && (solve.penalty == 0));
	QAction* penalty = new QAction("2 Second Penalty");
	penalty->setCheckable(true);
	penalty->setChecked(solve.ok && (solve.penalty == 2000));
	QAction* dnf = new QAction("Did Not Finish");
	dnf->setCheckable(true);
	dnf->setChecked(!solve.ok);
	menu.addAction(solveOK);
	menu.addAction(penalty);
	menu.addAction(dnf);
	QAction* clicked = menu.exec(QCursor::pos() + QPoint(2, 2));
	if (clicked == solveOK)
	{
		solve.ok = true;
		solve.time -= solve.penalty;
		solve.penalty = 0;
		solve.update.id = History::instance.idGenerator->GenerateId();
		solve.update.date = time(NULL);
		solve.dirty = true;
	}
	else if (clicked == penalty)
	{
		solve.ok = true;
		solve.time -= solve.penalty;
		solve.penalty = 2000;
		solve.time += solve.penalty;
		solve.update.id = History::instance.idGenerator->GenerateId();
		solve.update.date = time(NULL);
		solve.dirty = true;
	}
	else if (clicked == dnf)
	{
		solve.ok = false;
		solve.update.id = History::instance.idGenerator->GenerateId();
		solve.update.date = time(NULL);
		solve.dirty = true;
	}
	else
	{
		return;
	}

	History::instance.activeSession->update.id = History::instance.idGenerator->GenerateId();
	History::instance.activeSession->update.date = time(NULL);
	History::instance.activeSession->dirty = true;
	History::instance.UpdateDatabaseForSession(History::instance.activeSession);
	updateHistory();
}


void SessionWidget::remove(int row)
{
	int solveIndex = (int)History::instance.activeSession->solves.size() - (row + 1);
	if (solveIndex < 0)
		return;

	const Solve& solve = History::instance.activeSession->solves[solveIndex];
	QString msg;
	if (solve.ok)
		msg = QString("Delete solve with time of ") + stringForSolveTime(solve) + QString("?");
	else
		msg = "Delete DNF solve?";
	if (QMessageBox::critical(this, "Delete Solve", msg, QMessageBox::Yes, QMessageBox::No) != QMessageBox::Yes)
		return;

	History::instance.activeSession->solves.erase(History::instance.activeSession->solves.begin() + solveIndex);
	History::instance.activeSession->dirty = true;
	History::instance.UpdateDatabaseForSession(History::instance.activeSession);

	if (History::instance.activeSession->solves.size() == 0)
		History::instance.DeleteSession(History::instance.activeSession);

	updateHistory();
}


void SessionWidget::resetSession()
{
	History::instance.ResetSession();
	updateHistory();
}


void SessionWidget::updateHistory()
{
	if (History::instance.activeSession)
	{
		m_averageOf5->setText(stringForTime(History::instance.activeSession->avgOfLast(5, false)));
		m_averageOf12->setText(stringForTime(History::instance.activeSession->avgOfLast(12, false)));
		if (History::instance.activeSession->solves.size() != 0)
			m_sessionAverage->setText(stringForTime(History::instance.activeSession->sessionAvg()));
		else
			m_sessionAverage->setText("-");
		int best = History::instance.activeSession->bestSolve();
		m_bestSolve->setText(stringForTime(best));
		m_bestAverageOf5->setText(stringForTime(History::instance.activeSession->bestAvgOf(5)));

		int allTimeBest = -1;
		for (auto& i : History::instance.sessions)
		{
			if (i->type != History::instance.activeSession->type)
				continue;
			int cur = i->bestSolve();
			if ((cur != -1) && ((allTimeBest == -1) || (cur < allTimeBest)))
				allTimeBest = cur;
		}

		while (m_solveLabels.size() < (int)History::instance.activeSession->solves.size())
		{
			int row = m_solveLabels.size();
			SolveLabels labels;
			labels.num = new ThinLabel("");
			labels.time = new QLabel("");
			labels.penalty = new QLabel("");
			labels.options = new ClickableLabel(" ≡", Theme::selection, Theme::blue,
				[=]() { options(row); });
			labels.options->setToolTip("Set solve penalties");
			labels.remove = new ClickableLabel(" ×", Theme::selection, Theme::red,
				[=]() { remove(row); });
			labels.remove->setToolTip("Delete solve");

			QPalette pal = palette();
			pal.setColor(QPalette::WindowText, Theme::red);
			labels.penalty->setPalette(pal);
			pal.setColor(QPalette::WindowText, Theme::disabled);
			labels.num->setPalette(pal);

			m_solveLayout->addWidget(labels.num, row, 0);
			m_solveLayout->addWidget(labels.time, row, 1, Qt::AlignRight);
			m_solveLayout->addWidget(labels.penalty, row, 2, Qt::AlignRight);
			m_solveLayout->addWidget(labels.options, row, 3, Qt::AlignRight);
			m_solveLayout->addWidget(labels.remove, row, 4, Qt::AlignRight);
			m_solveLabels.push_back(labels);
		}

		while (m_solveLabels.size() > (int)History::instance.activeSession->solves.size())
		{
			m_solveLabels[m_solveLabels.size() - 1].num->deleteLater();
			m_solveLabels[m_solveLabels.size() - 1].time->deleteLater();
			m_solveLabels[m_solveLabels.size() - 1].penalty->deleteLater();
			m_solveLabels[m_solveLabels.size() - 1].options->deleteLater();
			m_solveLabels[m_solveLabels.size() - 1].remove->deleteLater();
			m_solveLabels.erase(m_solveLabels.end() - 1);
		}

		for (int i = 0; i < m_solveLabels.size(); i++)
		{
			int solveIdx = (int)History::instance.activeSession->solves.size() - (i + 1);
			const Solve& solve = History::instance.activeSession->solves[solveIdx];
			m_solveLabels[i].num->setText(QString::asprintf("%d.", solveIdx + 1));
			m_solveLabels[i].time->setText(stringForSolveTime(solve));
			m_solveLabels[i].penalty->setText((solve.ok && solve.penalty) ?
				QString::asprintf("<span style='font-size:%fpt'>(+%d)</span> ",
					relativeFontSize(0.75f), solve.penalty / 1000) : "");

			QPalette pal = palette();
			if (solve.ok && ((int)solve.time == allTimeBest))
				pal.setColor(QPalette::WindowText, Theme::orange);
			else if (solve.ok && ((int)solve.time == best))
				pal.setColor(QPalette::WindowText, Theme::green);
			else if (!solve.ok)
				pal.setColor(QPalette::WindowText, Theme::red);
			m_solveLabels[i].time->setPalette(pal);
		}

		m_noSolves->setVisible(History::instance.activeSession->solves.size() == 0);
	}
	else
	{
		m_averageOf5->setText("-");
		m_averageOf12->setText("-");
		m_sessionAverage->setText("-");
		m_bestSolve->setText("-");
		m_bestAverageOf5->setText("-");
		m_noSolves->setVisible(true);

		for (auto& i : m_solveLabels)
		{
			i.num->deleteLater();
			i.time->deleteLater();
			i.penalty->deleteLater();
			i.options->deleteLater();
			i.remove->deleteLater();
		}
		m_solveLabels.clear();
	}
}
