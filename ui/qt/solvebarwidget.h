#pragma once

#include <QtWidgets/QWidget>
#include "history.h"

class SolveBarWidget: public QWidget
{
	Solve m_solve;
	int m_barHeight = 2;
	float m_barWidth = 1.0f;
	int m_topPadding = 0;
	int m_bottomPadding = 0;

protected:
	virtual void paintEvent(QPaintEvent* event) override;

public:
	SolveBarWidget(const Solve& solve);
	void setBarRelativeWidth(float width) { m_barWidth = width; }
	void setBarHeight(int height) { m_barHeight = height; }
	void setPadding(int top, int bot);
	virtual QSize sizeHint() const override;
};
