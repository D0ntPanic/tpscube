#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QApplication>
#include "utilwidgets.h"

using namespace std;


Subheading::Subheading(const QString& name, const QColor& color, bool large)
{
	QVBoxLayout* layout = new QVBoxLayout(this);
	layout->setContentsMargins(0, 0, 0, 0);

	m_label = new QLabel(name);
	if (large)
		m_label->setFont(QFont("Open Sans", 15, QFont::DemiBold));
	else
		m_label->setFont(QFont("Open Sans", 12, QFont::DemiBold));
	QPalette headerPalette(palette());
	headerPalette.setColor(QPalette::WindowText, color);
	m_label->setPalette(headerPalette);
	layout->addWidget(m_label);

	QFrame* frame = new QFrame();
	frame->setFrameShape(QFrame::HLine);
	frame->setFrameShadow(QFrame::Plain);
	QPalette framePalette(palette());
	framePalette.setColor(QPalette::WindowText, color.darker());
	frame->setPalette(framePalette);
	layout->addWidget(frame);

	setLayout(layout);
}


void Subheading::setName(const QString& name)
{
	m_label->setText(name);
}


Heading::Heading(const QString& name): Subheading(name, Theme::blue, true)
{
}


ThinLabel::ThinLabel(const QString& text): QLabel(text)
{
	setFont(QFont("Open Sans", 12, QFont::Thin));
}


ClickableLabel::ClickableLabel(const QString& text, QColor defaultColor, QColor hoverColor,
	const function<void()>& func): QLabel(text), m_onClick(func),
	m_defaultColor(defaultColor), m_hoverColor(hoverColor)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_defaultColor);
	setPalette(pal);
}


void ClickableLabel::mousePressEvent(QMouseEvent*)
{
	m_onClick();
}


void ClickableLabel::enterEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_hoverColor);
	setPalette(pal);
}


void ClickableLabel::leaveEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_defaultColor);
	setPalette(pal);
}
