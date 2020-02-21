#include <QtGui/QPainter>
#include "solvebarwidget.h"
#include "theme.h"


SolveBarWidget::SolveBarWidget(const Solve& solve): m_solve(solve)
{
}


void SolveBarWidget::paintEvent(QPaintEvent*)
{
	QPainter p(this);

	int w = (int)(width() * m_barWidth) - 1;
	int h = height() - (4 + m_topPadding + m_bottomPadding);
	int totalTime = m_solve.time - m_solve.penalty;

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

		p.fillRect(QRect(0, 2 + m_topPadding, crossStartX, h),
			MixColor(Theme::selection, Theme::red, 64));
		p.fillRect(QRect(crossStartX, 2 + m_topPadding, crossEndX - crossStartX, h), Theme::red);
		p.fillRect(QRect(crossEndX, 2 + m_topPadding, f2lPair1StartX - crossEndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(f2lPair1StartX, 2 + m_topPadding, f2lPair1EndX - f2lPair1StartX, h), Theme::blue);
		p.fillRect(QRect(f2lPair1EndX, 2 + m_topPadding, f2lPair2StartX - f2lPair1EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(f2lPair2StartX, 2 + m_topPadding, f2lPair2EndX - f2lPair2StartX, h), Theme::blue);
		p.fillRect(QRect(f2lPair2EndX, 2 + m_topPadding, f2lPair3StartX - f2lPair2EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(f2lPair3StartX, 2 + m_topPadding, f2lPair3EndX - f2lPair3StartX, h), Theme::blue);
		p.fillRect(QRect(f2lPair3EndX, 2 + m_topPadding, f2lPair4StartX - f2lPair3EndX, h),
			MixColor(Theme::selection, Theme::blue, 64));
		p.fillRect(QRect(f2lPair4StartX, 2 + m_topPadding, f2lPair4EndX - f2lPair4StartX, h), Theme::blue);
		p.fillRect(QRect(f2lPair4EndX, 2 + m_topPadding, ollCrossStartX - f2lPair4EndX, h),
			MixColor(Theme::selection, Theme::yellow, 64));
		p.fillRect(QRect(ollCrossStartX, 2 + m_topPadding, ollCrossEndX - ollCrossStartX, h), Theme::yellow);
		p.fillRect(QRect(ollCrossEndX, 2 + m_topPadding, ollStartX - ollCrossEndX, h),
			MixColor(Theme::selection, Theme::yellow, 64));
		p.fillRect(QRect(ollStartX, 2 + m_topPadding, ollEndX - ollStartX, h), Theme::yellow);
		p.fillRect(QRect(ollEndX, 2 + m_topPadding, pllCornerStartX - ollEndX, h),
			MixColor(Theme::selection, Theme::green, 64));
		p.fillRect(QRect(pllCornerStartX, 2 + m_topPadding, pllCornerEndX - pllCornerStartX, h), Theme::green);
		p.fillRect(QRect(pllCornerEndX, 2 + m_topPadding, pllStartX - pllCornerEndX, h),
			MixColor(Theme::selection, Theme::green, 64));
		p.fillRect(QRect(pllStartX, 2 + m_topPadding, pllEndX - pllStartX, h), Theme::green);

		p.fillRect(QRect(f2lPair1EndX, 1 + m_topPadding, 1, h + 2), Theme::blue);
		p.fillRect(QRect(f2lPair2EndX, 1 + m_topPadding, 1, h + 2), Theme::blue);
		p.fillRect(QRect(f2lPair3EndX, 1 + m_topPadding, 1, h + 2), Theme::blue);
		p.fillRect(QRect(ollCrossEndX, 1 + m_topPadding, 1, h + 2), Theme::yellow);
		p.fillRect(QRect(pllCornerEndX, 1 + m_topPadding, 1, h + 2), Theme::green);

		p.fillRect(QRect(crossEndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(f2lPair4EndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(ollEndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(pllEndX, m_topPadding, 1, h + 4), Theme::content);
	}
	else
	{
		int crossEndX = (int)((float)w * (float)m_solve.crossTime / (float)totalTime);
		int f2lEndX = (int)((float)w * (float)m_solve.f2lPairTimes[3] / (float)totalTime);
		int ollEndX = (int)((float)w * (float)m_solve.ollFinishTime / (float)totalTime);
		int pllEndX = w;

		p.fillRect(QRect(0, 2 + m_topPadding, crossEndX, h), Theme::red);
		p.fillRect(QRect(crossEndX, 2 + m_topPadding, f2lEndX - crossEndX, h), Theme::blue);
		p.fillRect(QRect(f2lEndX, 2 + m_topPadding, ollEndX - f2lEndX, h), Theme::yellow);
		p.fillRect(QRect(ollEndX, 2 + m_topPadding, pllEndX - ollEndX, h), Theme::green);

		for (size_t i = 0; i < 3; i++)
		{
			if (m_solve.f2lPairTimes[i] != 0)
			{
				int pairX = (int)((float)w * (float)m_solve.f2lPairTimes[i] / (float)totalTime);
				p.fillRect(QRect(pairX, 1 + m_topPadding, 1, h + 2), Theme::blue);
			}
		}

		if (m_solve.ollCrossTime != 0)
		{
			int ollCrossX = (int)((float)w * (float)m_solve.ollCrossTime / (float)totalTime);
			p.fillRect(QRect(ollCrossX, 1 + m_topPadding, 1, h + 2), Theme::yellow);
		}

		if (m_solve.pllCornerTime != 0)
		{
			int pllCornerX = (int)((float)w * (float)m_solve.pllCornerTime / (float)totalTime);
			p.fillRect(QRect(pllCornerX, 1 + m_topPadding, 1, h + 2), Theme::green);
		}

		p.fillRect(QRect(crossEndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(f2lEndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(ollEndX, m_topPadding, 1, h + 4), Theme::content);
		p.fillRect(QRect(pllEndX, m_topPadding, 1, h + 4), Theme::content);
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
