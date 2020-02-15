#pragma once

#include <QtWidgets/QAbstractScrollArea>
#include "history.h"

struct SessionHistoryInfo
{
	std::shared_ptr<Session> session;
	int y, height;
	int rows, columns, columnWidth;
	int timeXOffset;
	int bestSolveTime, bestAvgOf5, bestAvgOf12, sessionAvg;
	Solve bestSolve;
};

class HistoryMode;

class HistoryElement
{
	QRect m_rect;
	std::vector<std::shared_ptr<HistoryElement>> m_children;

public:
	virtual ~HistoryElement() {}

	virtual QSize sizeHint() const { return QSize(m_rect.width(), m_rect.height()); }

	const QRect rect() const { return m_rect; }
	void setRect(const QRect& r) { m_rect = r; }
	void setRect(int x, int y, int w, int h) { m_rect = QRect(x, y, w, h); }

	bool contains(int x, int y) const { return m_rect.contains(x, y); }

	void move(int dx, int dy);

	virtual std::vector<std::shared_ptr<HistoryElement>> children() { return m_children; }
	void addChild(const std::shared_ptr<HistoryElement>& child) { m_children.push_back(child); }

	virtual void paint(QPainter& p, bool hovering) = 0;
	virtual bool interactable() const { return false; }
	virtual bool click(HistoryMode* parent, QMouseEvent* event) { (void)parent; (void)event; return false; }
	virtual bool hasHandCursor() const { return false; }
};

class HistoryAllTimeBestElement: public HistoryElement
{
	QString m_title;
	int m_best;

public:
	HistoryAllTimeBestElement(const QString& title, int best);

	virtual QSize sizeHint() const override;
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool hasHandCursor() const override { return true; }
};

class HistoryAllTimeBestSolveElement: public HistoryAllTimeBestElement
{
	Solve m_solve;

public:
	HistoryAllTimeBestSolveElement(const QString& title, int best, const Solve& solve);
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistoryAllTimeBestAverageElement: public HistoryAllTimeBestElement
{
	std::shared_ptr<Session> m_session;
	int m_start, m_size;

public:
	HistoryAllTimeBestAverageElement(const QString& title, int best,
		const std::shared_ptr<Session>& session, int start, int size);
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistorySessionElement: public HistoryElement
{
	std::shared_ptr<Session> m_session;

	int m_rows, m_columns, m_columnWidth;
	int m_bestSolveTime, m_bestAvgOf5, m_bestAvgOf12, m_sessionAvg;
	int m_bestAvgOf5Start, m_bestAvgOf12Start;
	Solve m_bestSolve;
	int* m_allTimeBestSolve;

public:
	HistorySessionElement(const std::shared_ptr<Session>& session, int x, int y, int width,
		int* allTimeBestSolve);
	virtual std::vector<std::shared_ptr<HistoryElement>> children() override;
	virtual void paint(QPainter& p, bool hovering) override;
};

class HistorySessionOptionsElement: public HistoryElement
{
	std::shared_ptr<Session> m_session;

public:
	HistorySessionOptionsElement(const std::shared_ptr<Session>& session);
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistorySessionSolveTimeElement: public HistoryElement
{
	std::shared_ptr<Session> m_session;
	int m_index;
	int m_bestSolveTime;
	int* m_allTimeBestSolve;

public:
	HistorySessionSolveTimeElement(const std::shared_ptr<Session>& session, int idx,
		int bestSolveTime, int* allTimeBestSolve);
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
	virtual bool hasHandCursor() const override { return true; }
};

class HistorySessionSolveOptionsElement: public HistoryElement
{
	std::shared_ptr<Session> m_session;
	int m_index;

public:
	HistorySessionSolveOptionsElement(const std::shared_ptr<Session>& session, int idx);
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistorySessionSolveRemoveElement: public HistoryElement
{
	std::shared_ptr<Session> m_session;
	int m_index;

public:
	HistorySessionSolveRemoveElement(const std::shared_ptr<Session>& session, int idx);
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistorySessionBestElement: public HistoryElement
{
	QString m_title;
	int m_best;

public:
	HistorySessionBestElement(const QString& title, int best);

	virtual QSize sizeHint() const override;
	virtual void paint(QPainter& p, bool hovering) override;
	virtual bool interactable() const override { return true; }
	virtual bool hasHandCursor() const override { return true; }
};

class HistorySessionBestSolveElement: public HistorySessionBestElement
{
	Solve m_solve;

public:
	HistorySessionBestSolveElement(const QString& title, int best, const Solve& solve);
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistorySessionBestAverageElement: public HistorySessionBestElement
{
	std::shared_ptr<Session> m_session;
	int m_start, m_size;

public:
	HistorySessionBestAverageElement(const QString& title, int best,
		const std::shared_ptr<Session>& session, int start, int size);
	virtual bool click(HistoryMode* parent, QMouseEvent* event) override;
};

class HistoryMode: public QAbstractScrollArea
{
	SolveType m_type = SOLVE_3X3X3;
	int m_bestSolveTime = -1;

	std::vector<std::shared_ptr<Session>> m_sessions;
	std::vector<std::shared_ptr<HistoryElement>> m_elements;
	std::shared_ptr<HistoryElement> m_hoverElement;

	void paintElement(QPainter& p, QPaintEvent* event, const std::shared_ptr<HistoryElement>& element);
	std::shared_ptr<HistoryElement> getInteractableElement(int x, int y, const std::shared_ptr<HistoryElement>& element);

protected:
	virtual void paintEvent(QPaintEvent* event) override;
	virtual void resizeEvent(QResizeEvent* event) override;
	virtual void mouseMoveEvent(QMouseEvent* event) override;
	virtual void mousePressEvent(QMouseEvent* event) override;
	virtual void leaveEvent(QEvent* event) override;
	virtual void scrollContentsBy(int dx, int dy) override;

public:
	HistoryMode(QWidget* parent);

	const std::vector<std::shared_ptr<Session>>& sessions() const { return m_sessions; }
	void updateHistory();

	static QString stringForDate(time_t date);
};
