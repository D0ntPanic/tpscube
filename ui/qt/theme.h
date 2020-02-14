#pragma once

#include <QtGui/QColor>
#include <QtGui/QFont>

namespace Theme
{
	extern QColor background;
	extern QColor backgroundDark;
	extern QColor backgroundWindow;
	extern QColor backgroundHighlight;
	extern QColor content;
	extern QColor light;
	extern QColor disabled;
	extern QColor deselected;
	extern QColor selection;
	extern QColor selectionLight;
	extern QColor green;
	extern QColor red;
	extern QColor blue;
	extern QColor cyan;
	extern QColor lightCyan;
	extern QColor orange;
	extern QColor yellow;
	extern QColor magenta;
}

QColor MixColor(const QColor& a, const QColor& b, uint8_t alpha = 128);
float relativeFontSize(float mult);
QFont fontOfRelativeSize(float mult, int weight = QFont::Normal, bool italic = false);
