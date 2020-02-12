#pragma once

#include <QtWidgets/QAbstractScrollArea>
#include "history.h"

struct SessionHistoryInfo
{
	std::shared_ptr<Session> session;
	int y, height;
	int rows, columns, columnWidth;
	int timeXOffset;
	int bestSolve, bestAvgOf5, bestAvgOf12, sessionAvg;
};

class HistoryMode: public QAbstractScrollArea
{
	SolveType m_type = SOLVE_3X3X3;
	std::vector<SessionHistoryInfo> m_sessions;
	int m_bestSolve = -1;
	int m_bestAvgOf5 = -1;
	int m_bestAvgOf12 = -1;

	std::shared_ptr<Session> m_hoverSession;
	int m_hoverSolve = -1;
	int m_hoverIcon = -1;

	void paintAllTimeBest(QPainter& p, int x, const QString& title, int best);
	void paintSessionBest(QPainter& p, int& x, int y, const QString& title, int best);

protected:
	virtual void paintEvent(QPaintEvent* event) override;
	virtual void resizeEvent(QResizeEvent* event) override;
	virtual void mouseMoveEvent(QMouseEvent* event) override;
	virtual void mousePressEvent(QMouseEvent* event) override;
	virtual void leaveEvent(QEvent* event) override;

public:
	HistoryMode(QWidget* parent);

	void updateHistory();

	static QString stringForDate(time_t date);
};
