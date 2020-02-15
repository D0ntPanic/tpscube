#include <QtWidgets/QScrollBar>
#include <QtWidgets/QMessageBox>
#include <QtWidgets/QMenu>
#include <QtWidgets/QInputDialog>
#include <QtGui/QPainter>
#include <QtGui/QPaintEvent>
#include <QtGui/QFont>
#include <QtGui/QFontMetrics>
#include <QtCore/QDateTime>
#include "historymode.h"
#include "theme.h"
#include "sessionwidget.h"
#include "solvedialog.h"
#include "averagedialog.h"
#include "tooltip.h"

using namespace std;


void HistoryElement::move(int dx, int dy)
{
	m_rect = QRect(m_rect.x() + dx, m_rect.y() + dy, m_rect.width(), m_rect.height());
	for (auto& i : m_children)
		i->move(dx, dy);
}


HistoryAllTimeBestElement::HistoryAllTimeBestElement(const QString& title, int best):
	m_title(title), m_best(best)
{
}


QSize HistoryAllTimeBestElement::sizeHint() const
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont largestFont = fontOfRelativeSize(2.5f, QFont::Light);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics largestMetrics(largestFont);
	int titleWidth = lightMetrics.boundingRect(m_title).width();
	int bestWidth = largestMetrics.boundingRect("00:00.00").width();
	if (titleWidth > bestWidth)
		return QSize(titleWidth, lightMetrics.height() + largestMetrics.height());
	return QSize(bestWidth, lightMetrics.height() + largestMetrics.height());
}


void HistoryAllTimeBestElement::paint(QPainter& p, bool hovering)
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont largestFont = fontOfRelativeSize(2.5f, QFont::Light);
	QFont largeFont = fontOfRelativeSize(2.0f, QFont::Light);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics largestMetrics(largestFont);
	QFontMetrics largeMetrics(largeFont);

	int centerX = rect().x() + (rect().width() / 2);
	int headingWidth = lightMetrics.boundingRect(m_title).width();

	p.setFont(lightFont);
	p.setPen(Theme::content);
	p.drawText(centerX - (headingWidth / 2), rect().y() + lightMetrics.ascent(), m_title);
	int y = rect().y() + lightMetrics.height();

	int hs = (m_best + 5) / 10;
	int minutes = hs / 6000;
	int seconds = (hs / 100) % 60;
	hs %= 100;

	QString largeText, smallText;
	if (minutes > 0)
		largeText = QString::asprintf("%d:%02d", minutes, seconds);
	else
		largeText = QString::asprintf("%d", seconds);
	smallText = QString::asprintf(".%02d", hs);

	int largestWidth = largestMetrics.horizontalAdvance(largeText);
	int timeWidth = largestWidth + largeMetrics.horizontalAdvance(smallText);
	int timeX = centerX - (timeWidth / 2);
	p.setFont(largestFont);
	if (hovering)
		p.setPen(Theme::blue);
	else
		p.setPen(Theme::orange);
	p.drawText(timeX, y + largestMetrics.ascent(), largeText);
	p.setFont(largeFont);
	p.drawText(timeX + largestWidth, y + largestMetrics.ascent(), smallText);
}


HistoryAllTimeBestSolveElement::HistoryAllTimeBestSolveElement(const QString& title, int best, const Solve& solve):
	HistoryAllTimeBestElement(title, best), m_solve(solve)
{
}


bool HistoryAllTimeBestSolveElement::click(HistoryMode*, QMouseEvent*)
{
	SolveDialog* dlg = new SolveDialog(m_solve);
	dlg->show();
	return false;
}


void HistoryAllTimeBestSolveElement::tooltip(HistoryMode* parent) const
{
	SolveWidget* widget = new SolveWidget(m_solve);
	Tooltip* tooltip = new Tooltip(widget);
	tooltip->show(parent->viewport());
}


HistoryAllTimeBestAverageElement::HistoryAllTimeBestAverageElement(const QString& title, int best,
	const shared_ptr<Session>& session, int start, int size):
	HistoryAllTimeBestElement(title, best), m_session(session), m_start(start), m_size(size)
{
}


bool HistoryAllTimeBestAverageElement::click(HistoryMode*, QMouseEvent*)
{
	vector<Solve> solves;
	if ((!m_session) || ((m_start + m_size) > (int)m_session->solves.size()))
		return false;
	for (int i = 0; i < m_size; i++)
		solves.push_back(m_session->solves[m_start + i]);
	AverageDialog* dlg = new AverageDialog(solves);
	dlg->show();
	dlg->setFixedSize(dlg->size());
	return false;
}


void HistoryAllTimeBestAverageElement::tooltip(HistoryMode* parent) const
{
	vector<Solve> solves;
	if ((!m_session) || ((m_start + m_size) > (int)m_session->solves.size()))
		return;
	for (int i = 0; i < m_size; i++)
		solves.push_back(m_session->solves[m_start + i]);
	AverageWidget* widget = new AverageWidget(solves);
	Tooltip* tooltip = new Tooltip(widget);
	tooltip->show(parent->viewport());
}


HistorySessionElement::HistorySessionElement(const std::shared_ptr<Session>& session,
	int x, int y, int width, int* allTimeBestSolve): m_session(session),
	m_allTimeBestSolve(allTimeBestSolve)
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(smallFont);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();

	int height = headingFontHeight + 24;

	// Compute size of individual components of each solve display
	QString maxSolveNum = QString::asprintf("%d.    ", (int)m_session->solves.size());
	int solveNumWidth = lightMetrics.boundingRect(maxSolveNum).width();

	int maxSolveTime = 60000;
	for (auto& j : m_session->solves)
	{
		if (j.ok && ((int)j.time > maxSolveTime))
			maxSolveTime = (int)j.time;
	}
	int minutes = (maxSolveTime + 5) / 60000;
	QString maxTimeStr = QString::asprintf("%d:00.00", minutes);
	int timeWidth = normalMetrics.boundingRect(maxTimeStr).width();
	int penaltyWidth = smallMetrics.boundingRect("  (+2) ").width();
	int optionWidth = normalMetrics.boundingRect("  ≡ ").width();
	int removeWidth = normalMetrics.boundingRect("  × ").width();

	// Compute the number of columns that can be displayed for this session
	m_columnWidth = solveNumWidth + timeWidth + penaltyWidth +
		optionWidth + removeWidth + 16;
	m_columns = (width - 8) / m_columnWidth;
	if (m_columns < 1)
		m_columns = 1;

	// Compute the number of rows that are required and update size of session
	m_rows = ((int)m_session->solves.size() + m_columns - 1) / m_columns;
	height += m_rows * normalMetrics.height();

	// Compute best times for the session
	m_bestSolveTime = m_session->bestSolve(&m_bestSolve);
	m_bestAvgOf5 = m_session->bestAvgOf(5, &m_bestAvgOf5Start);
	m_bestAvgOf12 = m_session->bestAvgOf(12, &m_bestAvgOf12Start);
	m_sessionAvg = m_session->sessionAvg();

	height += normalMetrics.height() + 16;

	setRect(QRect(x, y, width, height));
}


vector<shared_ptr<HistoryElement>> HistorySessionElement::children()
{
	if (HistoryElement::children().size() != 0)
		return HistoryElement::children();

	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();
	int solveHeight = normalMetrics.height();

	int sessionOptionsWidth = normalMetrics.boundingRect("  ≡ ").width();
	shared_ptr<HistorySessionOptionsElement> sessionOptions =
		make_shared<HistorySessionOptionsElement>(m_session);
	sessionOptions->setRect(QRect(rect().x() + rect().width() - (sessionOptionsWidth + 16),
		rect().y() + 4, sessionOptionsWidth, headingFontHeight));
	addChild(sessionOptions);

	int solveY = rect().y() + headingFontHeight + 16;

	for (int row = 0; row < m_rows; row++)
	{
		for (int col = 0; col < m_columns; col++)
		{
			int solveIndex = (col * m_rows) + row;
			if (solveIndex >= (int)m_session->solves.size())
				break;

			int removeWidth = normalMetrics.boundingRect("  × ").width();
			int optionsWidth = normalMetrics.boundingRect("  ≡ ").width();

			shared_ptr<HistorySessionSolveTimeElement> timeElement =
				make_shared<HistorySessionSolveTimeElement>(m_session, solveIndex,
				m_bestSolveTime, m_allTimeBestSolve);
			timeElement->setRect(QRect(rect().x() + 8 + col * m_columnWidth,
				solveY + row * solveHeight, m_columnWidth - (16 + removeWidth + optionsWidth), solveHeight));
			addChild(timeElement);

			shared_ptr<HistorySessionSolveOptionsElement> optionsElement =
				make_shared<HistorySessionSolveOptionsElement>(m_session, solveIndex);
			optionsElement->setRect(QRect(timeElement->rect().x() + timeElement->rect().width(),
				timeElement->rect().y(), optionsWidth, timeElement->rect().height()));
			addChild(optionsElement);

			shared_ptr<HistorySessionSolveRemoveElement> removeElement =
				make_shared<HistorySessionSolveRemoveElement>(m_session, solveIndex);
			removeElement->setRect(QRect(optionsElement->rect().x() + optionsElement->rect().width(),
				optionsElement->rect().y(), removeWidth, optionsElement->rect().height()));
			addChild(removeElement);
		}
	}

	int bestTimeY = solveY + m_rows * solveHeight + 16;
	int x = rect().x() + 8;
	if (m_sessionAvg != -1)
	{
		shared_ptr<HistorySessionAverageElement> avg =
			make_shared<HistorySessionAverageElement>("Session avg: ", m_sessionAvg);
		QSize size = avg->sizeHint();
		avg->setRect(QRect(x, bestTimeY, size.width(), normalMetrics.height()));
		x += size.width() + 32;
		addChild(avg);
	}

	if (m_bestSolveTime != -1)
	{
		shared_ptr<HistorySessionBestSolveElement> best =
			make_shared<HistorySessionBestSolveElement>("Best solve: ", m_bestSolveTime,
			m_bestSolve);
		QSize size = best->sizeHint();
		best->setRect(QRect(x, bestTimeY, size.width(), normalMetrics.height()));
		x += size.width() + 32;
		addChild(best);
	}

	if (m_bestAvgOf5 != -1)
	{
		shared_ptr<HistorySessionBestAverageElement> avg =
			make_shared<HistorySessionBestAverageElement>("Best avg of 5: ", m_bestAvgOf5,
			m_session, m_bestAvgOf5Start, 5);
		QSize size = avg->sizeHint();
		avg->setRect(QRect(x, bestTimeY, size.width(), normalMetrics.height()));
		x += size.width() + 32;
		addChild(avg);
	}

	if (m_bestAvgOf12 != -1)
	{
		shared_ptr<HistorySessionBestAverageElement> avg =
			make_shared<HistorySessionBestAverageElement>("Best avg of 12: ", m_bestAvgOf12,
			m_session, m_bestAvgOf12Start, 12);
		QSize size = avg->sizeHint();
		avg->setRect(QRect(x, bestTimeY, size.width(), normalMetrics.height()));
		x += size.width() + 32;
		addChild(avg);
	}

	return HistoryElement::children();
}


void HistorySessionElement::paint(QPainter& p, bool)
{
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();
	int solveHeight = normalMetrics.height();

	p.fillRect(rect(), Theme::background);

	p.setFont(headingFont);
	p.setPen(Theme::blue);
	QString dateText;
	if (m_session->solves.size() != 0)
		dateText = HistoryMode::stringForDate(m_session->solves[m_session->solves.size() - 1].created);
	else
		dateText = HistoryMode::stringForDate(m_session->update.date);
	QString sessionText;
	if (m_session->name.size() == 0)
		sessionText = "Session - " + dateText;
	else
		sessionText = QString::fromStdString(m_session->name) + " - " + dateText;
	p.drawText(QRect(rect().x() + 8, rect().y() + 4, rect().width() - 16, headingFontHeight), sessionText);

	// Draw line to separate heading from solve list
	p.fillRect(rect().x() + 8, rect().y() + headingFontHeight + 8, rect().width() - 16, 1, Theme::blue.darker());

	int solveY = rect().y() + headingFontHeight + 16;

	// Draw lines to separate columns in the solve list
	for (int col = 1; col < m_columns; col++)
		p.fillRect(rect().x() + col * m_columnWidth, solveY, 1, solveHeight * m_rows, Theme::selection);

	// Draw line to separate solve list from best times
	int bestTimeY = solveY + m_rows * solveHeight + 16;
	p.fillRect(rect().x() + 8, bestTimeY - 8, rect().width() - 16, 1, Theme::selection);
}


HistorySessionOptionsElement::HistorySessionOptionsElement(const std::shared_ptr<Session>& session):
	m_session(session)
{
}


void HistorySessionOptionsElement::paint(QPainter& p, bool hovering)
{
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFontMetrics normalMetrics(normalFont);
	p.setFont(normalFont);
	if (hovering)
		p.setPen(Theme::blue);
	else
		p.setPen(Theme::selection);
	QTextOption option;
	option.setAlignment(Qt::AlignVCenter | Qt::AlignCenter);
	p.drawText(rect(), "  ≡ ");
}


bool HistorySessionOptionsElement::click(HistoryMode* parent, QMouseEvent*)
{
	shared_ptr<Session> aboveSession, belowSession;
	for (size_t i = 0; i < parent->sessions().size(); i++)
	{
		if (parent->sessions()[i] == m_session)
		{
			if (i > 0)
				aboveSession = parent->sessions()[i - 1];
			if (i < (parent->sessions().size() - 1))
				belowSession = parent->sessions()[i + 1];
			break;
		}
	}

	QMenu menu;
	QAction* rename = new QAction("Rename Session...");
	QAction* remove = new QAction("Delete Session");
	QAction* mergeAbove = nullptr;
	if (aboveSession)
		mergeAbove = new QAction("Merge with Above Session");
	QAction* mergeBelow = nullptr;
	if (belowSession)
		mergeBelow = new QAction("Merge with Below Session");
	menu.addAction(rename);
	menu.addAction(remove);
	if (mergeAbove || mergeBelow)
		menu.addSeparator();
	if (mergeAbove)
		menu.addAction(mergeAbove);
	if (mergeBelow)
		menu.addAction(mergeBelow);

	QAction* clicked = menu.exec(QCursor::pos() + QPoint(2, 2));
	if (clicked == rename)
	{
		QString name = QString::fromStdString(m_session->name);
		name = QInputDialog::getText(parent, "Rename Session", "Session name:", QLineEdit::Normal, name);
		if (name.isNull())
			return false;

		m_session->name = name.toStdString();
		m_session->dirty = true;
		History::instance.UpdateDatabaseForSession(m_session);
		return true;
	}
	else if (clicked == remove)
	{
		if (QMessageBox::critical(parent, "Delete Session", "Are you sure you want to delete this session?",
			QMessageBox::Yes, QMessageBox::No) == QMessageBox::Yes)
		{
			History::instance.DeleteSession(m_session);
			return true;
		}
	}
	else if (mergeAbove && (clicked == mergeAbove))
	{
		History::instance.MergeSessions(m_session, aboveSession, m_session->name);
		return true;
	}
	else if (mergeBelow && (clicked == mergeBelow))
	{
		History::instance.MergeSessions(belowSession, m_session, m_session->name);
		return true;
	}

	return false;
}


HistorySessionSolveTimeElement::HistorySessionSolveTimeElement(const shared_ptr<Session>& session, int idx,
	int bestSolveTime, int* allTimeBestSolve): m_session(session), m_index(idx),
	m_bestSolveTime(bestSolveTime), m_allTimeBestSolve(allTimeBestSolve)
{
}


void HistorySessionSolveTimeElement::paint(QPainter& p, bool hovering)
{
	if (m_index >= (int)m_session->solves.size())
		return;

	const Solve& solve = m_session->solves[m_index];

	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(smallFont);

	// Draw solve number to the left
	p.setPen(Theme::disabled);
	p.setFont(lightFont);
	p.drawText(rect().x(), rect().y() + lightMetrics.ascent(), QString::asprintf("%d.", m_index + 1));

	// Create strings for the solve time
	QString largeTimeText, smallTimeText;
	if (solve.ok)
	{
		int hs = (solve.time + 5) / 10;
		int minutes = hs / 6000;
		int seconds = (hs / 100) % 60;
		hs %= 100;
		if (minutes > 0)
			largeTimeText = QString::asprintf("%d:%02d", minutes, seconds);
		else
			largeTimeText = QString::asprintf("%d", seconds);
		smallTimeText = QString::asprintf(".%02d", hs);
	}
	else
	{
		largeTimeText = "DNF";
		smallTimeText = "";
	}

	// Draw penalty
	int x = rect().x() + rect().width();
	x -= smallMetrics.boundingRect("  (+2) ").width();
	if (solve.ok && (solve.penalty != 0))
	{
		p.setFont(smallFont);
		p.setPen(Theme::red);
		p.drawText(x, rect().y() + normalMetrics.ascent(),
			QString::asprintf("  (+%d)", (int)solve.penalty / 1000));
	}

	// Draw solve time
	x -= normalMetrics.horizontalAdvance(largeTimeText) +
		smallMetrics.horizontalAdvance(smallTimeText);
	p.setFont(normalFont);
	if (hovering)
		p.setPen(Theme::blue);
	else if (solve.ok && ((int)solve.time == *m_allTimeBestSolve))
		p.setPen(Theme::orange);
	else if (solve.ok && ((int)solve.time == m_bestSolveTime))
		p.setPen(Theme::green);
	else if (solve.ok)
		p.setPen(Theme::content);
	else
		p.setPen(Theme::red);
	p.drawText(x, rect().y() + normalMetrics.ascent(), largeTimeText);
	p.setFont(smallFont);
	p.drawText(x + normalMetrics.horizontalAdvance(largeTimeText),
		rect().y() + normalMetrics.ascent(), smallTimeText);
}


bool HistorySessionSolveTimeElement::click(HistoryMode*, QMouseEvent*)
{
	if (m_index >= (int)m_session->solves.size())
		return false;
	const Solve& solve = m_session->solves[m_index];
	SolveDialog* dlg = new SolveDialog(solve);
	dlg->show();
	return false;
}


void HistorySessionSolveTimeElement::tooltip(HistoryMode* parent) const
{
	if (m_index >= (int)m_session->solves.size())
		return;
	const Solve& solve = m_session->solves[m_index];
	SolveWidget* widget = new SolveWidget(solve);
	Tooltip* tooltip = new Tooltip(widget);
	tooltip->show(parent->viewport());
}


HistorySessionSolveOptionsElement::HistorySessionSolveOptionsElement(const shared_ptr<Session>& session, int idx):
	m_session(session), m_index(idx)
{
}


void HistorySessionSolveOptionsElement::paint(QPainter& p, bool hovering)
{
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFontMetrics normalMetrics(normalFont);
	p.setFont(normalFont);
	if (hovering)
		p.setPen(Theme::blue);
	else
		p.setPen(Theme::selection);
	p.drawText(rect().x(), rect().y() + normalMetrics.ascent(), " ≡ ");
}


bool HistorySessionSolveOptionsElement::click(HistoryMode*, QMouseEvent*)
{
	if (m_index >= (int)m_session->solves.size())
		return false;

	Solve& solve = m_session->solves[m_index];

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
	QAction* split = nullptr;
	if (m_index > 0)
		split = new QAction("Split Session Starting Here");
	menu.addAction(solveOK);
	menu.addAction(penalty);
	menu.addAction(dnf);
	if (split)
	{
		menu.addSeparator();
		menu.addAction(split);
	}

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
	else if (split && (clicked == split))
	{
		History::instance.SplitSessionAtSolve(m_session, (size_t)m_index);
		return true;
	}
	else
	{
		return false;
	}

	m_session->update.id = History::instance.idGenerator->GenerateId();
	m_session->update.date = time(NULL);
	m_session->dirty = true;
	History::instance.UpdateDatabaseForSession(m_session);
	return true;
}


HistorySessionSolveRemoveElement::HistorySessionSolveRemoveElement(const shared_ptr<Session>& session, int idx):
	m_session(session), m_index(idx)
{
}


void HistorySessionSolveRemoveElement::paint(QPainter& p, bool hovering)
{
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFontMetrics normalMetrics(normalFont);
	p.setFont(normalFont);
	if (hovering)
		p.setPen(Theme::red);
	else
		p.setPen(Theme::selection);
	p.drawText(rect().x(), rect().y() + normalMetrics.ascent(), " × ");
}


bool HistorySessionSolveRemoveElement::click(HistoryMode* parent, QMouseEvent*)
{
	if (m_index >= (int)m_session->solves.size())
		return false;

	Solve& solve = m_session->solves[m_index];

	QString msg;
	if (solve.ok)
		msg = QString("Delete solve with time of ") + SessionWidget::stringForSolveTime(solve) + QString("?");
	else
		msg = "Delete DNF solve?";
	if (QMessageBox::critical(parent, "Delete Solve", msg, QMessageBox::Yes, QMessageBox::No) != QMessageBox::Yes)
		return false;

	m_session->solves.erase(m_session->solves.begin() + m_index);
	m_session->dirty = true;
	History::instance.UpdateDatabaseForSession(m_session);

	if (m_session->solves.size() == 0)
		History::instance.DeleteSession(m_session);

	return true;
}


HistorySessionBestElement::HistorySessionBestElement(const QString& title, int best):
	m_title(title), m_best(best)
{
}


QSize HistorySessionBestElement::sizeHint() const
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(normalFont);

	int width = lightMetrics.horizontalAdvance(m_title);

	int hs = (m_best + 5) / 10;
	int minutes = hs / 6000;
	int seconds = (hs / 100) % 60;
	hs %= 100;

	QString largeText, smallText;
	if (minutes > 0)
		largeText = QString::asprintf("%d:%02d", minutes, seconds);
	else
		largeText = QString::asprintf("%d", seconds);
	smallText = QString::asprintf(".%02d", hs);

	width += normalMetrics.horizontalAdvance(largeText);
	width += smallMetrics.horizontalAdvance(smallText);
	return QSize(width, normalMetrics.height());
}


void HistorySessionBestElement::paint(QPainter& p, bool hovering)
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(normalFont);

	int x = rect().x();
	p.setFont(lightFont);
	p.setPen(Theme::disabled);
	p.drawText(x, rect().y() + normalMetrics.ascent(), m_title);
	x += lightMetrics.horizontalAdvance(m_title);

	int hs = (m_best + 5) / 10;
	int minutes = hs / 6000;
	int seconds = (hs / 100) % 60;
	hs %= 100;

	QString largeText, smallText;
	if (minutes > 0)
		largeText = QString::asprintf("%d:%02d", minutes, seconds);
	else
		largeText = QString::asprintf("%d", seconds);
	smallText = QString::asprintf(".%02d", hs);

	p.setFont(normalFont);
	if (hovering)
		p.setPen(Theme::blue);
	else
		p.setPen(Theme::content);
	p.drawText(x, rect().y() + normalMetrics.ascent(), largeText);
	x += normalMetrics.horizontalAdvance(largeText);
	p.setFont(smallFont);
	p.drawText(x, rect().y() + normalMetrics.ascent(), smallText);
	x += smallMetrics.horizontalAdvance(smallText);
}


HistorySessionBestSolveElement::HistorySessionBestSolveElement(const QString& title, int best, const Solve& solve):
	HistorySessionBestElement(title, best), m_solve(solve)
{
}


bool HistorySessionBestSolveElement::click(HistoryMode*, QMouseEvent*)
{
	SolveDialog* dlg = new SolveDialog(m_solve);
	dlg->show();
	return false;
}


void HistorySessionBestSolveElement::tooltip(HistoryMode* parent) const
{
	SolveWidget* widget = new SolveWidget(m_solve);
	Tooltip* tooltip = new Tooltip(widget);
	tooltip->show(parent->viewport());
}


HistorySessionAverageElement::HistorySessionAverageElement(const QString& title, int best):
	HistorySessionBestElement(title, best)
{
}


HistorySessionBestAverageElement::HistorySessionBestAverageElement(const QString& title, int best,
	const shared_ptr<Session>& session, int start, int size):
	HistorySessionBestElement(title, best), m_session(session), m_start(start), m_size(size)
{
}


bool HistorySessionBestAverageElement::click(HistoryMode*, QMouseEvent*)
{
	vector<Solve> solves;
	if ((!m_session) || ((m_start + m_size) > (int)m_session->solves.size()))
		return false;
	for (int i = 0; i < m_size; i++)
		solves.push_back(m_session->solves[m_start + i]);
	AverageDialog* dlg = new AverageDialog(solves);
	dlg->show();
	dlg->setFixedSize(dlg->size());
	return false;
}


void HistorySessionBestAverageElement::tooltip(HistoryMode* parent) const
{
	vector<Solve> solves;
	if ((!m_session) || ((m_start + m_size) > (int)m_session->solves.size()))
		return;
	for (int i = 0; i < m_size; i++)
		solves.push_back(m_session->solves[m_start + i]);
	AverageWidget* widget = new AverageWidget(solves);
	Tooltip* tooltip = new Tooltip(widget);
	tooltip->show(parent->viewport());
}


HistoryMode::HistoryMode(QWidget* parent): QAbstractScrollArea(parent)
{
	setFrameStyle(QFrame::NoFrame);
	setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
	setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOn);
	setMouseTracking(true);

	m_hoverTimer = new QTimer(this);
	m_hoverTimer->setSingleShot(true);
	m_hoverTimer->setInterval(500);
	connect(m_hoverTimer, &QTimer::timeout, this, &HistoryMode::hoverTooltip);
}


void HistoryMode::paintElement(QPainter& p, QPaintEvent* event, const shared_ptr<HistoryElement>& element)
{
	int yofs = verticalScrollBar()->value();
	if (!QRect(event->rect().x(), event->rect().y() + yofs,
		event->rect().width(), event->rect().height()).intersects(element->rect()))
		return;

	element->paint(p, element == m_hoverElement);

	for (auto& i : element->children())
		paintElement(p, event, i);
}


void HistoryMode::paintEvent(QPaintEvent* event)
{
	QPainter p(viewport());
	int yofs = verticalScrollBar()->value();
	p.translate(0, -yofs);

	if (m_elements.size() == 0)
	{
		// No sessions, don't leave it blank
		QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light, true);
		QFontMetrics lightMetrics(lightFont);
		p.setFont(lightFont);
		p.setPen(Theme::disabled);
		p.drawText(16, 16 - yofs + lightMetrics.ascent(), "No sessions have been completed.");
		return;
	}

	for (auto& i : m_elements)
		paintElement(p, event, i);
}


void HistoryMode::resizeEvent(QResizeEvent*)
{
	updateHistory();
}


shared_ptr<HistoryElement> HistoryMode::getInteractableElement(int x, int y, const shared_ptr<HistoryElement>& element)
{
	if (!element->rect().contains(x, y))
		return shared_ptr<HistoryElement>();
	for (auto& i : element->children())
	{
		shared_ptr<HistoryElement> result = getInteractableElement(x, y, i);
		if (result)
			return result;
	}
	if (!element->interactable())
		return shared_ptr<HistoryElement>();
	return element;
}


void HistoryMode::mouseMoveEvent(QMouseEvent* event)
{
	int yofs = verticalScrollBar()->value();
	int x = event->x();
	int y = event->y() + yofs;

	shared_ptr<HistoryElement> result;
	for (auto& i : m_elements)
	{
		result = getInteractableElement(x, y, i);
		if (result)
			break;
	}

	if (result != m_hoverElement)
	{
		m_hoverElement = result;
		if (result && result->hasHandCursor())
			setCursor(Qt::PointingHandCursor);
		else
			setCursor(Qt::ArrowCursor);
		viewport()->update();
	}

	m_hoverTimer->stop();
	if (m_hoverElement)
		m_hoverTimer->start();
}


void HistoryMode::mousePressEvent(QMouseEvent* event)
{
	setCursor(Qt::ArrowCursor);
	if (m_hoverElement)
	{
		if (m_hoverElement->click(this, event))
		{
			m_hoverElement.reset();
			updateHistory();
		}
	}
}


void HistoryMode::leaveEvent(QEvent*)
{
	m_hoverElement.reset();
	viewport()->update();
	setCursor(Qt::ArrowCursor);
	m_hoverTimer->stop();
}


void HistoryMode::scrollContentsBy(int dx, int dy)
{
	if (m_hoverElement)
	{
		m_hoverElement.reset();
		viewport()->update();
	}
	QAbstractScrollArea::scrollContentsBy(dx, dy);
}


void HistoryMode::hoverTooltip()
{
	if (m_hoverElement)
		m_hoverElement->tooltip(this);
}


void HistoryMode::updateHistory()
{
	m_sessions.clear();
	m_elements.clear();
	m_hoverElement.reset();

	// Sort sessions so that newest sessions are on top
	vector<shared_ptr<Session>> sortedSessions = History::instance.sessions;
	sort(sortedSessions.begin(), sortedSessions.end(),
		[](const shared_ptr<Session>& a, const shared_ptr<Session>& b) {
			time_t aTime = a->update.date;
			time_t bTime = b->update.date;
			if (a->solves.size() > 0)
				aTime = a->solves[a->solves.size() - 1].created;
			if (b->solves.size() > 0)
				bTime = b->solves[b->solves.size() - 1].created;
			return aTime > bTime;
		});

	// Create fonts and measurements for them
	int y = 16;

	m_bestSolveTime = -1;
	Solve bestSolve;
	int bestAvgOf5 = -1;
	shared_ptr<Session> bestAvgOf5Session;
	int bestAvgOf5Start = -1;
	int bestAvgOf12 = -1;
	shared_ptr<Session> bestAvgOf12Session;
	int bestAvgOf12Start = -1;

	for (auto& i : sortedSessions)
	{
		if (i->type != m_type)
			continue;

		shared_ptr<HistorySessionElement> element = make_shared<HistorySessionElement>(i, 16,
			y, viewport()->width() - 32, &m_bestSolveTime);
		m_elements.push_back(element);
		m_sessions.push_back(i);
		y += element->rect().height() + 16;

		// Compute best times for the session
		Solve bestSessionSolve;
		int bestSessionSolveTime = i->bestSolve(&bestSessionSolve);
		int bestSessionAvgOf5Start, bestSessionAvgOf12Start;
		int bestSessionAvgOf5 = i->bestAvgOf(5, &bestSessionAvgOf5Start);
		int bestSessionAvgOf12 = i->bestAvgOf(12, &bestSessionAvgOf12Start);

		if ((bestSessionSolveTime != -1) && ((m_bestSolveTime == -1) || (bestSessionSolveTime < m_bestSolveTime)))
		{
			m_bestSolveTime = bestSessionSolveTime;
			bestSolve = bestSessionSolve;
		}

		if ((bestSessionAvgOf5 != -1) && ((bestAvgOf5 == -1) || (bestSessionAvgOf5 < bestAvgOf5)))
		{
			bestAvgOf5 = bestSessionAvgOf5;
			bestAvgOf5Session = i;
			bestAvgOf5Start = bestSessionAvgOf5Start;
		}
		if ((bestSessionAvgOf12 != -1) && ((bestAvgOf12 == -1) || (bestSessionAvgOf12 < bestAvgOf12)))
		{
			bestAvgOf12 = bestSessionAvgOf12;
			bestAvgOf12Session = i;
			bestAvgOf12Start = bestSessionAvgOf12Start;
		}
	}

	y += 16; // End padding

	if ((m_bestSolveTime != -1) || (bestAvgOf5 != -1) || (bestAvgOf12 != -1))
	{
		// Show a personal best area at the top
		shared_ptr<HistoryAllTimeBestSolveElement> solveElement;
		shared_ptr<HistoryAllTimeBestAverageElement> avgOf5Element, avgOf12Element;
		int width = 0;
		int height = 0;

		if (m_bestSolveTime != -1)
		{
			solveElement = make_shared<HistoryAllTimeBestSolveElement>(
				"Best solve", m_bestSolveTime, bestSolve);
			width += solveElement->sizeHint().width();
			height = solveElement->sizeHint().height();
		}

		if (bestAvgOf5 != -1)
		{
			avgOf5Element = make_shared<HistoryAllTimeBestAverageElement>(
				"Best avg of 5", bestAvgOf5, bestAvgOf5Session, bestAvgOf5Start, 5);
			if (width > 0)
				width += 32;
			width += avgOf5Element->sizeHint().width();
			height = avgOf5Element->sizeHint().height();
		}

		if (bestAvgOf12 != -1)
		{
			avgOf12Element = make_shared<HistoryAllTimeBestAverageElement>(
				"Best avg of 12", bestAvgOf12, bestAvgOf12Session, bestAvgOf12Start, 12);
			if (width > 0)
				width += 32;
			width += avgOf12Element->sizeHint().width();
			height = avgOf12Element->sizeHint().height();
		}

		int x = (viewport()->width() / 2) - (width / 2);
		if (solveElement)
		{
			solveElement->setRect(QRect(x, 16, solveElement->sizeHint().width(), height));
			x += solveElement->rect().width() + 32;
		}
		if (avgOf5Element)
		{
			avgOf5Element->setRect(QRect(x, 16, avgOf5Element->sizeHint().width(), height));
			x += avgOf5Element->rect().width() + 32;
		}
		if (avgOf12Element)
		{
			avgOf12Element->setRect(QRect(x, 16, avgOf12Element->sizeHint().width(), height));
			x += avgOf12Element->rect().width() + 32;
		}

		for (auto& i : m_elements)
			i->move(0, height + 16);
		y += height + 16;

		if (solveElement)
			m_elements.push_back(solveElement);
		if (avgOf5Element)
			m_elements.push_back(avgOf5Element);
		if (avgOf12Element)
			m_elements.push_back(avgOf12Element);
	}

	verticalScrollBar()->setRange(0, y - viewport()->height());
	verticalScrollBar()->setSingleStep(8);
	verticalScrollBar()->setPageStep(viewport()->height());

	viewport()->update();
}


QString HistoryMode::stringForDate(time_t date)
{
	QDateTime dt = QDateTime::fromTime_t(date);
	QDateTime now = QDateTime::currentDateTime();
	QString t = dt.toString("h:mm ap");
	if (dt.daysTo(now) == 0)
		return QString("Today at %1").arg(t);
	else if (dt.daysTo(now) == 1)
		return QString("Yesterday at %1").arg(t);
	else if (dt.daysTo(now) < 7)
		return QString("%1 at %2").arg(dt.toString("dddd")).arg(t);
	else if (dt.daysTo(now) < 365)
		return QString("%1 at %2").arg(dt.toString("MMMM d")).arg(t);
	return QString("%1 at %2").arg(dt.toString("MMMM d, yyyy")).arg(t);
}


QString HistoryMode::shortStringForDate(time_t date)
{
	QDateTime dt = QDateTime::fromTime_t(date);
	QDateTime now = QDateTime::currentDateTime();
	QString t = dt.toString("h:mm ap");
	if (dt.daysTo(now) == 0)
		return QString("Today at %1").arg(t);
	else if (dt.daysTo(now) < 7)
		return QString("%1 at %2").arg(dt.toString("ddd")).arg(t);
	else if (dt.daysTo(now) < 365)
		return QString("%1 at %2").arg(dt.toString("MMM d")).arg(t);
	return QString("%1 at %2").arg(dt.toString("MMM d, yyyy")).arg(t);
}
