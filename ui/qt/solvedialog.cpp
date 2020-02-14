#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QAction>
#include <QtGui/QClipboard>
#include <QtGui/QGuiApplication>
#include "solvedialog.h"
#include "mainwindow.h"

using namespace std;


SolveDialog::SolveDialog(const Solve& solve): QDialog(MainWindow::instance())
{
	setModal(false);
	setWindowTitle("Solve");

	QVBoxLayout* layout = new QVBoxLayout();
	m_solve = new SolveWidget(this, solve);
	layout->addWidget(m_solve);
	layout->addSpacing(8);

	QHBoxLayout* buttonLayout = new QHBoxLayout();
	buttonLayout->addStretch(1);

	QPushButton* copyButton = new QPushButton("Copy to Clipboard");
	copyButton->setDefault(false);
	copyButton->setAutoDefault(false);
	connect(copyButton, &QPushButton::clicked, this, &SolveDialog::copy);
	QAction* copyAction = new QAction("Copy", this);
	copyAction->setShortcut(QKeySequence::Copy);
	copyAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
	addAction(copyAction);
	connect(copyAction, &QAction::triggered, this, &SolveDialog::copy);
	buttonLayout->addWidget(copyButton);

	QPushButton* closeButton = new QPushButton("Close");
	closeButton->setDefault(true);
	closeButton->setAutoDefault(true);
	connect(closeButton, &QPushButton::clicked, this, &SolveDialog::accept);
	buttonLayout->addWidget(closeButton);
	layout->addLayout(buttonLayout);

	setLayout(layout);
}


void SolveDialog::copy()
{
	QString text = m_solve->solveDetailsText();
	QGuiApplication::clipboard()->setText(text);
}
