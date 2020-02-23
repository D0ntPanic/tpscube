#pragma once

#include <QtWidgets/QWidget>
#include <QtGui/QPicture>
#include <chrono>
#include "history.h"
#include "scramblewidget.h"
#include "cube3x3widget.h"
#include "solvebarwidget.h"
#include "utilwidgets.h"
#include "solvestatswidget.h"

class SolveWidget: public QWidget
{
	Q_OBJECT

	Solve m_solve;
	ScrambleWidget* m_scramble;
	QLabel* m_timer;
	Cube3x3Widget* m_cube = nullptr;

	AnimatedMoveSequence m_playback;
	size_t m_playbackMoveIndex = 0;
	int m_playbackTimestamp = 0;
	bool m_playbackRunning = false;
	QTimer* m_playbackTimer;
	std::chrono::time_point<std::chrono::steady_clock> m_lastPlaybackTick;
	QLabel* m_playbackTimeLabel = nullptr;
	SolveBarWidget* m_solveBar = nullptr;
	SolveStatsWidget* m_stats = nullptr;

	ClickableLabel* m_playButton = nullptr;
	QPicture m_playIcon, m_playHoverIcon;
	QPicture m_pauseIcon, m_pauseHoverIcon;

	void startPlayback();
	void pausePlayback();
	void updateCurrentTimestampDisplay();

private slots:
	void seekInPlayback(int timestamp);
	void updatePlayback();

public:
	SolveWidget(const Solve& solve, bool fullDetails = false);

	QString solveDetailsText();
	static QString solveTimeText(const Solve& solve);
	static bool solveFromText(const QString& text, Solve& solve);
};
