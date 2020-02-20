#include <QtWidgets/QLabel>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QHBoxLayout>
#include <QtWidgets/QPushButton>
#include "bluetoothresetwidget.h"

using namespace std;


BluetoothResetWidget::BluetoothResetWidget()
{
	QVBoxLayout* layout = new QVBoxLayout();
	layout->setContentsMargins(0, 0, 0, 0);

	m_heading = new Heading("Synchronize");
	layout->addWidget(m_heading);

	m_cubeWidget = new Cube3x3Widget();
	layout->addWidget(m_cubeWidget, 1);
	layout->addSpacing(8);

	QLabel* syncLabel = new QLabel("Solve your cube and then click\nFinish to complete synchronization.");
	syncLabel->setAlignment(Qt::AlignVCenter | Qt::AlignCenter);
	layout->addWidget(syncLabel);
	layout->addSpacing(8);

	QHBoxLayout* buttonLayout = new QHBoxLayout();
	buttonLayout->addStretch(1);
	QPushButton* finishButton = new QPushButton("Finish");
	connect(finishButton, &QPushButton::clicked, this, &BluetoothResetWidget::donePushed);
	buttonLayout->addWidget(finishButton);
	QPushButton* cancelButton = new QPushButton("Cancel");
	connect(cancelButton, &QPushButton::clicked, this, &BluetoothResetWidget::cancelPushed);
	buttonLayout->addWidget(cancelButton);

	layout->addLayout(buttonLayout);
	setLayout(layout);
}


void BluetoothResetWidget::setCube(const shared_ptr<BluetoothCube>& cube)
{
	m_cube = cube;
	m_heading->setName("Synchronize " + QString::fromStdString(m_cube->GetDevice()->GetName()));
}


void BluetoothResetWidget::donePushed()
{
	m_cube->ResetToSolved();
	emit done();
}


void BluetoothResetWidget::cancelPushed()
{
	emit cancel();
}
