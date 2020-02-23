#include <QtGui/QPainter>
#include <QtGui/QPolygon>
#include <QtGui/QMouseEvent>
#include "solvebarwidget.h"
#include "theme.h"


SolveBarWidget::SolveBarWidget(const Solve& solve): m_solve(solve)
{
}


void SolveBarWidget::paintEvent(QPaintEvent*)
{
	QPainter p(this);

	int x = 0;
	int w = (int)(width() * m_barWidth) - 1;
	int h = m_barHeight;
	int y = m_topPadding + ((height() - (m_barHeight + 4 + m_topPadding + m_bottomPadding)) / 2);
	int totalTime = m_solve.time - m_solve.penalty;

	if (m_showCurrentPos)
	{
		w -= 6;
		x += 3;
	}

	if (m_solve.solveMoves.moves.size() != 0)
	{
		DetailedSplitTimes splits = m_solve.GenerateDetailedSplitTimes();

		int crossStartX = (int)((float)w * (float)splits.cross.firstMoveTime / (float)totalTime);
		int crossEndX = (int)((float)w * (float)splits.cross.finishTime / (float)totalTime);
		int f2lPair1StartX = (int)((float)w * (float)splits.f2lPair[0].firstMoveTime / (float)totalTime);
		int f2lPair1EndX = (int)((float)w * (float)splits.f2lPair[0].finishTime / (float)totalTime);
		int f2lPair2StartX = (int)((float)w * (float)splits.f2lPair[1].firstMoveTime / (float)totalTime);
		int f2lPair2EndX = (int)((float)w * (float)splits.f2lPair[1].finishTime / (float)totalTime);
		int f2lPair3StartX = (int)((float)w * (float)splits.f2lPair[2].firstMoveTime / (float)totalTime);
		int f2lPair3EndX = (int)((float)w * (float)splits.f2lPair[2].finishTime / (float)totalTime);
		int f2lPair4StartX = (int)((float)w * (float)splits.f2lPair[3].firstMoveTime / (float)totalTime);
		int f2lPair4EndX = (int)((float)w * (float)splits.f2lPair[3].finishTime / (float)totalTime);
		int ollCrossStartX = (int)((float)w * (float)splits.ollCross.firstMoveTime / (float)totalTime);
		int ollCrossEndX = (int)((float)w * (float)splits.ollCross.finishTime / (float)totalTime);
		int ollStartX = (int)((float)w * (float)splits.ollFinish.firstMoveTime / (float)totalTime);
		int ollEndX = (int)((float)w * (float)splits.ollFinish.finishTime / (float)totalTime);
		int pllCornerStartX = (int)((float)w * (float)splits.pllCorner.firstMoveTime / (float)totalTime);
		int pllCornerEndX = (int)((float)w * (float)splits.pllCorner.finishTime / (float)totalTime);
		int pllStartX = (int)((float)w * (float)splits.pllFinish.firstMoveTime / (float)totalTime);
		int pllEndX = (int)((float)w * (float)splits.pllFinish.finishTime / (float)totalTime);

		p.fillRect(QRect(x, 2 + y, crossStartX, h),
			MixColor(Theme::selection, Theme::red, 64));
		p.fillRect(QRect(x + crossStartX, 2 + y, crossEndX - crossStartX, h), Theme::red);
		p.fillRect(QRect(x + crossEndX, 2 + y, f2lPair1StartX - crossEndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(x + f2lPair1StartX, 2 + y, f2lPair1EndX - f2lPair1StartX, h), Theme::blue);
		p.fillRect(QRect(x + f2lPair1EndX, 2 + y, f2lPair2StartX - f2lPair1EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(x + f2lPair2StartX, 2 + y, f2lPair2EndX - f2lPair2StartX, h), Theme::blue);
		p.fillRect(QRect(x + f2lPair2EndX, 2 + y, f2lPair3StartX - f2lPair2EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(x + f2lPair3StartX, 2 + y, f2lPair3EndX - f2lPair3StartX, h), Theme::blue);
		p.fillRect(QRect(x + f2lPair3EndX, 2 + y, f2lPair4StartX - f2lPair3EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(x + f2lPair4StartX, 2 + y, f2lPair4EndX - f2lPair4StartX, h), Theme::blue);
		p.fillRect(QRect(x + f2lPair4EndX, 2 + y, ollCrossStartX - f2lPair4EndX, h),
			MixColor(Theme::selection, Theme::yellow, 64));
		p.fillRect(QRect(x + ollCrossStartX, 2 + y, ollCrossEndX - ollCrossStartX, h), Theme::yellow);
		p.fillRect(QRect(x + ollCrossEndX, 2 + y, ollStartX - ollCrossEndX, h),
			MixColor(Theme::selection, Theme::yellow, 64));
		p.fillRect(QRect(x + ollStartX, 2 + y, ollEndX - ollStartX, h), Theme::yellow);
		p.fillRect(QRect(x + ollEndX, 2 + y, pllCornerStartX - ollEndX, h),
			MixColor(Theme::selection, Theme::green, 64));
		p.fillRect(QRect(x + pllCornerStartX, 2 + y, pllCornerEndX - pllCornerStartX, h), Theme::green);
		p.fillRect(QRect(x + pllCornerEndX, 2 + y, pllStartX - pllCornerEndX, h),
			MixColor(Theme::selection, Theme::green, 64));
		p.fillRect(QRect(x + pllStartX, 2 + y, pllEndX - pllStartX, h), Theme::green);

		p.fillRect(QRect(x + f2lPair1EndX, 1 + y, 1, h + 2), Theme::blue);
		p.fillRect(QRect(x + f2lPair2EndX, 1 + y, 1, h + 2), Theme::blue);
		p.fillRect(QRect(x + f2lPair3EndX, 1 + y, 1, h + 2), Theme::blue);
		p.fillRect(QRect(x + ollCrossEndX, 1 + y, 1, h + 2), Theme::yellow);
		p.fillRect(QRect(x + pllCornerEndX, 1 + y, 1, h + 2), Theme::green);

		p.fillRect(QRect(x + crossEndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + f2lPair4EndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + ollEndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + pllEndX, y, 1, h + 4), Theme::content);
	}
	else
	{
		int crossEndX = (int)((float)w * (float)m_solve.crossTime / (float)totalTime);
		int f2lEndX = (int)((float)w * (float)m_solve.f2lPairTimes[3] / (float)totalTime);
		int ollEndX = (int)((float)w * (float)m_solve.ollFinishTime / (float)totalTime);
		int pllEndX = w;

		p.fillRect(QRect(x, 2 + y, crossEndX, h), Theme::red);
		p.fillRect(QRect(x + crossEndX, 2 + y, f2lEndX - crossEndX, h), Theme::blue);
		p.fillRect(QRect(x + f2lEndX, 2 + y, ollEndX - f2lEndX, h), Theme::yellow);
		p.fillRect(QRect(x + ollEndX, 2 + y, pllEndX - ollEndX, h), Theme::green);

		for (size_t i = 0; i < 3; i++)
		{
			if (m_solve.f2lPairTimes[i] != 0)
			{
				int pairX = (int)((float)w * (float)m_solve.f2lPairTimes[i] / (float)totalTime);
				p.fillRect(QRect(x + pairX, 1 + y, 1, h + 2), Theme::blue);
			}
		}

		if (m_solve.ollCrossTime != 0)
		{
			int ollCrossX = (int)((float)w * (float)m_solve.ollCrossTime / (float)totalTime);
			p.fillRect(QRect(x + ollCrossX, 1 + y, 1, h + 2), Theme::yellow);
		}

		if (m_solve.pllCornerTime != 0)
		{
			int pllCornerX = (int)((float)w * (float)m_solve.pllCornerTime / (float)totalTime);
			p.fillRect(QRect(x + pllCornerX, 1 + y, 1, h + 2), Theme::green);
		}

		p.fillRect(QRect(x + crossEndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + f2lEndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + ollEndX, y, 1, h + 4), Theme::content);
		p.fillRect(QRect(x + pllEndX, y, 1, h + 4), Theme::content);
	}

	if (m_showCurrentPos)
	{
		int ofs = (m_currentPos * w) / totalTime;
		QVector<QPoint> pts;
		pts.push_back(QPoint(x + ofs - 2, y - 3));
		pts.push_back(QPoint(x + ofs + 2, y - 3));
		pts.push_back(QPoint(x + ofs, y - 1));
		p.setPen(Theme::content);
		p.setBrush(Theme::content);
		p.drawPolygon(QPolygon(pts));
		pts.clear();
		pts.push_back(QPoint(x + ofs - 2, y + h + 6));
		pts.push_back(QPoint(x + ofs + 2, y + h + 6));
		pts.push_back(QPoint(x + ofs, y + h + 4));
		p.drawPolygon(QPolygon(pts));
	}
}


void SolveBarWidget::setPadding(int top, int bot)
{
	m_topPadding = top;
	m_bottomPadding = bot;
}


QSize SolveBarWidget::sizeHint() const
{
	return QSize(128, m_barHeight + m_topPadding + m_bottomPadding + 4);
}


void SolveBarWidget::setCurrentPos(int ms)
{
	m_currentPos = ms;
	update();
}


void SolveBarWidget::reportSeekForX(int x)
{
	int w = (int)(width() * m_barWidth) - 1;
	if (m_showCurrentPos)
	{
		w -= 6;
		x -= 3;
	}

	int totalTime = m_solve.time - m_solve.penalty;
	int ms = (x * totalTime) / w;
	if (ms < 0)
		ms = 0;
	if (ms > totalTime)
		ms = totalTime;
	emit seek(ms);
}


void SolveBarWidget::mousePressEvent(QMouseEvent* event)
{
	m_buttonDown = true;
	reportSeekForX(event->x());
}


void SolveBarWidget::mouseReleaseEvent(QMouseEvent*)
{
	m_buttonDown = false;
}


void SolveBarWidget::mouseMoveEvent(QMouseEvent* event)
{
	if (m_buttonDown)
		reportSeekForX(event->x());
}
