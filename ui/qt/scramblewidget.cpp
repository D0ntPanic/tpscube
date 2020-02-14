#include <QtCore/QRandomGenerator>
#include "scramblewidget.h"
#include "theme.h"

#define MOVES_PER_ROW 8


int QtRandomSource::Next(int range)
{
	return QRandomGenerator::global()->bounded(range);
}


ScrambleWidget::ScrambleWidget(QWidget* parent): QLabel(parent)
{
	setFont(QFont("Open Sans", 26, QFont::Light));
	setAlignment(Qt::AlignCenter | Qt::AlignVCenter);

	QPalette pal(palette());
	pal.setColor(QPalette::Text, Theme::blue);
	setPalette(pal);
}


void ScrambleWidget::updateText()
{
	size_t rows = m_maxMoveCount / MOVES_PER_ROW;
	size_t neededRows = m_scramble.moves.size() / MOVES_PER_ROW;
	if (neededRows == 0)
		neededRows = 1;
	if (!m_reserveVerticalSpace)
		rows = neededRows;
	size_t cols = (m_scramble.moves.size() + (neededRows - 1)) / neededRows;

	QString text;
	size_t moveIdx = 0;
	for (size_t i = 0; i < rows; i++)
	{
		if (i > 0)
			text += "\n";
		for (size_t j = 0; j < cols; j++)
		{
			if (moveIdx >= m_scramble.moves.size())
				break;
			if (j > 0)
				text += "  ";
			text += QString::fromStdString(CubeMoveSequence::MoveToString(m_scramble.moves[moveIdx++]));
		}
	}

	setText(text);
}


void ScrambleWidget::setScramble(const CubeMoveSequence& scramble)
{
	m_scramble = scramble;
	updateText();
}


void ScrambleWidget::invalidateScramble()
{
	m_scramble.moves.clear();
	updateText();
}


void ScrambleWidget::setFontSize(int size)
{
	setFont(QFont("Open Sans", size, QFont::Light));
}


void ScrambleWidget::setMaxMoveCount(size_t count)
{
	m_maxMoveCount = count;
	updateText();
}


void ScrambleWidget::setReserveVerticalSpace(bool enable)
{
	m_reserveVerticalSpace = enable;
	updateText();
}
