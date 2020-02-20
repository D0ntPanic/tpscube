#include <QtCore/QUuid>
#include "timermode.h"
#include "cube3x3.h"
#include "scramblewidget.h"

using namespace std;


ScrambleThread::ScrambleThread(QObject* owner): QThread(owner)
{
}


void ScrambleThread::run()
{
	while (m_running)
	{
		m_mutex.lock();
		if (m_running && !m_requestPending)
			m_cond.wait(&m_mutex);
	
		if (!m_running)
		{
			m_mutex.unlock();
			break;
		}
		if (!m_requestPending)
		{
			m_mutex.unlock();
			continue;
		}

		shared_ptr<Scrambler> scrambler = m_scrambler;
		m_requestPending = false;

		m_mutex.unlock();

		QtRandomSource rng;
		CubeMoveSequence result = scrambler->GetScramble(rng);

		m_mutex.lock();
		if (!m_requestPending)
		{
			m_result = result;
			emit scrambleGenerated();
		}
		m_mutex.unlock();
	}
}


void ScrambleThread::stop()
{
	m_running = false;
	m_cond.notify_all();
}


void ScrambleThread::requestScramble(const shared_ptr<Scrambler>& scrambler)
{
	QMutexLocker lock(&m_mutex);
	m_requestPending = true;
	m_scrambler = scrambler;
	m_cond.notify_one();
}


CubeMoveSequence ScrambleThread::scramble()
{
	QMutexLocker lock(&m_mutex);
	return m_result;
}


TimerMode::TimerMode(QWidget* parent): QWidget(parent)
{
	setBackgroundRole(QPalette::Base);
	setAutoFillBackground(true);

	QHBoxLayout* layout = new QHBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	m_session = new SessionWidget(this);
	layout->addWidget(m_session);

	m_rightAreaLayout = new QVBoxLayout();
	m_rightAreaLayout->addStretch(3);

	m_scrambler = make_shared<Cube3x3RandomStateScramble>();
	m_scrambleWidget = new ScrambleWidget(this);
	m_scrambleWidget->setMaxMoveCount(m_scrambler->GetMaxMoveCount());
	connect(m_scrambleWidget, &ScrambleWidget::scrambleComplete, this, &TimerMode::scrambleComplete);
	m_rightAreaLayout->addWidget(m_scrambleWidget);
	m_scrambleStretch = m_rightAreaLayout->count();
	m_rightAreaLayout->addStretch(1);

	m_cube3x3Widget = new Cube3x3Widget();
	m_cube3x3Widget->hide();
	m_cube3x3Widget->setBackgroundColor(Theme::backgroundDark);
	m_rightAreaLayout->addWidget(m_cube3x3Widget, 32);

	m_timer = new TimerWidget(this);
	connect(m_timer, &TimerWidget::aboutToStart, this, &TimerMode::solveStarting);
	connect(m_timer, &TimerWidget::reset, this, &TimerMode::solveStopping);
	connect(m_timer, &TimerWidget::completed, this, &TimerMode::solveComplete);
	m_rightAreaLayout->addWidget(m_timer);

	m_rightAreaLayout->addStretch(3);
	layout->addLayout(m_rightAreaLayout, 1);
	setLayout(layout);

	updateFontSizes();

	m_scrambleThread = new ScrambleThread(this);
	connect(m_scrambleThread, &ScrambleThread::scrambleGenerated, this, &TimerMode::scrambleGenerated);
	m_scrambleThread->start();
	newScramble();
}


TimerMode::~TimerMode()
{
	m_scrambleThread->stop();
	m_scrambleThread->wait();
}


void TimerMode::newScramble()
{
	if (m_pendingScrambleValid)
	{
		// There is an extra scramble already generated, use it now
		m_scrambleValid = true;
		m_currentScramble = m_pendingScramble;
		m_pendingScrambleValid = false;
		m_scrambleWidget->setScramble(m_currentScramble);
		m_timer->enable();
	}
	else
	{
		// There are no valid scrambles, need to wait for one
		m_scrambleValid = false;
		m_scrambleWidget->invalidateScramble();
		m_timer->disable();
	}

	// Generate a new scramble
	m_scrambleThread->requestScramble(m_scrambler);
}


void TimerMode::updateFontSizes()
{
	int w = width() * 2 / 3;
	int h = height();
	int size = w;
	if (h < size)
		size = h;
#ifdef __APPLE__
	m_timer->setFontSize(size / 6);
	m_scrambleWidget->setFontSize(size / 19);
#else
	m_timer->setFontSize(size / 7);
	m_scrambleWidget->setFontSize(size / 22);
#endif
}


void TimerMode::resizeEvent(QResizeEvent*)
{
	updateFontSizes();
}


void TimerMode::buttonDown()
{
	if (!m_scrambleValid)
		return;
	m_timer->buttonDown();
}


void TimerMode::buttonUp()
{
	m_timer->buttonUp();
}


bool TimerMode::running() const
{
	return m_timer->running();
}


void TimerMode::solveStarting()
{
	// On timer start, hide everything but the timer
	m_scrambleWidget->hide();
	m_session->hide();
	m_rightAreaLayout->setStretch(m_scrambleStretch, 0);
	emit timerStarting();
}


void TimerMode::solveStopping()
{
	// When timer is stopped, show the rest of the interface
	m_scrambleWidget->show();
	m_session->show();
	m_rightAreaLayout->setStretch(m_scrambleStretch, 1);
	emit timerStopping();
}


void TimerMode::solveComplete()
{
	// Record solve in session
	Solve solve;
	solve.id = History::instance.idGenerator->GenerateId();
	solve.scramble = m_currentScramble;
	solve.created = time(NULL);
	solve.update.id = History::instance.idGenerator->GenerateId();
	solve.update.date = solve.created;
	solve.ok = true;
	solve.time = m_timer->value();
	solve.penalty = 0;
	solve.dirty = true;
	History::instance.RecordSolve(m_solveType, solve);
	m_session->updateHistory();

	// Generate new scramble after solve
	newScramble();
}


void TimerMode::scrambleGenerated()
{
	if (m_scrambleValid)
	{
		// There is already a scramble, save it for next time
		m_pendingScrambleValid = true;
		m_pendingScramble = m_scrambleThread->scramble();
		return;
	}

	// Was waiting on a scramble, set it up
	m_scrambleValid = true;
	m_currentScramble = m_scrambleThread->scramble();
	m_scrambleWidget->setScramble(m_currentScramble);
	m_timer->enable();

	// Generate an extra scramble in the background so that the
	// next solve will be ready to start immediately
	m_scrambleThread->requestScramble(m_scrambler);
}


void TimerMode::updateHistory()
{
	m_session->updateHistory();
}


void TimerMode::setBluetoothCube(const shared_ptr<BluetoothCube>& cube)
{
	m_bluetoothCube = cube;
	m_cube3x3Widget->setBluetoothCube(cube);
	m_scrambleWidget->setBluetoothCube(cube);
	m_timer->setBluetoothCube(cube);
	if (m_bluetoothCube)
	{
		m_cube3x3Widget->show();
		m_rightAreaLayout->setStretch(2, 0);
	}
	else
	{
		m_cube3x3Widget->hide();
		m_rightAreaLayout->setStretch(2, 1);
	}
}


void TimerMode::scrambleComplete()
{
	m_timer->readyForBluetoothSolve();
}
