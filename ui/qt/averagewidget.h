#pragma once

#include <QtWidgets/QWidget>
#include <QtWidgets/QLabel>
#include "history.h"

class AverageWidget: public QWidget
{
	std::vector<Solve> m_solves;
	size_t m_highest, m_lowest;
	QLabel* m_timer;

public:
	AverageWidget(const std::vector<Solve>& solves, bool fullDetails = false);

	QString averageDetailsText();
};
