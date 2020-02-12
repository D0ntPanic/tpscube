#include <QtWidgets/QHBoxLayout>
#include "topbar.h"
#include "theme.h"
#include "utilwidgets.h"

using namespace std;


ModeLabel::ModeLabel(const QString& text, const function<void()>& func):
	QLabel(text), m_onClick(func)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, Theme::disabled);
	setPalette(pal);
	setFont(QFont("Open Sans", 13, QFont::Light));
}


void ModeLabel::mousePressEvent(QMouseEvent*)
{
	m_onClick();
}


void ModeLabel::enterEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::content);
	setPalette(pal);
}


void ModeLabel::leaveEvent(QEvent*)
{
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::disabled);
	setPalette(pal);
}


void ModeLabel::setActive(bool active)
{
	m_active = active;
	setFont(QFont("Open Sans", 13, active ? QFont::Bold : QFont::Light));
	QPalette pal = palette();
	pal.setColor(QPalette::WindowText, m_active ? Theme::green : Theme::disabled);
	setPalette(pal);
}


TopBar::TopBar(QWidget* parent): QWidget(parent)
{
	setBackgroundRole(QPalette::Window);
	setAutoFillBackground(true);

	QHBoxLayout* layout = new QHBoxLayout();
	m_timerMode = new ModeLabel("Timer", [this]() { timerModeClicked(); });
	layout->addWidget(m_timerMode);
	layout->addSpacing(12);
	m_historyMode = new ModeLabel("History", [this]() { historyModeClicked(); });
	layout->addWidget(m_historyMode);
	layout->addSpacing(12);
	m_graphMode = new ModeLabel("Graphs", [this]() { graphModeClicked(); });
	layout->addWidget(m_graphMode);
	layout->addSpacing(12);
	m_algorithmMode = new ModeLabel("Algorithms", [this]() { algorithmModeClicked(); });
	layout->addWidget(m_algorithmMode);
	layout->addSpacing(12);
	m_timerMode->setActive(true);
	m_graphMode->setVisible(false);
	m_algorithmMode->setVisible(false);

	layout->addStretch(1);

	setLayout(layout);
}


void TopBar::timerModeClicked()
{
	m_timerMode->setActive(true);
	m_historyMode->setActive(false);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(false);
	emit showTimer();
}


void TopBar::historyModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(true);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(false);
	emit showHistory();
}


void TopBar::graphModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(false);
	m_graphMode->setActive(true);
	m_algorithmMode->setActive(false);
	emit showGraphs();
}


void TopBar::algorithmModeClicked()
{
	m_timerMode->setActive(false);
	m_historyMode->setActive(false);
	m_graphMode->setActive(false);
	m_algorithmMode->setActive(true);
	emit showAlgorithms();
}
