#pragma once

#include <QtWidgets/QLabel>
#include <QtCore/QTimer>
#include <functional>
#include "theme.h"

class Subheading: public QWidget
{
	QLabel* m_label;

public:
	Subheading(const QString& name, const QColor& color = Theme::content, bool large = false);
	void setName(const QString& name);
};

class Heading: public Subheading
{
public:
	Heading(const QString& name);
};

class ThinLabel: public QLabel
{
public:
	ThinLabel(const QString& text);
};

class ClickableLabel: public QLabel
{
	Q_OBJECT

	std::function<void()> m_onClick, m_tooltip;
	QColor m_defaultColor, m_hoverColor;

	QTimer* m_hoverTimer;

protected:
	virtual void mousePressEvent(QMouseEvent* event);
	virtual void mouseMoveEvent(QMouseEvent* event);
	virtual void enterEvent(QEvent* event);
	virtual void leaveEvent(QEvent* event);

private slots:
	void hoverTooltip();

public:
	ClickableLabel(const QString& text, QColor defaultColor, QColor hoverColor,
		const std::function<void()>& func);
	void setColors(QColor defaultColor, QColor hoverColor);
	void setTooltipFunction(const std::function<void()>& func);
};
