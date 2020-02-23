#pragma once

#include <QtWidgets/QWidget>
#include "history.h"

class SolveBarWidget: public QWidget
{
	Q_OBJECT

	Solve m_solve;
	int m_barHeight = 2;
	float m_barWidth = 1.0f;
	int m_topPadding = 0;
	int m_bottomPadding = 0;
	bool m_showCurrentPos = false;
	int m_currentPos = 0;
	bool m_buttonDown = false;

	void reportSeekForX(int x);

protected:
	virtual void paintEvent(QPaintEvent* event) override;
	virtual void mousePressEvent(QMouseEvent* event) override;
	virtual void mouseReleaseEvent(QMouseEvent* event) override;
	virtual void mouseMoveEvent(QMouseEvent* event) override;

public:
	SolveBarWidget(const Solve& solve);
	void setBarRelativeWidth(float width) { m_barWidth = width; }
	void setBarHeight(int height) { m_barHeight = height; }
	void setPadding(int top, int bot);
	void setShowCurrentPos(bool show) { m_showCurrentPos = show; }
	void setCurrentPos(int ms);
	virtual QSize sizeHint() const override;

signals:
	void seek(int ms);
};
