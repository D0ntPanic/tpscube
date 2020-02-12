#include <QtWidgets/QVBoxLayout>
#include <QtGui/QKeyEvent>
#include "mainwindow.h"
#include "theme.h"
#include "topbar.h"
#include "timermode.h"
#include "historymode.h"

using namespace std;


MainWindow* MainWindow::m_instance = nullptr;


MainWindow::MainWindow()
{
	m_instance = this;

	resize(QSize(960, 600));
	setWindowTitle("TPS Cube");

	QWidget* container = new QWidget(this);
	QVBoxLayout* containerLayout = new QVBoxLayout();
	containerLayout->setContentsMargins(0, 0, 0, 0);
	containerLayout->setSpacing(0);

	m_topBar = new TopBar(container);
	containerLayout->addWidget(m_topBar);
	connect(m_topBar, &TopBar::showTimer, this, &MainWindow::showTimer);
	connect(m_topBar, &TopBar::showHistory, this, &MainWindow::showHistory);
	connect(m_topBar, &TopBar::showGraphs, this, &MainWindow::showGraphs);
	connect(m_topBar, &TopBar::showAlgorithms, this, &MainWindow::showAlgorithms);

	m_stackedWidget = new QStackedWidget();
	containerLayout->addWidget(m_stackedWidget, 1);

	m_timerMode = new TimerMode(this);
	m_timerModeIndex = m_stackedWidget->addWidget(m_timerMode);
	connect(m_timerMode, &TimerMode::timerStarting, this, &MainWindow::timerStarting);
	connect(m_timerMode, &TimerMode::timerStopping, this, &MainWindow::timerStopping);

	m_historyMode = new HistoryMode(this);
	m_historyModeIndex = m_stackedWidget->addWidget(m_historyMode);

	m_stackedWidget->setCurrentIndex(m_timerModeIndex);

	container->setLayout(containerLayout);
	setCentralWidget(container);
}


MainWindow::~MainWindow()
{
	History::instance.CloseDatabase();
}


MainWindow* MainWindow::instance()
{
	return m_instance;
}


void MainWindow::keyPressEvent(QKeyEvent* event)
{
	if ((m_stackedWidget->currentIndex() == m_timerModeIndex) && !event->isAutoRepeat())
	{
		if (event->key() == Qt::Key_Space)
		{
			m_timerMode->buttonDown();
		}
		else if (m_timerMode->running())
		{
			// To stop the timer, hit anything
			m_timerMode->buttonDown();
			m_timerMode->buttonUp();
		}
	}
	QMainWindow::keyPressEvent(event);
}


void MainWindow::keyReleaseEvent(QKeyEvent* event)
{
	if ((m_stackedWidget->currentIndex() == m_timerModeIndex) && !event->isAutoRepeat())
	{
		if (event->key() == Qt::Key_Space)
			m_timerMode->buttonUp();
	}
	QMainWindow::keyReleaseEvent(event);
}


void MainWindow::timerStarting()
{
	m_topBar->hide();
}


void MainWindow::timerStopping()
{
	m_topBar->show();
}


void MainWindow::showTimer()
{
	m_timerMode->updateHistory();
	m_stackedWidget->setCurrentIndex(m_timerModeIndex);
}


void MainWindow::showHistory()
{
	m_historyMode->updateHistory();
	m_stackedWidget->setCurrentIndex(m_historyModeIndex);
}


void MainWindow::showGraphs()
{
}


void MainWindow::showAlgorithms()
{
}
