#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QAction>
#include <QtGui/QKeyEvent>
#include <QtGui/QGuiApplication>
#include <QtGui/QClipboard>
#include "mainwindow.h"
#include "theme.h"
#include "topbar.h"
#include "timermode.h"
#include "historymode.h"
#include "graphmode.h"
#include "settingsmode.h"
#include "bluetoothdialog.h"
#include "solvedialog.h"

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
	connect(m_topBar, &TopBar::showSettings, this, &MainWindow::showSettings);
	connect(m_topBar, &TopBar::connectToBluetoothCube, this, &MainWindow::connectToBluetoothCube);
	connect(m_topBar, &TopBar::disconnectFromBluetoothCube, this, &MainWindow::disconnectFromBluetoothCube);

	m_stackedWidget = new QStackedWidget();
	containerLayout->addWidget(m_stackedWidget, 1);

	m_timerMode = new TimerMode(this);
	m_timerModeIndex = m_stackedWidget->addWidget(m_timerMode);
	connect(m_timerMode, &TimerMode::timerStarting, this, &MainWindow::timerStarting);
	connect(m_timerMode, &TimerMode::timerStopping, this, &MainWindow::timerStopping);

	m_historyMode = new HistoryMode(this);
	m_historyModeIndex = m_stackedWidget->addWidget(m_historyMode);

	m_graphMode = new GraphMode(this);
	m_graphModeIndex = m_stackedWidget->addWidget(m_graphMode);

	m_settingsMode = new SettingsMode(this);
	m_settingsModeIndex = m_stackedWidget->addWidget(m_settingsMode);

	m_stackedWidget->setCurrentIndex(m_timerModeIndex);

	QAction* pasteAction = new QAction(this);
	pasteAction->setShortcut(QKeySequence::Paste);
	connect(pasteAction, &QAction::triggered, this, &MainWindow::paste);
	addAction(pasteAction);

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
	m_graphMode->updateGraph();
	m_stackedWidget->setCurrentIndex(m_graphModeIndex);
}


void MainWindow::showAlgorithms()
{
}


void MainWindow::showSettings()
{
	m_stackedWidget->setCurrentIndex(m_settingsModeIndex);
}


void MainWindow::connectToBluetoothCube()
{
	BluetoothDialog dlg;
	if (dlg.exec() == QDialog::Accepted)
	{
		m_timerMode->setBluetoothCube(dlg.cube());
		m_topBar->setBluetoothCube(dlg.cube());
	}
}


void MainWindow::disconnectFromBluetoothCube()
{
	m_timerMode->setBluetoothCube(shared_ptr<BluetoothCube>());
	m_topBar->setBluetoothCube(shared_ptr<BluetoothCube>());
}


void MainWindow::paste()
{
	QString text = QGuiApplication::clipboard()->text();
	QStringList lines = text.split('\n');
	if ((lines.size() == 1) || ((lines.size() >= 2) && (lines[1].startsWith("Solve:"))))
	{
		Solve solve;
		if (SolveWidget::solveFromText(text, solve))
		{
			SolveDialog* dlg = new SolveDialog(solve);
			dlg->show();
		}
	}
}
