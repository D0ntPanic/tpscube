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

using namespace std;


HistoryMode::HistoryMode(QWidget* parent): QAbstractScrollArea(parent)
{
	setFrameStyle(QFrame::NoFrame);
	setHorizontalScrollBarPolicy(Qt::ScrollBarAlwaysOff);
	setVerticalScrollBarPolicy(Qt::ScrollBarAlwaysOn);
	setMouseTracking(true);
}


void HistoryMode::paintAllTimeBest(QPainter& p, int x, const QString& title, int best)
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont largestFont = fontOfRelativeSize(2.5f, QFont::Light);
	QFont largeFont = fontOfRelativeSize(2.0f, QFont::Light);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics largestMetrics(largestFont);
	QFontMetrics largeMetrics(largeFont);

	int yofs = verticalScrollBar()->value();

	int headingWidth = lightMetrics.boundingRect(title).width();
	p.setFont(lightFont);
	p.setPen(Theme::content);
	p.drawText(x - (headingWidth / 2), 16 + lightMetrics.ascent() - yofs, title);
	int y = 16 + lightMetrics.height() - yofs;

	int hs = (best + 5) / 10;
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
	int timeX = x - (timeWidth / 2);
	p.setFont(largestFont);
	p.setPen(Theme::orange);
	p.drawText(timeX, y + largestMetrics.ascent(), largeText);
	p.setFont(largeFont);
	p.drawText(timeX + largestWidth, y + largestMetrics.ascent(), smallText);
}


void HistoryMode::paintSessionBest(QPainter& p, int& x, int y, const QString& title, int best)
{
	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(normalFont);

	p.setFont(lightFont);
	p.setPen(Theme::disabled);
	p.drawText(x, y + normalMetrics.ascent(), title);
	x += lightMetrics.horizontalAdvance(title);

	int hs = (best + 5) / 10;
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
	p.setPen(Theme::content);
	p.drawText(x, y + normalMetrics.ascent(), largeText);
	x += normalMetrics.horizontalAdvance(largeText);
	p.setFont(smallFont);
	p.drawText(x, y + normalMetrics.ascent(), smallText);
	x += smallMetrics.horizontalAdvance(smallText);
}


void HistoryMode::paintEvent(QPaintEvent* event)
{
	QPainter p(viewport());
	int width = viewport()->width();

	// Create fonts used during rendering
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();

	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(normalFont);
	int solveHeight = normalMetrics.height();

	int yofs = verticalScrollBar()->value();

	if (m_sessions.size() == 0)
	{
		// No sessions, don't leave it blank
		p.setFont(fontOfRelativeSize(1.0f, QFont::Light, true));
		p.setPen(Theme::disabled);
		p.drawText(16, 16 - yofs + lightMetrics.ascent(), "No sessions have been completed.");
		return;
	}

	// Render personal bests at top
	int bestCount = 0;
	if (m_bestSolve != -1)
		bestCount++;
	if (m_bestAvgOf5 != -1)
		bestCount++;
	if (m_bestAvgOf12 != -1)
		bestCount++;

	if (bestCount > 0)
	{
		QFont largestFont = fontOfRelativeSize(2.5f, QFont::Light);
		QFontMetrics largestMetrics(largestFont);
		int bestWidth = largestMetrics.boundingRect("00:00.00").width() + 32;

		int x = (viewport()->width() / 2) + (bestWidth / 2) - ((bestWidth * bestCount) / 2);

		if (m_bestSolve != -1)
		{
			paintAllTimeBest(p, x, "Best solve", m_bestSolve);
			x += bestWidth;
		}

		if (m_bestAvgOf5 != -1)
		{
			paintAllTimeBest(p, x, "Best avg of 5", m_bestAvgOf5);
			x += bestWidth;
		}

		if (m_bestAvgOf12 != -1)
		{
			paintAllTimeBest(p, x, "Best avg of 12", m_bestAvgOf12);
			x += bestWidth;
		}
	}

	// Render sessions
	for (auto& i : m_sessions)
	{
		if ((i.y - yofs) > event->rect().bottom())
			break;
		if ((i.y + i.height - yofs) < event->rect().top())
			continue;

		p.fillRect(16, i.y - yofs, width - 32, i.height, Theme::background);

		// Draw session heading
		p.setFont(headingFont);
		p.setPen(Theme::blue);
		QString dateText;
		if (i.session->solves.size() != 0)
			dateText = stringForDate(i.session->solves[i.session->solves.size() - 1].created);
		else
			dateText = stringForDate(i.session->update.date);
		QString sessionText;
		if (i.session->name.size() == 0)
			sessionText = "Session - " + dateText;
		else
			sessionText = QString::fromStdString(i.session->name) + " - " + dateText;
		p.drawText(QRect(24, i.y + 4 - yofs, width - 48, headingFontHeight), sessionText);

		p.setFont(normalFont);
		if ((m_hoverSession == i.session) && (m_hoverSolve == -1) && (m_hoverIcon == 0))
			p.setPen(Theme::blue);
		else
			p.setPen(Theme::selection);
		QTextOption textOption;
		textOption.setAlignment(Qt::AlignVCenter | Qt::AlignRight);
		p.drawText(QRect(24, i.y + 4 - yofs, width - 48, headingFontHeight), "  ≡ ", textOption);

		// Draw line to separate heading from solve list
		p.fillRect(24, i.y + headingFontHeight + 8 - yofs, width - 48, 1, Theme::blue.darker());

		int solveY = i.y + headingFontHeight + 16 - yofs;

		// Draw lines to separate columns in the solve list
		for (int col = 1; col < i.columns; col++)
			p.fillRect(16 + col * i.columnWidth, solveY, 1, solveHeight * i.rows, Theme::selection);

		// Draw solves
		for (int row = 0; row < i.rows; row++)
		{
			if (solveY > event->rect().bottom())
				break;
			if ((solveY + (row + 1) * solveHeight) < event->rect().top())
				continue;

			for (int col = 0; col < i.columns; col++)
			{
				int solveIndex = (col * i.rows) + row;
				if (solveIndex >= (int)i.session->solves.size())
					break;

				const Solve& solve = i.session->solves[solveIndex];

				// Draw solve number to the left
				p.setPen(Theme::disabled);
				p.setFont(lightFont);
				p.drawText(24 + col * i.columnWidth, solveY + row * solveHeight + lightMetrics.ascent(),
					QString::asprintf("%d.", solveIndex + 1));

				// Draw options and remove buttons
				int x = 8 + (col + 1) * i.columnWidth;
				x -= normalMetrics.boundingRect("  × ").width();
				p.setFont(normalFont);
				if ((m_hoverSession == i.session) && (m_hoverSolve == solveIndex) && (m_hoverIcon == 1))
					p.setPen(Theme::red);
				else
					p.setPen(Theme::selection);
				p.drawText(x, solveY + row * solveHeight + normalMetrics.ascent(), "  × ");

				x -= normalMetrics.boundingRect("  ≡ ").width();
				p.setFont(normalFont);
				if ((m_hoverSession == i.session) && (m_hoverSolve == solveIndex) && (m_hoverIcon == 0))
					p.setPen(Theme::blue);
				else
					p.setPen(Theme::selection);
				p.drawText(x, solveY + row * solveHeight + normalMetrics.ascent(), "  ≡ ");

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
					smallTimeText = " ";
				}

				// Draw penalty
				x -= smallMetrics.boundingRect("  (+2) ").width();
				if (solve.ok && (solve.penalty != 0))
				{
					p.setFont(smallFont);
					p.setPen(Theme::red);
					p.drawText(x, solveY + row * solveHeight + normalMetrics.ascent(),
						QString::asprintf("  (+%d)", (int)solve.penalty / 1000));
				}

				// Draw solve time
				x -= normalMetrics.horizontalAdvance(largeTimeText) +
					smallMetrics.horizontalAdvance(smallTimeText);
				p.setFont(normalFont);
				if (solve.ok && ((int)solve.time == m_bestSolve))
					p.setPen(Theme::orange);
				else if (solve.ok && ((int)solve.time == i.bestSolve))
					p.setPen(Theme::green);
				else if (solve.ok)
					p.setPen(Theme::content);
				else
					p.setPen(Theme::red);
				p.drawText(x, solveY + row * solveHeight + normalMetrics.ascent(), largeTimeText);
				p.setFont(smallFont);
				p.drawText(x + normalMetrics.horizontalAdvance(largeTimeText),
					solveY + row * solveHeight + normalMetrics.ascent(), smallTimeText);
			}
		}

		// Draw line to separate solve list from best times
		int bestTimeY = solveY + i.rows * solveHeight + 16;
		p.fillRect(24, bestTimeY - 8, width - 48, 1, Theme::selection);

		int x = 24;
		if (i.sessionAvg != -1)
		{
			paintSessionBest(p, x, bestTimeY, "Session avg: ", i.sessionAvg);
			x += 32;
		}

		if (i.bestSolve != -1)
		{
			paintSessionBest(p, x, bestTimeY, "Best solve: ", i.bestSolve);
			x += 32;
		}

		if (i.bestAvgOf5 != -1)
		{
			paintSessionBest(p, x, bestTimeY, "Best avg of 5: ", i.bestAvgOf5);
			x += 32;
		}

		if (i.bestAvgOf12 != -1)
		{
			paintSessionBest(p, x, bestTimeY, "Best avg of 12: ", i.bestAvgOf12);
			x += 32;
		}
	}
}


void HistoryMode::resizeEvent(QResizeEvent*)
{
	updateHistory();
}


void HistoryMode::mouseMoveEvent(QMouseEvent* event)
{
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFontMetrics normalMetrics(normalFont);
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();
	int solveHeight = normalMetrics.height();

	int yofs = verticalScrollBar()->value();
	int x = event->x();
	int y = event->y() + yofs;

	for (auto& i : m_sessions)
	{
		if (y > (i.y + i.height))
			continue;
		if (y < i.y)
			break;

		if (y < (i.y + headingFontHeight + 16))
		{
			int rightX = viewport()->width() - 24;
			int leftX = rightX - normalMetrics.boundingRect("  ≡ ").width();
			if ((x >= leftX) && (x <= rightX))
			{
				if ((m_hoverSession == i.session) && (m_hoverSolve == -1) && (m_hoverIcon == 0))
					return;
				m_hoverSession = i.session;
				m_hoverSolve = -1;
				m_hoverIcon = 0;
				viewport()->update();
				return;
			}
			break;
		}

		int solveY = i.y + headingFontHeight + 16;
		if (y < solveY)
			break;
		int row = (y - solveY) / solveHeight;
		int col = (x - 24) / i.columnWidth;

		if ((row < 0) || (row >= i.rows) || (col < 0) || (col >= i.columns))
			break;

		int solveIdx = (col * i.rows) + row;
		if ((solveIdx < 0) || (solveIdx >= (int)i.session->solves.size()))
			break;

		int rightX = 8 + (col + 1) * i.columnWidth;
		int leftX = rightX - normalMetrics.boundingRect("  × ").width();
		if ((x >= leftX) && (x <= rightX))
		{
			if ((m_hoverSession == i.session) && (m_hoverSolve == solveIdx) && (m_hoverIcon == 1))
				return;
			m_hoverSession = i.session;
			m_hoverSolve = solveIdx;
			m_hoverIcon = 1;
			viewport()->update();
			return;
		}

		rightX = leftX;
		leftX = rightX - normalMetrics.boundingRect("  ≡ ").width();
		if ((x >= leftX) && (x <= rightX))
		{
			if ((m_hoverSession == i.session) && (m_hoverSolve == solveIdx) && (m_hoverIcon == 0))
				return;
			m_hoverSession = i.session;
			m_hoverSolve = solveIdx;
			m_hoverIcon = 0;
			viewport()->update();
			return;
		}
	}

	if (m_hoverSession)
	{
		m_hoverSession.reset();
		m_hoverSolve = -1;
		m_hoverIcon = -1;
		viewport()->update();
	}
}


void HistoryMode::mousePressEvent(QMouseEvent*)
{
	if (m_hoverSession && (m_hoverSolve != -1) && (m_hoverSolve < (int)m_hoverSession->solves.size()))
	{
		shared_ptr<Session> session = m_hoverSession;
		int solveIdx = m_hoverSolve;
		Solve& solve = session->solves[solveIdx];

		if (m_hoverIcon == 0)
		{
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
			if (m_hoverSolve > 0)
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
				History::instance.SplitSessionAtSolve(session, (size_t)solveIdx);
				m_hoverSession.reset();
				m_hoverSolve = -1;
				m_hoverIcon = -1;
				updateHistory();
				return;
			}
			else
			{
				return;
			}

			session->update.id = History::instance.idGenerator->GenerateId();
			session->update.date = time(NULL);
			session->dirty = true;
			History::instance.UpdateDatabaseForSession(session);

			m_hoverSession.reset();
			m_hoverSolve = -1;
			m_hoverIcon = -1;
			updateHistory();
		}
		else if (m_hoverIcon == 1)
		{
			QString msg;
			if (solve.ok)
				msg = QString("Delete solve with time of ") + SessionWidget::stringForSolveTime(solve) + QString("?");
			else
				msg = "Delete DNF solve?";
			if (QMessageBox::critical(this, "Delete Solve", msg, QMessageBox::Yes, QMessageBox::No) != QMessageBox::Yes)
				return;

			session->solves.erase(session->solves.begin() + solveIdx);
			session->dirty = true;
			History::instance.UpdateDatabaseForSession(session);

			if (session->solves.size() == 0)
				History::instance.DeleteSession(session);

			m_hoverSession.reset();
			m_hoverSolve = -1;
			m_hoverIcon = -1;
			updateHistory();
		}
	}
	else if (m_hoverSession && (m_hoverSolve == -1) && (m_hoverIcon == 0))
	{
		shared_ptr<Session> session = m_hoverSession;
		shared_ptr<Session> aboveSession, belowSession;
		for (size_t i = 0; i < m_sessions.size(); i++)
		{
			if (m_sessions[i].session == session)
			{
				if (i > 0)
					aboveSession = m_sessions[i - 1].session;
				if (i < (m_sessions.size() - 1))
					belowSession = m_sessions[i + 1].session;
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
			QString name = QString::fromStdString(session->name);
			name = QInputDialog::getText(this, "Rename Session", "Session name:", QLineEdit::Normal, name);
			if (name.isNull())
				return;

			session->name = name.toStdString();
			session->dirty = true;
			History::instance.UpdateDatabaseForSession(session);
		}
		else if (clicked == remove)
		{
			if (QMessageBox::critical(this, "Delete Session", "Are you sure you want to delete this session?",
				QMessageBox::Yes, QMessageBox::No) == QMessageBox::Yes)
				History::instance.DeleteSession(session);
		}
		else if (mergeAbove && (clicked == mergeAbove))
		{
			History::instance.MergeSessions(session, aboveSession, session->name);
		}
		else if (mergeBelow && (clicked == mergeBelow))
		{
			History::instance.MergeSessions(belowSession, session, session->name);
		}
		else
		{
			return;
		}

		m_hoverSession.reset();
		m_hoverSolve = -1;
		m_hoverIcon = -1;
		updateHistory();
	}
}


void HistoryMode::leaveEvent(QEvent*)
{
	m_hoverSession.reset();
	m_hoverSolve = -1;
	m_hoverIcon = -1;
	viewport()->update();
}


void HistoryMode::updateHistory()
{
	m_sessions.clear();

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
	QFont headingFont = fontOfRelativeSize(1.1f, QFont::Light);
	QFontMetrics headingMetrics(headingFont);
	int headingFontHeight = headingMetrics.height();

	QFont lightFont = fontOfRelativeSize(1.0f, QFont::Light);
	QFont normalFont = fontOfRelativeSize(1.0f, QFont::Normal);
	QFont smallFont = fontOfRelativeSize(0.75f, QFont::Normal);
	QFont largeFont = fontOfRelativeSize(2.5f, QFont::Light);
	QFontMetrics lightMetrics(lightFont);
	QFontMetrics normalMetrics(normalFont);
	QFontMetrics smallMetrics(normalFont);
	QFontMetrics largeMetrics(largeFont);

	m_bestSolve = -1;
	m_bestAvgOf5 = -1;
	m_bestAvgOf12 = -1;

	for (auto& i : sortedSessions)
	{
		if (i->type != m_type)
			continue;

		SessionHistoryInfo info;
		info.session = i;
		info.y = y;
		info.height = headingFontHeight + 24;

		// Compute size of individual components of each solve display
		QString maxSolveNum = QString::asprintf("%d.    ", (int)i->solves.size());
		int solveNumWidth = lightMetrics.boundingRect(maxSolveNum).width();

		int maxSolveTime = 60000;
		for (auto& j : i->solves)
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
		info.timeXOffset = solveNumWidth + timeWidth;

		// Compute the number of columns that can be displayed for this session
		info.columnWidth = solveNumWidth + timeWidth + penaltyWidth +
			optionWidth + removeWidth + 16;
		info.columns = (viewport()->width() - 40) / info.columnWidth;
		if (info.columns < 1)
			info.columns = 1;

		// Compute the number of rows that are required and update size of session
		info.rows = ((int)i->solves.size() + info.columns - 1) / info.columns;
		info.height += info.rows * normalMetrics.height();

		// Compute best times for the session
		info.bestSolve = i->bestSolve();
		info.bestAvgOf5 = i->bestAvgOf(5);
		info.bestAvgOf12 = i->bestAvgOf(12);
		info.sessionAvg = i->sessionAvg();

		if ((info.bestSolve != -1) && ((m_bestSolve == -1) || (info.bestSolve < m_bestSolve)))
			m_bestSolve = info.bestSolve;
		if ((info.bestAvgOf5 != -1) && ((m_bestAvgOf5 == -1) || (info.bestAvgOf5 < m_bestAvgOf5)))
			m_bestAvgOf5 = info.bestAvgOf5;
		if ((info.bestAvgOf12 != -1) && ((m_bestAvgOf12 == -1) || (info.bestAvgOf12 < m_bestAvgOf12)))
			m_bestAvgOf12 = info.bestAvgOf12;

		info.height += normalMetrics.height() + 16;

		m_sessions.push_back(info);

		y += info.height + 16;
	}

	y += 16; // End padding

	if ((m_bestSolve != -1) || (m_bestAvgOf5 != -1) || (m_bestAvgOf12 != -1))
	{
		// Show a personal best area at the top
		int bestHeight = lightMetrics.height() + largeMetrics.height() + 16;
		for (auto& i : m_sessions)
			i.y += bestHeight;
		y += bestHeight;
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
