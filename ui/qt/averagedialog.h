#pragma once

#include <QtWidgets/QDialog>
#include "averagewidget.h"

class AverageDialog: public QDialog
{
	Q_OBJECT
	AverageWidget* m_average;

private slots:
	void copy();

public:
	AverageDialog(const std::vector<Solve>& solves);
};
