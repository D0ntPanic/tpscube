#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QGridLayout>
#include <QtGui/QKeyEvent>
#include <QtGui/QPainter>
#include <QtCore/QDateTime>
#include "solvewidget.h"
#include "historymode.h"
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

	bool showSolveBarAsScrubBar = false;
	if (fullDetails)
	{
		layout->addSpacing(8);

		m_cube = new Cube3x3Widget();
		m_cube->applyImmediate(m_solve.scramble);
		layout->addWidget(m_cube, 1);

		if (solve.ok && solve.crossTime && solve.f2lPairTimes[3] && solve.ollFinishTime &&
			(solve.solveMoves.moves.size() != 0))
		{
			showSolveBarAsScrubBar = true;

			QGridLayout* playbackLayout = new QGridLayout();
			playbackLayout->setSpacing(12);

			QFontMetrics metrics(font());

			QImage playImage(":/images/play.png");
			QPainter playPainter(&m_playIcon);
			playPainter.setRenderHint(QPainter::SmoothPixmapTransform);
			playPainter.drawImage(QRect(0, 0, 16, 16), playImage);

			QImage playHoverImage(":/images/play_hover.png");
			QPainter playHoverPainter(&m_playHoverIcon);
			playHoverPainter.setRenderHint(QPainter::SmoothPixmapTransform);
			playHoverPainter.drawImage(QRect(0, 0, 16, 16), playHoverImage);

			QImage pauseImage(":/images/pause.png");
			QPainter pausePainter(&m_pauseIcon);
			pausePainter.setRenderHint(QPainter::SmoothPixmapTransform);
			pausePainter.drawImage(QRect(0, 0, 16, 16), pauseImage);

			QImage pauseHoverImage(":/images/pause_hover.png");
			QPainter pauseHoverPainter(&m_pauseHoverIcon);
			pauseHoverPainter.setRenderHint(QPainter::SmoothPixmapTransform);
			pauseHoverPainter.drawImage(QRect(0, 0, 16, 16), pauseHoverImage);

			m_playButton = new ClickableLabel("",
				Theme::content, Theme::blue, [this]() {
					if (m_playbackRunning)
						pausePlayback();
					else
						startPlayback();
				});
			m_playButton->setPictures(m_playIcon, m_playHoverIcon);
			m_playButton->setCursor(Qt::PointingHandCursor);
			m_playButton->setAlignment(Qt::AlignLeft | Qt::AlignVCenter);
			playbackLayout->addWidget(m_playButton, 0, 0);

			m_playbackTimeLabel = new QLabel(SessionWidget::stringForTime(0));
			m_playbackTimeLabel->setAlignment(Qt::AlignRight | Qt::AlignVCenter);
			playbackLayout->addWidget(m_playbackTimeLabel, 0, 1);

			m_solveBar = new SolveBarWidget(solve);
			m_solveBar->setBarHeight(4);
			m_solveBar->setPadding(7, 4);
			m_solveBar->setShowCurrentPos(true);
			connect(m_solveBar, &SolveBarWidget::seek, this, &SolveWidget::seekInPlayback);
			playbackLayout->addWidget(m_solveBar, 0, 2);

			playbackLayout->setColumnStretch(0, 0);
			playbackLayout->setColumnMinimumWidth(1, metrics.boundingRect(
				solveTimeText(solve)).width());
			playbackLayout->setColumnStretch(1, 0);
			playbackLayout->setColumnStretch(2, 1);
			layout->addLayout(playbackLayout);
		}
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
		if (!showSolveBarAsScrubBar)
		{
			SolveBarWidget* bar = new SolveBarWidget(solve);
			layout->addWidget(bar);
		}

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
			splitLayout->setColumnMinimumWidth(5, 16);
			QLabel* movesLabel = new QLabel("Moves");
			movesLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			movesLabel->setAlignment(Qt::AlignCenter);
			movesLabel->setToolTip("Number of moves (outer turn metric)");
			splitLayout->addWidget(movesLabel, 0, 6);
			QLabel* moveCount = new QLabel(QString::asprintf("<span style='font-size:%fpt'>%d</span>",
				relativeFontSize(1.0f), (int)splits.moveCount));
			moveCount->setAlignment(Qt::AlignCenter);
			moveCount->setToolTip("Number of moves (outer turn metric)");
			splitLayout->addWidget(moveCount, 1, 6);
			QLabel* idleLabel = new QLabel("Idle");
			idleLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			idleLabel->setAlignment(Qt::AlignCenter);
			idleLabel->setToolTip("Time spent not performing moves (recognition time)");
			splitLayout->addWidget(idleLabel, 0, 7);
			QLabel* idleTime = new QLabel(SessionWidget::stringForTime(splits.idleTime));
			idleTime->setAlignment(Qt::AlignCenter);
			idleTime->setToolTip("Time spent not performing moves (recognition time)");
			splitLayout->addWidget(idleTime, 1, 7);
			QLabel* etpsLabel = new QLabel("eTPS");
			etpsLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			etpsLabel->setAlignment(Qt::AlignCenter);
			etpsLabel->setToolTip("Execution turns per second (excludes idle time)");
			splitLayout->addWidget(etpsLabel, 0, 8);
			QLabel* etps = new QLabel(QString::asprintf("<span style='font-size:%fpt'>%.2f</span>",
				relativeFontSize(1.0f), splits.etps));
			etps->setAlignment(Qt::AlignCenter);
			etps->setToolTip("Execution turns per second (excludes idle time)");
			splitLayout->addWidget(etps, 1, 8);
			QLabel* tpsLabel = new QLabel("TPS");
			tpsLabel->setFont(fontOfRelativeSize(0.8f, QFont::Thin));
			tpsLabel->setAlignment(Qt::AlignCenter);
			tpsLabel->setToolTip("Turns per second (includes idle time)");
			splitLayout->addWidget(tpsLabel, 0, 9);
			QLabel* tps = new QLabel(QString::asprintf("<span style='font-size:%fpt'>%.2f</span>",
				relativeFontSize(1.0f), splits.tps));
			tps->setAlignment(Qt::AlignCenter);
			tps->setToolTip("Turns per second (includes idle time)");
			splitLayout->addWidget(tps, 1, 9);
			columns = 9;

			m_playback = AnimatedMoveSequence(solve.solveMoves);
		}

		splitLayout->setColumnStretch(0, 1);
		splitLayout->setColumnStretch(columns + 1, 1);
		layout->addLayout(splitLayout);
	}

	setLayout(layout);

	m_playbackTimer = new QTimer(this);
	m_playbackTimer->setSingleShot(false);
	m_playbackTimer->setInterval(1000 / 60);
	connect(m_playbackTimer, &QTimer::timeout, this, &SolveWidget::updatePlayback);
}



QString SolveWidget::solveDetailsText()
{
	QString result = solveTimeText(m_solve);
	result += "   ";
	result += QString::fromStdString(m_solve.scramble.ToString());
	result += "   @";
	result += QDateTime::fromSecsSinceEpoch(m_solve.created).toString(Qt::DateFormat::TextDate);
	if (m_solve.solveMoves.moves.size() != 0)
	{
		result += "\nSolve:";
		for (auto i: m_solve.solveMoves.moves)
		{
			result += QString(" ");
			result += QString::fromStdString(CubeMoveSequence::MoveToString(i.move));
			result += QString::asprintf("@%d", (int) i.timestamp);
		}


	}
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


bool SolveWidget::solveFromText(const QString& text, Solve& solve)
{
	QStringList lines = text.split('\n', QString::SkipEmptyParts);
	if (lines.size() == 0)
		return false;

	int timePos = lines[0].lastIndexOf('@');
	if (timePos == -1)
		return false;
	solve.id = History::instance.idGenerator->GenerateId();
	QDateTime created = QDateTime::fromString(lines[0].mid(timePos + 1));
	if (!created.isValid())
		return false;
	solve.created = (time_t)created.toSecsSinceEpoch();
	solve.update.date = solve.created;
	solve.update.id = History::instance.idGenerator->GenerateId();
	solve.dirty = true;
	solve.penalty = 0;

	QStringList firstLineParts = lines[0].mid(0, timePos).split(' ', QString::SkipEmptyParts);
	if (firstLineParts.size() == 0)
		return false;

	QString timeStr = firstLineParts[0];
	if (timeStr == "DNF")
	{
		solve.ok = false;
	}
	else
	{
		int colonPos = timeStr.indexOf(':');
		bool ok;
		if (colonPos == -1)
		{
			double t = timeStr.toDouble(&ok);
			if (!ok)
				return false;
			solve.time = (int)(t * 1000.0f + 0.5f);
		}
		else
		{
			int minutes = timeStr.mid(0, colonPos).toInt(&ok);
			if (!ok)
				return false;
			double seconds = timeStr.mid(colonPos + 1).toDouble(&ok);
			if (!ok)
				return false;
			solve.time = (int)(seconds * 1000.0f + 0.5f) + minutes * 60000;
		}
		solve.ok = true;
	}

	for (int i = 1; i < firstLineParts.size(); i++)
	{
		QString moveStr = firstLineParts[i];
		CubeMove move;
		if (!CubeMoveSequence::MoveFromString(moveStr.toStdString(), move))
			return false;
		solve.scramble.moves.push_back(move);
	}

	if (lines.size() > 1)
	{
		QStringList secondLineParts = lines[1].split(' ', QString::SkipEmptyParts);
		if ((secondLineParts.size() >= 2) && (secondLineParts[0] == "Solve:"))
		{
			TimedCubeMoveSequence solveMoves;
			bool ok = true;
			for (int i = 1; i < secondLineParts.size(); i++)
			{
				QString moveStr = secondLineParts[i];
				int atPos = moveStr.indexOf('@');
				if (atPos == -1)
				{
					ok = false;
					break;
				}

				QString moveName = moveStr.mid(0, atPos);
				TimedCubeMove move;
				if (!CubeMoveSequence::MoveFromString(moveName.toStdString(), move.move))
				{
					ok = false;
					break;
				}

				move.timestamp = moveStr.mid(atPos + 1).toUInt(&ok);
				if (!ok)
					break;

				solveMoves.moves.push_back(move);
			}

			if (ok)
			{
				solve.solveMoves = solveMoves;
				solve.GenerateSplitTimesFromMoves();
			}
		}
	}

	return true;
}


void SolveWidget::startPlayback()
{
	if (m_playback.moves.size() == 0)
		return;
	if (!m_cube)
		return;
	if (m_playbackMoveIndex >= m_playback.moves.size())
	{
		// If at end of playback, restart playback
		m_playbackMoveIndex = 0;
		m_playbackTimestamp = -1000;
		m_cube->setCubeState(Cube3x3());
		m_cube->applyImmediate(m_solve.scramble);
		m_cube->update();
	}

	m_playbackRunning = true;
	m_lastPlaybackTick = chrono::steady_clock::now();
	m_playbackTimer->start();
	m_playButton->setPictures(m_pauseIcon, m_pauseHoverIcon);

	updateCurrentTimestampDisplay();
}


void SolveWidget::pausePlayback()
{
	m_playbackRunning = false;
	m_playbackTimer->stop();
	m_playButton->setPictures(m_playIcon, m_playHoverIcon);
}


void SolveWidget::seekInPlayback(int timestamp)
{
	pausePlayback();

	size_t oldIndex = m_playbackMoveIndex;

	// Find current move in playback sequence for this timestamp
	m_playbackMoveIndex = 0;
	m_playbackTimestamp = timestamp;
	for (size_t i = 0; i < m_playback.moves.size(); i++)
	{
		if ((int)m_playback.moves[i].timestamp > timestamp)
			break;
		m_playbackMoveIndex = i + 1;
	}

	if (m_playbackMoveIndex > oldIndex)
	{
		// Moving forward, animate moves in sequence
		for (size_t i = oldIndex; i < m_playbackMoveIndex; i++)
			m_cube->apply(m_playback.moves[i].move, 3.0f, true);
	}
	else if (m_playbackMoveIndex < oldIndex)
	{
		// Move backward, animate inverted moves
		for (size_t i = 1; i <= (oldIndex - m_playbackMoveIndex); i++)
			m_cube->apply(CubeMoveSequence::InvertedMove(m_playback.moves[oldIndex - i].move), 3.0f, true);
	}
	m_cube->update();

	updateCurrentTimestampDisplay();
}


void SolveWidget::updatePlayback()
{
	if (!m_cube)
		return;
	if (!m_playbackRunning)
	{
		m_playbackTimer->stop();
		return;
	}

	chrono::time_point<chrono::steady_clock> curTime = chrono::steady_clock::now();
	m_playbackTimestamp += (int)chrono::duration_cast<chrono::milliseconds>(curTime - m_lastPlaybackTick).count();

	// Update current move and play animations as they occur in the playback list
	while ((m_playbackMoveIndex < m_playback.moves.size()) &&
		(m_playbackTimestamp >= (int)m_playback.moves[m_playbackMoveIndex].timestamp))
	{
		m_cube->apply(m_playback.moves[m_playbackMoveIndex].move,
			m_playback.moves[m_playbackMoveIndex].tps);
		m_playbackMoveIndex++;
	}

	m_lastPlaybackTick = curTime;
	updateCurrentTimestampDisplay();

	if (m_playbackMoveIndex >= m_playback.moves.size())
		pausePlayback();
}


void SolveWidget::updateCurrentTimestampDisplay()
{
	if (m_playbackTimeLabel)
	{
		if (m_playbackMoveIndex >= m_playback.moves.size())
			m_playbackTimeLabel->setText(SessionWidget::stringForTime(m_solve.time - m_solve.penalty));
		else if (m_playbackTimestamp >= 0)
			m_playbackTimeLabel->setText(SessionWidget::stringForTime(m_playbackTimestamp));
		else
			m_playbackTimeLabel->setText(SessionWidget::stringForTime(0));
	}
	if (m_solveBar)
	{
		if (m_playbackMoveIndex >= m_playback.moves.size())
			m_solveBar->setCurrentPos(m_solve.time - m_solve.penalty);
		else if (m_playbackTimestamp >= 0)
			m_solveBar->setCurrentPos(m_playbackTimestamp);
		else
			m_solveBar->setCurrentPos(0);
	}
}
