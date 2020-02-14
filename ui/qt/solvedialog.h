#pragma once

#include <QtWidgets/QDialog>
#include "solvewidget.h"

class SolveDialog: public QDialog
{
	Q_OBJECT
	SolveWidget* m_solve;

private slots:
	void copy();

public:
	SolveDialog(const Solve& solve);
};
