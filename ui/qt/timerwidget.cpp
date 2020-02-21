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

	m_bluetoothUpdateTimer = new QTimer(this);
	m_bluetoothUpdateTimer->setSingleShot(false);
	m_bluetoothUpdateTimer->setInterval(25);
	connect(m_bluetoothUpdateTimer, &QTimer::timeout, this, &TimerWidget::updateBluetoothSolve);

	setAlignment(Qt::AlignCenter | Qt::AlignVCenter);

	setFont(QFont("Open Sans", m_fontSize, QFont::Light));
	updateText();
}


TimerWidget::~TimerWidget()
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
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
	case TIMER_BLUETOOTH_READY:
		m_prepareTime = chrono::steady_clock::now();
		m_state = TIMER_READY_TO_START;
		updateText();
		break;
	case TIMER_RUNNING:
		if (value() < 500)
			break;
		m_endTime = chrono::steady_clock::now();
		m_state = TIMER_STOPPED;
		m_bluetoothTimeOverride = false;
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
			m_solveMoves.moves.clear();
			m_bluetoothTimeOverride = false;
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
	if (m_bluetoothTimeOverride)
		return m_bluetoothTimeValue;
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


void TimerWidget::setBluetoothCube(const shared_ptr<BluetoothCube>& cube)
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
	{
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
		m_bluetoothCube.reset();
	}

	m_bluetoothCube = cube;
	if (m_bluetoothCube)
	{
		m_bluetoothCubeClient = make_shared<BluetoothCubeClient>();
		m_bluetoothCube->AddClient(m_bluetoothCubeClient);
		m_bluetoothUpdateTimer->start();
	}
	else
	{
		m_bluetoothUpdateTimer->stop();
		if (m_state == TIMER_BLUETOOTH_READY)
		{
			m_state = TIMER_STOPPED;
			m_updateTimer->stop();
			updateText();
			emit reset();
		}
	}
}


void TimerWidget::updateBluetoothSolve()
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
	{
		TimedCubeMoveSequence moves = m_bluetoothCubeClient->GetLatestMoves();
		if (moves.moves.size() != 0)
			m_bluetoothLastTimestamp = moves.moves[moves.moves.size() - 1].timestamp;

		if (m_state == TIMER_BLUETOOTH_READY)
		{
			if (moves.moves.size() != 0)
			{
				m_bluetoothStartTimestamp = moves.moves[0].timestamp;
				m_startTime = chrono::steady_clock::now();
				m_state = TIMER_RUNNING;
				m_bluetoothTimeOverride = false;
				emit started();
				updateText();

				m_solveMoves = moves;
			}
		}
		else if (m_state == TIMER_RUNNING)
		{
			m_solveMoves.moves.insert(m_solveMoves.moves.end(), moves.moves.begin(), moves.moves.end());

			if (m_bluetoothCube->GetCubeState().IsSolved())
			{
				m_endTime = chrono::steady_clock::now();

				int realTime = (int)chrono::duration_cast<chrono::milliseconds>(m_endTime - m_startTime).count();
				int bluetoothTime = m_bluetoothLastTimestamp - m_bluetoothStartTimestamp;

				if (((bluetoothTime - realTime) > -500) && ((bluetoothTime - realTime) < 500))
				{
					// If Bluetooth cube's timing information is reasonable, use it
					m_bluetoothTimeOverride = true;
					m_bluetoothTimeValue = bluetoothTime;
				}
				else if (m_solveMoves.moves.size() != 0)
				{
					// Rebase individual move times to use real time
					double ratio = (double)realTime / (double)bluetoothTime;
					for (size_t i = 1; i < m_solveMoves.moves.size(); i++)
						m_solveMoves.moves[i].timestamp -= m_solveMoves.moves[0].timestamp;
					m_solveMoves.moves[0].timestamp = 0;
					for (auto& i : m_solveMoves.moves)
						i.timestamp = (uint64_t)((double)i.timestamp * ratio);
					m_solveMoves.moves[m_solveMoves.moves.size() - 1].timestamp = (uint64_t)realTime;
				}

				m_state = TIMER_STOPPED;
				m_updateTimer->stop();
				updateText();
				emit completed();
				emit reset();
			}
		}
	}
}


void TimerWidget::readyForBluetoothSolve()
{
	if (m_bluetoothCube && m_bluetoothCubeClient && (m_state == TIMER_STOPPED))
	{
		m_bluetoothCubeClient->GetLatestMoves();

		m_state = TIMER_BLUETOOTH_READY;
		m_solveMoves.moves.clear();
		updateText();
		emit aboutToStart();
		m_updateTimer->start();
	}
}
