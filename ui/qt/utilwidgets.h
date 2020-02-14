#pragma once

#include <QtWidgets/QLabel>
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
	std::function<void()> m_onClick;
	QColor m_defaultColor, m_hoverColor;

protected:
	virtual void mousePressEvent(QMouseEvent* event);
	virtual void enterEvent(QEvent* event);
	virtual void leaveEvent(QEvent* event);

public:
	ClickableLabel(const QString& text, QColor defaultColor, QColor hoverColor,
		const std::function<void()>& func);
	void setColors(QColor defaultColor, QColor hoverColor);
};
