#include <QtWidgets/QMainWindow>
#include <QtWidgets/QStackedWidget>

class TopBar;
class TimerMode;
class HistoryMode;

class MainWindow: public QMainWindow
{
	Q_OBJECT

	TopBar* m_topBar;
	QStackedWidget* m_stackedWidget;
	TimerMode* m_timerMode;
	int m_timerModeIndex;
	HistoryMode* m_historyMode;
	int m_historyModeIndex;

	static MainWindow* m_instance;

private slots:
	void timerStarting();
	void timerStopping();
	void showTimer();
	void showHistory();
	void showGraphs();
	void showAlgorithms();
	void connectToBluetoothCube();
	void disconnectFromBluetoothCube();
	void paste();

protected:
	virtual void keyPressEvent(QKeyEvent* event) override;
	virtual void keyReleaseEvent(QKeyEvent* event) override;

public:
	MainWindow();
	~MainWindow();

	static MainWindow* instance();
};
