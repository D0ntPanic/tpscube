#pragma once

#include <QtWidgets/QScrollArea>
#include "utilwidgets.h"

class SettingsMode: public QScrollArea
{
	Q_OBJECT

	void exportSolves();
	void importSolves();

public:
	SettingsMode(QWidget* parent);
};
