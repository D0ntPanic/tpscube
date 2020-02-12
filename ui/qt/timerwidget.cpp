#include "timerwidget.h"
#include "theme.h"

using namespace std;


TimerWidget::TimerWidget(QWidget* parent): QLabel(parent)
{
	m_startTime = chrono::steady_clock::now();
	m_endTime = m_startTime;

	m_updateTimer = new QTimer(this);
	m_updateTimer->setInterval(100);
	m_updateTimer->setSingleShot(false);
	connect(m_updateTimer, &QTimer::timeout, this, &TimerWidget::updateText);

	setAlignment(Qt::AlignCenter | Qt::AlignVCenter);

	setFont(QFont("Open Sans", m_fontSize, QFont::Light));
	updateText();
}


void TimerWidget::updateText()
{
	int val = value();
	int hs = (val + 5) / 10;
	int minutes = hs / 6000;
	int seconds = (hs / 100) % 60;
	hs %= 100;
	if (m_state == TIMER_STOPPED)
	{
		if (minutes > 0)
		{
			setText(QString::asprintf("<span style='font-size:%dpt'>%d:%02d</span><span style='font-size:%dpt'>.%02d</span>",
				m_fontSize, minutes, seconds, m_fontSize * 4 / 5, hs));
		}
		else
		{
			setText(QString::asprintf("<span style='font-size:%dpt'>%d</span><span style='font-size:%dpt'>.%02d</span>",
				m_fontSize, seconds, m_fontSize * 4 / 5, hs));
		}
	}
	else
	{
		if (minutes > 0)
		{
			setText(QString::asprintf("<span style='font-size:%dpt'>%d:%02d</span><span style='font-size:%dpt'>.%d</span>",
				m_fontSize, minutes, seconds, m_fontSize * 4 / 5, hs / 10));
		}
		else
		{
			setText(QString::asprintf("<span style='font-size:%dpt'>%d</span><span style='font-size:%dpt'>.%d</span>",
				m_fontSize, seconds, m_fontSize * 4 / 5, hs / 10));
		}
	}

	QPalette pal(palette());
	if (m_enabled)
	{
		if (m_state == TIMER_READY_TO_START)
		{
			if (chrono::duration_cast<chrono::milliseconds>(chrono::steady_clock::now() - m_prepareTime).count() < 500)
				pal.setColor(QPalette::Text, Theme::backgroundHighlight);
			else
				pal.setColor(QPalette::Text, Theme::green);
		}
		else
		{
			pal.setColor(QPalette::Text, Theme::content);
		}
	}
	else
	{
		pal.setColor(QPalette::Text, Theme::disabled);
	}
	setPalette(pal);
}


void TimerWidget::buttonDown()
{
	switch (m_state)
	{
	case TIMER_STOPPED:
		m_prepareTime = chrono::steady_clock::now();
		m_state = TIMER_READY_TO_START;
		updateText();
		emit aboutToStart();
		m_updateTimer->start();
		break;
	case TIMER_RUNNING:
		if (value() < 500)
			break;
		m_endTime = chrono::steady_clock::now();
		m_state = TIMER_STOPPED;
		m_updateTimer->stop();
		updateText();
		emit completed();
		emit reset();
		break;
	default:
		break;
	}
}


void TimerWidget::buttonUp()
{
	if (m_state == TIMER_READY_TO_START)
	{
		if (chrono::duration_cast<chrono::milliseconds>(chrono::steady_clock::now() - m_prepareTime).count() < 500)
		{
			m_state = TIMER_STOPPED;
			m_updateTimer->stop();
			updateText();
			emit reset();
		}
		else
		{
			m_startTime = chrono::steady_clock::now();
			m_state = TIMER_RUNNING;
			emit started();
			updateText();
		}
	}
}


int TimerWidget::value()
{
	if (m_state == TIMER_READY_TO_START)
		return 0;
	if (m_state == TIMER_RUNNING)
		m_endTime = chrono::steady_clock::now();
	return (int)chrono::duration_cast<chrono::milliseconds>(m_endTime - m_startTime).count();
}


void TimerWidget::setFontSize(int size)
{
	m_fontSize = size;
	setFont(QFont("Open Sans", m_fontSize, QFont::Light));
	updateText();
}


void TimerWidget::disable()
{
	m_enabled = false;
	updateText();
}


void TimerWidget::enable()
{
	m_enabled = true;
	updateText();
}
