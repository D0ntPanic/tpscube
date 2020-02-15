#pragma once

#include <QtWidgets/QFrame>

class Tooltip: public QFrame
{
	static Tooltip* m_activeTooltip;

protected:
	bool eventFilter(QObject* obj, QEvent* event) override;

public:
	Tooltip(QWidget* contents);
	~Tooltip();
	void show(QWidget* srcWidget);
};
