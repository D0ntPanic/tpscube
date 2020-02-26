#pragma once

#include <QtWidgets/QScrollArea>
#include "utilwidgets.h"
#include "history.h"

class SettingsMode: public QScrollArea
{
	Q_OBJECT

	void exportSolves();
	void importSolves();

	bool importNativeJson(const QJsonObject& solveData, std::vector<std::shared_ptr<Session>>& result);
	bool importCstimerJson(const QJsonObject& solveData, std::vector<std::shared_ptr<Session>>& result);

public:
	SettingsMode(QWidget* parent);
};
