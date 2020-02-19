#pragma once

#include <QtWidgets/QLabel>
#include "utilwidgets.h"

class ModeLabel: public QLabel
{
	bool m_active = false;
	std::function<void()> m_onClick;

protected:
	virtual void mousePressEvent(QMouseEvent* event);
	virtual void enterEvent(QEvent* event);
	virtual void leaveEvent(QEvent* event);

public:
	ModeLabel(const QString& text, const std::function<void()>& func);
	void setActive(bool active);
};

class TopBar: public QWidget
{
	Q_OBJECT

	ModeLabel* m_timerMode;
	ModeLabel* m_historyMode;
	ModeLabel* m_graphMode;
	ModeLabel* m_algorithmMode;
	ClickableLabel* m_bluetooth;

	void timerModeClicked();
	void historyModeClicked();
	void graphModeClicked();
	void algorithmModeClicked();
	void bluetoothClicked();

public:
	TopBar(QWidget* parent);

signals:
	void showTimer();
	void showHistory();
	void showGraphs();
	void showAlgorithms();
};
