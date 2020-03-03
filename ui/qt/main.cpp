#include <QtWidgets/QApplication>
#include <QtWidgets/QStyleFactory>
#include <QtWidgets/QMessageBox>
#include <QtWidgets/QProgressDialog>
#include <QtGui/QFontDatabase>
#include <QtGui/QSurfaceFormat>
#include <QtCore/QCoreApplication>
#include <QtCore/QStandardPaths>
#include <QtCore/QDir>
#include <QtCore/QUuid>
#include <stdio.h>
#include <vector>
#include "mainwindow.h"
#include "theme.h"
#include "cube3x3.h"
#include "history.h"
#include "bluetoothcube.h"

using namespace std;


class QtIdGenerator: public IdGenerator
{
public:
	virtual string GenerateId() override
	{
		return QUuid::createUuid().toString().toStdString();
	}
};


#define EXPECT(test, msg, dbg) \
	{ \
		fprintf(stderr, "%s... ", string(msg).c_str()); \
		fflush(stderr); \
		bool result = (test); \
		if (result) \
		{ \
			fprintf(stderr, "OK\n"); \
		} \
		else \
		{ \
			fprintf(stderr, "FAIL\n"); \
			dbg; \
			return 1; \
		} \
	}

#define EXPECT_REPEAT(test, count, msg, dbg) \
	{ \
		fprintf(stderr, "%s... ", string(msg).c_str()); \
		fflush(stderr); \
		for (int _i = 0; _i < (count); _i++) \
		{ \
			bool result = (test); \
			if (!result) \
			{ \
				fprintf(stderr, "FAIL\n"); \
				dbg; \
				return 1; \
			} \
		} \
		fprintf(stderr, "OK\n"); \
	}


template<class T>
int Cube3x3BasicMoveTest(const std::string& name)
{
	T cube;
	EXPECT(cube.IsSolved(), name + ": Initial state is solved", Cube3x3Faces(cube).PrintDebugState());
	cube.Move(MOVE_U);
	EXPECT(!cube.IsSolved(), name + ": State after U is not solved", Cube3x3Faces(cube).PrintDebugState());
	cube.Move(MOVE_Up);
	EXPECT(cube.IsSolved(), name + ": State after U U' is solved", Cube3x3Faces(cube).PrintDebugState());
	vector<CubeMove> yPerm {MOVE_F, MOVE_R, MOVE_Up, MOVE_Rp, MOVE_Up, MOVE_R, MOVE_U, MOVE_Rp,
		MOVE_Fp, MOVE_R, MOVE_U, MOVE_Rp, MOVE_Up, MOVE_Rp, MOVE_F, MOVE_R, MOVE_Fp};
	for (size_t i = 0; i < 2; i++)
		for (auto j : yPerm)
			cube.Move(j);
	EXPECT(cube.IsSolved(), name + ": State after 2x Y perm is solved", Cube3x3Faces(cube).PrintDebugState());
	vector<CubeMove> scramble { MOVE_D2, MOVE_R2, MOVE_B2, MOVE_L, MOVE_U2, MOVE_R, MOVE_D2,
		MOVE_Lp, MOVE_B2, MOVE_R2, MOVE_D2, MOVE_Fp, MOVE_Lp, MOVE_D, MOVE_L, MOVE_R2, MOVE_D,
		MOVE_B, MOVE_U, MOVE_L2 };
	vector<CubeMove> invScramble { MOVE_L2, MOVE_Up, MOVE_Bp, MOVE_Dp, MOVE_R2, MOVE_Lp, MOVE_Dp,
		MOVE_L, MOVE_F, MOVE_D2, MOVE_R2, MOVE_B2, MOVE_L, MOVE_D2, MOVE_Rp, MOVE_U2, MOVE_Lp,
		MOVE_B2, MOVE_R2, MOVE_D2 };
	for (auto i : scramble)
		cube.Move(i);
	for (auto i : invScramble)
		cube.Move(i);
	EXPECT(cube.IsSolved(), name + ": State after fixed scramble and inverse is solved", Cube3x3Faces(cube).PrintDebugState());
	return 0;
}


int Cube3x3MatchTest()
{
	for (uint8_t move = MOVE_U; move <= MOVE_D2; move += 3)
	{
		Cube3x3 pieces;
		Cube3x3Faces faces;
		pieces.Move((CubeMove)move);
		faces.Move((CubeMove)move);
		Cube3x3Faces piecesConverted(pieces);
		Cube3x3 facesConverted(faces);
		EXPECT((faces == piecesConverted) && (facesConverted == pieces), "3x3 format match: Move " +
			CubeMoveSequence::MoveToString((CubeMove)move), {
				fprintf(stderr, "Face color format:\n");
				faces.PrintDebugState();
				fprintf(stderr, "Piece format:\n");
				piecesConverted.PrintDebugState();
				fprintf(stderr, "Faces converted to piece format:\n");
				Cube3x3Faces(facesConverted).PrintDebugState();
			});
	}

	Cube3x3 pieces;
	Cube3x3Faces faces;
	SimpleSeededRandomSource rng;
	for (size_t i = 0; i < 100; i++)
	{
		CubeMove move = (CubeMove)rng.Next(MOVE_D2 + 1);
		pieces.Move(move);
		faces.Move(move);
	}
	Cube3x3Faces piecesConverted(pieces);
	Cube3x3 facesConverted(faces);
	EXPECT((faces == piecesConverted) && (facesConverted == pieces), "3x3 format match: 100 random moves", {
		fprintf(stderr, "Face color format:\n");
		faces.PrintDebugState();
		fprintf(stderr, "Piece format:\n");
		piecesConverted.PrintDebugState();
		fprintf(stderr, "Faces converted to pieces format:\n");
		Cube3x3Faces(facesConverted).PrintDebugState();
	});
	return 0;
}


int Cube3x3IndexTest()
{
	Cube3x3 cube;
	SimpleSeededRandomSource rng;
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetCornerOrientationIndex() < CORNER_ORIENTATION_INDEX_COUNT),
		10000, "3x3 index: Corner orientation index bounds", Cube3x3Faces(cube).PrintDebugState());
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetEdgeOrientationIndex() < EDGE_ORIENTATION_INDEX_COUNT),
		10000, "3x3 index: Edge orientation index bounds", Cube3x3Faces(cube).PrintDebugState());
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetEquatorialEdgeSliceIndex() < EDGE_ORIENTATION_INDEX_COUNT),
		10000, "3x3 index: Equatorial edge slice index bounds", Cube3x3Faces(cube).PrintDebugState());
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetCornerPermutationIndex() < CORNER_PERMUTATION_INDEX_COUNT),
		10000, "3x3 index: Corner permutation index bounds", Cube3x3Faces(cube).PrintDebugState());
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetPhase2EdgePermutationIndex() < PHASE_2_EDGE_PERMUTATION_INDEX_COUNT),
		10000, "3x3 index: Phase 2 edge permutation index bounds", Cube3x3Faces(cube).PrintDebugState());
	EXPECT_REPEAT((cube.Move(CubeMoveSequence::RandomMove(rng)),
		cube.GetPhase2EquatorialEdgePermutationIndex() < PHASE_2_EQUATORIAL_EDGE_PERMUTATION_INDEX_COUNT),
		10000, "3x3 index: Phase 2 equatorial edge permutation index bounds", Cube3x3Faces(cube).PrintDebugState());
	return 0;
}


int Cube3x3SolveTest()
{
	SimpleSeededRandomSource rng;
	for (size_t i = 0; i < 10; i++)
	{
		Cube3x3 cube;
		cube.GenerateRandomState(rng);

		std::chrono::time_point<std::chrono::steady_clock> start = std::chrono::steady_clock::now();
		CubeMoveSequence solution = cube.Solve();
		std::chrono::time_point<std::chrono::steady_clock> end = std::chrono::steady_clock::now();
		int ms = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
		fprintf(stderr, "3x3 solve: %d ms for solution in %d moves (%s)\n", ms, (int)solution.moves.size(), solution.ToString().c_str());

		Cube3x3 initial = cube;
		for (auto j : solution.moves)
			cube.Move(j);
		if (!cube.IsSolved())
		{
			fprintf(stderr, "NOT SOLVED\n");
			Cube3x3Faces(initial).PrintDebugState();
			return 1;
		}
	}
	return 0;
}


int Cube3x3IntermediateSolveTest()
{
	SimpleSeededRandomSource rng;
	Cube3x3 cube;
	cube.GenerateRandomState(rng);
	CubeMoveSequence moves = cube.Solve();

	for (auto i : moves.moves)
	{
		std::chrono::time_point<std::chrono::steady_clock> start = std::chrono::steady_clock::now();
		cube.Move(i);
		CubeMoveSequence solution = cube.Solve();
		std::chrono::time_point<std::chrono::steady_clock> end = std::chrono::steady_clock::now();
		int ms = std::chrono::duration_cast<std::chrono::milliseconds>(end - start).count();
		fprintf(stderr, "3x3 solve: %d ms for solution in %d moves (%s)\n", ms, (int)solution.moves.size(), solution.ToString().c_str());

		Cube3x3 initial = cube;
		Cube3x3 solved = cube;
		for (auto j : solution.moves)
			solved.Move(j);
		if (!solved.IsSolved())
		{
			fprintf(stderr, "NOT SOLVED\n");
			Cube3x3Faces(initial).PrintDebugState();
			return 1;
		}
	}
	return 0;
}


int RunTest()
{
	if (Cube3x3BasicMoveTest<Cube3x3Faces>("3x3"))
		return 1;
	if (Cube3x3MatchTest())
		return 1;
	if (Cube3x3BasicMoveTest<Cube3x3>("3x3 pieces"))
		return 1;
	if (Cube3x3IndexTest())
		return 1;
	if (Cube3x3SolveTest())
		return 1;
	if (Cube3x3IntermediateSolveTest())
		return 1;
	return 0;
}


int main(int argc, char* argv[])
{
	if ((argc > 1) && !strcmp(argv[1], "--test"))
		return RunTest();

	QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
	QCoreApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

	QSurfaceFormat fmt;
	fmt.setDepthBufferSize(24);
	fmt.setSamples(2);
	QSurfaceFormat::setDefaultFormat(fmt);

	QApplication app(argc, argv);

	// Set up the theme
	QPalette palette;
	palette.setColor(QPalette::Window, Theme::backgroundWindow);
	palette.setColor(QPalette::WindowText, Theme::content);
	palette.setColor(QPalette::Base, Theme::backgroundDark);
	palette.setColor(QPalette::AlternateBase, Theme::background);
	palette.setColor(QPalette::ToolTipBase, Theme::backgroundHighlight);
	palette.setColor(QPalette::ToolTipText, Theme::content);
	palette.setColor(QPalette::Text, Theme::content);
	palette.setColor(QPalette::Button, Theme::backgroundHighlight);
	palette.setColor(QPalette::ButtonText, Theme::content);
	palette.setColor(QPalette::BrightText, Theme::yellow);
	palette.setColor(QPalette::Link, Theme::blue);
	palette.setColor(QPalette::Highlight, Theme::blue);
	palette.setColor(QPalette::HighlightedText, Theme::backgroundDark);
	palette.setColor(QPalette::Light, Theme::light);
	QApplication::setPalette(palette);
	QApplication::setStyle(QStyleFactory::create("Fusion"));
	app.setStyleSheet(QString("QMenu { background-color: %1; } "
		"QMenu::separator { background-color: %2; } "
		"QMenu::item:selected { background-color: %3; } "
		"QMenu::item:disabled { color: %4; } "
		"QPushButton:disabled { color: %5; } "
		"QComboBox:editable { background-color: %6; } ").
		arg(Theme::backgroundHighlight.name(QColor::HexRgb)).
		arg(Theme::backgroundWindow.name(QColor::HexRgb)).
		arg(palette.color(QPalette::Highlight).name(QColor::HexRgb)).
		arg(Theme::disabled.name(QColor::HexRgb)).
		arg(Theme::disabled.name(QColor::HexRgb)).
		arg(Theme::backgroundWindow.name(QColor::HexRgb)));

	QFontDatabase::addApplicationFont(":/fonts/OpenSans-Regular.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-Italic.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-Bold.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-BoldItalic.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-ExtraBold.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-ExtraBoldItalic.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-Light.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-LightItalic.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-Semibold.ttf");
	QFontDatabase::addApplicationFont(":/fonts/OpenSans-SemiboldItalic.ttf");
#ifdef __APPLE__
	QApplication::setFont(QFont("Open Sans", 15));
#else
	QApplication::setFont(QFont("Open Sans", 13));
#endif

	QApplication::setApplicationName("tpscube");
	QApplication::setApplicationDisplayName("TPS Cube");

	History::instance.idGenerator = new QtIdGenerator();
	BluetoothCubeType::Init();

	QString dataPath = QStandardPaths::writableLocation(QStandardPaths::AppDataLocation);
	bool aborted = false;
	if (QDir().mkpath(dataPath))
	{
		QProgressDialog progress("Loading solve history...", "Cancel", 0, 1);
		progress.setWindowModality(Qt::ApplicationModal);
		leveldb::Status status = History::instance.OpenDatabase(QDir(dataPath).filePath("tpscube.solvedata").toStdString(),
			[&](size_t currentValue, size_t maxValue) {
				progress.setMaximum((int)maxValue);
				progress.setValue((int)currentValue);
				return progress.wasCanceled();
			});
		aborted = progress.wasCanceled();

		if (!status.ok())
		{
			if (History::instance.IsDatabaseOpen())
			{
				QMessageBox::critical(nullptr, "Error", QString::fromStdString(
					"One or more errors while opening solve history database: " + status.ToString() +
					"\nSolve history may not be complete."));
			}
			else
			{
				QMessageBox::critical(nullptr, "Error", QString::fromStdString(
					"Error while opening solve history database: " + status.ToString() +
					"\nSolve history will not be saved."));
			}
		}
	}
	else
	{
		QMessageBox::critical(nullptr, "Error", "Local data storage location " + dataPath + " could not be written to. "
			"Solve history will not be saved.");
	}

	if (aborted)
		return 0;

	MainWindow window;
	window.show();
	app.exec();
	return 0;
}
