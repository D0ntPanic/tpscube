#include <QtCore/QRandomGenerator>
#include "scramblewidget.h"
#include "theme.h"

#define MOVES_PER_ROW 8

using namespace std;


int QtRandomSource::Next(int range)
{
	return QRandomGenerator::global()->bounded(range);
}


RescrambleThread::RescrambleThread(QObject* owner): QThread(owner)
{
}


void RescrambleThread::run()
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

		CubeMoveSequence scramble = m_inScramble;
		Cube3x3 state = m_initialState;
		m_requestPending = false;

		m_mutex.unlock();

		CubeMoveSequence inverted = state.Solve(false);
		state = Cube3x3();
		state.Apply(inverted);
		state.Apply(scramble);
		CubeMoveSequence result = state.Solve().Inverted();

		m_mutex.lock();
		if (!m_requestPending)
		{
			m_outScramble = result;
			emit rescrambleGenerated();
		}
		m_mutex.unlock();
	}
}


void RescrambleThread::stop()
{
	m_running = false;
	m_cond.notify_all();
}


void RescrambleThread::requestRescramble(const Cube3x3& state, const CubeMoveSequence& scramble)
{
	QMutexLocker lock(&m_mutex);
	m_requestPending = true;
	m_inScramble = scramble;
	m_initialState = state;
	m_cond.notify_one();
}


CubeMoveSequence RescrambleThread::rescramble()
{
	QMutexLocker lock(&m_mutex);
	return m_outScramble;
}


ScrambleWidget::ScrambleWidget(QWidget* parent): QLabel(parent)
{
	setFont(QFont("Open Sans", 26, QFont::Light));
	setAlignment(Qt::AlignCenter | Qt::AlignVCenter);

	QPalette pal(palette());
	pal.setColor(QPalette::Text, Theme::blue);
	setPalette(pal);

	m_bluetoothUpdateTimer = new QTimer(this);
	m_bluetoothUpdateTimer->setSingleShot(false);
	m_bluetoothUpdateTimer->setInterval(100);
	connect(m_bluetoothUpdateTimer, &QTimer::timeout, this, &ScrambleWidget::updateBluetoothScramble);

	m_thread = new RescrambleThread(this);
	connect(m_thread, &RescrambleThread::rescrambleGenerated, this, &ScrambleWidget::rescrambleGenerated);
	m_thread->start();
}


ScrambleWidget::~ScrambleWidget()
{
	if (m_bluetoothCube && m_bluetoothCubeClient)
		m_bluetoothCube->RemoveClient(m_bluetoothCubeClient);
	m_thread->stop();
	m_thread->wait();
}


void ScrambleWidget::updateText()
{
	size_t rows = m_maxMoveCount / MOVES_PER_ROW;
	size_t neededRows = m_scramble.moves.size() / MOVES_PER_ROW;
	if (neededRows == 0)
		neededRows = 1;
	if (!m_reserveVerticalSpace)
		rows = neededRows;
	size_t cols = (m_scramble.moves.size() + (neededRows - 1)) / neededRows;

	QString text;
	if ((m_currentScrambleMove != -1) && (m_fixMoves.moves.size() != 0))
	{
		for (size_t i = 2; i < rows; i++)
			text += "<br/>\n";
		text += "Scramble incorrect, fix with:<br/>";
		for (size_t i = 0; i < m_fixMoves.moves.size(); i++)
		{
			if (i > 0)
				text += "&nbsp;&nbsp;";
			if (i == 0)
				text += "<b>";
			else
				text += "<font color='#808080'>";
			text += QString::fromStdString(CubeMoveSequence::MoveToString(
				m_fixMoves.moves[m_fixMoves.moves.size() - (i + 1)]));
			if (i == 0)
				text += "</b>";
			else
				text += "</font>";
		}
	}
	else
	{
		for (size_t i = neededRows; i < rows; i++)
			text += "<br/>\n";

		size_t moveIdx = 0;
		for (size_t i = 0; i < neededRows; i++)
		{
			if (i > 0)
				text += "<br/>\n";
			for (size_t j = 0; j < cols; j++)
			{
				if (moveIdx >= m_scramble.moves.size())
					break;
				if (j > 0)
					text += "&nbsp;&nbsp;";
				if (m_currentScrambleMove == (int)moveIdx)
					text += "<b>";
				else if (m_currentScrambleMove != -1)
					text += "<font color='#808080'>";
				text += QString::fromStdString(CubeMoveSequence::MoveToString(m_scramble.moves[moveIdx]));
				if (m_currentScrambleMove == (int)moveIdx)
					text += "</b>";
				else if (m_currentScrambleMove != -1)
					text += "</font>";
				moveIdx++;
			}
		}
	}

	setTextFormat(Qt::RichText);
	setText(text);
}


void ScrambleWidget::setScramble(const CubeMoveSequence& scramble)
{
	m_scramble = scramble;
	m_originalScramble = scramble;
	updateText();

	if (m_bluetoothCubeClient)
		m_bluetoothCubeClient->GetLatestMoves();

	if (m_bluetoothCube && m_bluetoothCube->GetCubeState().IsSolved())
	{
		m_currentScrambleMove = 0;
		m_currentScrambleSubMove = 0;
		m_fixMoves.moves.clear();
	}
	else
	{
		m_currentScrambleMove = -1;
	}
}


void ScrambleWidget::invalidateScramble()
{
	m_scramble.moves.clear();
	m_originalScramble.moves.clear();
	updateText();
}


void ScrambleWidget::setFontSize(int size)
{
	setFont(QFont("Open Sans", size, QFont::Light));
}


void ScrambleWidget::setMaxMoveCount(size_t count)
{
	m_maxMoveCount = count;
	updateText();
}


void ScrambleWidget::setReserveVerticalSpace(bool enable)
{
	m_reserveVerticalSpace = enable;
	updateText();
}


void ScrambleWidget::setBluetoothCube(const shared_ptr<BluetoothCube>& cube)
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

		if (m_bluetoothCube->GetCubeState().IsSolved())
		{
			m_currentScrambleMove = 0;
			m_currentScrambleSubMove = 0;
			m_fixMoves.moves.clear();
		}
		else
		{
			m_currentScrambleMove = -1;
			if (m_originalScramble.moves.size() != 0)
				m_thread->requestRescramble(m_bluetoothCube->GetCubeState(), m_originalScramble);
		}
		updateText();

		m_bluetoothUpdateTimer->start();
	}
	else
	{
		m_scramble = m_originalScramble;
		m_currentScrambleMove = -1;
		updateText();

		m_bluetoothUpdateTimer->stop();
	}
}


void ScrambleWidget::updateScrambleStateForHalfMove(CubeMove move, CubeMove forward, CubeMove back)
{
	if (move == forward)
	{
		m_currentScrambleSubMove++;
		if (m_currentScrambleSubMove == 2)
		{
			m_currentScrambleMove++;
			m_currentScrambleSubMove = 0;
		}
	}
	else if (move == back)
	{
		m_currentScrambleSubMove--;
		if (m_currentScrambleSubMove == -2)
		{
			m_currentScrambleMove++;
			m_currentScrambleSubMove = 0;
		}
	}
	else
	{
		m_fixMoves.moves.push_back(CubeMoveSequence::InvertedMove(move));
	}
}


void ScrambleWidget::updateScrambleStateForMove(CubeMove currentMove, CubeMove scrambleMove)
{
	switch (scrambleMove)
	{
	case MOVE_U:
	case MOVE_Up:
	case MOVE_F:
	case MOVE_Fp:
	case MOVE_R:
	case MOVE_Rp:
	case MOVE_B:
	case MOVE_Bp:
	case MOVE_L:
	case MOVE_Lp:
	case MOVE_D:
	case MOVE_Dp:
		if (scrambleMove != currentMove)
			m_fixMoves.moves.push_back(CubeMoveSequence::InvertedMove(currentMove));
		else
			m_currentScrambleMove++;
		break;
	case MOVE_U2:
		updateScrambleStateForHalfMove(currentMove, MOVE_U, MOVE_Up);
		break;
	case MOVE_F2:
		updateScrambleStateForHalfMove(currentMove, MOVE_F, MOVE_Fp);
		break;
	case MOVE_R2:
		updateScrambleStateForHalfMove(currentMove, MOVE_R, MOVE_Rp);
		break;
	case MOVE_B2:
		updateScrambleStateForHalfMove(currentMove, MOVE_B, MOVE_Bp);
		break;
	case MOVE_L2:
		updateScrambleStateForHalfMove(currentMove, MOVE_L, MOVE_Lp);
		break;
	case MOVE_D2:
		updateScrambleStateForHalfMove(currentMove, MOVE_D, MOVE_Dp);
		break;
	default:
		m_currentScrambleMove = -1;
		break;
	}
}


void ScrambleWidget::updateBluetoothScramble()
{
	if (m_bluetoothCubeClient)
	{
		TimedCubeMoveSequence moves = m_bluetoothCubeClient->GetLatestMoves();
		if (m_currentScrambleMove == -1)
		{
			if (m_bluetoothCube->GetCubeState().IsSolved())
			{
				m_scramble = m_originalScramble;
				m_currentScrambleMove = 0;
				m_currentScrambleSubMove = 0;
				m_fixMoves.moves.clear();
				updateText();
			}
			else if ((m_originalScramble.moves.size() != 0) && (moves.moves.size() != 0))
			{
				m_thread->requestRescramble(m_bluetoothCube->GetCubeState(), m_originalScramble);
			}
			return;
		}
		else if (m_currentScrambleMove >= (int)m_scramble.moves.size())
		{
			return;
		}

		for (auto& i : moves.moves)
		{
			if ((m_currentScrambleMove == -1) || (m_currentScrambleMove >= (int)m_scramble.moves.size()))
				break;

			if (m_fixMoves.moves.size() != 0)
			{
				if (i.move == m_fixMoves.moves[m_fixMoves.moves.size() - 1])
					m_fixMoves.moves.erase(m_fixMoves.moves.end() - 1);
				else
					m_fixMoves.moves.push_back(CubeMoveSequence::InvertedMove(i.move));
				continue;
			}

			CubeMove scrambleMove = m_scramble.moves[m_currentScrambleMove];
			CubeMove currentMove = i.move;
			updateScrambleStateForMove(currentMove, scrambleMove);
		}

		if (m_currentScrambleMove >= (int)m_scramble.moves.size())
			emit scrambleComplete();
		else if (m_fixMoves.moves.size() > 4)
		{
			m_currentScrambleMove = -1;
			m_thread->requestRescramble(m_bluetoothCube->GetCubeState(), m_originalScramble);
		}

		updateText();
	}
}


void ScrambleWidget::rescrambleGenerated()
{
	if (m_originalScramble.moves.size() == 0)
		return;

	m_scramble = m_thread->rescramble();
	m_currentScrambleMove = 0;
	m_currentScrambleSubMove = 0;
	m_fixMoves.moves.clear();
	updateText();
}
