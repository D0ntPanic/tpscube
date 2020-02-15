#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QPushButton>
#include <QtWidgets/QAction>
#include <QtGui/QClipboard>
#include <QtGui/QGuiApplication>
#include "averagedialog.h"
#include "mainwindow.h"

using namespace std;


AverageDialog::AverageDialog(const vector<Solve>& solves): QDialog(MainWindow::instance())
{
	setModal(false);
	setWindowTitle(QString::asprintf("Average of %d", (int)solves.size()));

	QVBoxLayout* layout = new QVBoxLayout();
	m_average = new AverageWidget(solves, true);
	layout->addWidget(m_average);
	layout->addSpacing(8);

	QHBoxLayout* buttonLayout = new QHBoxLayout();
	buttonLayout->addStretch(1);

	QPushButton* copyButton = new QPushButton("Copy to Clipboard");
	copyButton->setDefault(false);
	copyButton->setAutoDefault(false);
	connect(copyButton, &QPushButton::clicked, this, &AverageDialog::copy);
	QAction* copyAction = new QAction("Copy", this);
	copyAction->setShortcut(QKeySequence::Copy);
	copyAction->setShortcutContext(Qt::WidgetWithChildrenShortcut);
	addAction(copyAction);
	connect(copyAction, &QAction::triggered, this, &AverageDialog::copy);
	buttonLayout->addWidget(copyButton);

	QPushButton* closeButton = new QPushButton("Close");
	closeButton->setDefault(true);
	closeButton->setAutoDefault(true);
	connect(closeButton, &QPushButton::clicked, this, &AverageDialog::accept);
	buttonLayout->addWidget(closeButton);
	layout->addLayout(buttonLayout);

	setLayout(layout);
}


void AverageDialog::copy()
{
	QString text = m_average->averageDetailsText();
	QGuiApplication::clipboard()->setText(text);
}
