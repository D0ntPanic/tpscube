#include <QtWidgets/QApplication>
#include <QtWidgets/QVBoxLayout>
#include <QtWidgets/QDesktopWidget>
#include "tooltip.h"


Tooltip* Tooltip::m_activeTooltip = nullptr;


Tooltip::Tooltip(QWidget* contents)
{
	setContentsMargins(0, 0, 0, 0);
	setFrameStyle(QFrame::Box);

	QVBoxLayout* layout = new QVBoxLayout();
	layout->setContentsMargins(8, 8, 8, 8);
	layout->addWidget(contents);
	setLayout(layout);

	QApplication::instance()->installEventFilter(this);
}


Tooltip::~Tooltip()
{
	QApplication::instance()->removeEventFilter(this);
}


bool Tooltip::eventFilter(QObject*, QEvent* event)
{
	switch (event->type())
	{
	case QEvent::FocusIn:
	case QEvent::FocusOut:
	case QEvent::WindowActivate:
	case QEvent::WindowDeactivate:
	case QEvent::Leave:
	case QEvent::MouseButtonPress:
	case QEvent::MouseButtonRelease:
	case QEvent::MouseButtonDblClick:
	case QEvent::MouseMove:
		if (m_activeTooltip == this)
			m_activeTooltip = nullptr;
		hide();
		deleteLater();
		return false;
	default:
		return false;
	}
}


void Tooltip::show(QWidget* srcWidget)
{
	if (!srcWidget->isActiveWindow())
		return;

	if (m_activeTooltip)
	{
		m_activeTooltip->hide();
		m_activeTooltip->deleteLater();
	}

	m_activeTooltip = this;

	setWindowFlags(Qt::ToolTip | Qt::FramelessWindowHint | Qt::WindowDoesNotAcceptFocus);

	// Compute screen position of source area, and adjust to include cursor if the
	// cursor overlaps with it.
	QPoint cursorPos = QCursor::pos();
	QRect cursorRect = QRect(cursorPos.x(), cursorPos.y(), 16, 16);
	QSize targetSize = sizeHint();

	// Calculate screen position of tooltip so that it doesn't overlap the source area and
	// doesn't go off screen.
	QRect screen = QApplication::desktop()->availableGeometry(srcWidget);
	QPoint dest;
	if ((cursorRect.right() + targetSize.width()) > screen.right())
	{
		if ((cursorRect.left() - targetSize.width()) < screen.left())
			dest.setX(screen.right() - targetSize.width());
		else
			dest.setX(cursorRect.left() - targetSize.width());
	}
	else
	{
		dest.setX(cursorRect.right());
	}
	if ((cursorRect.bottom() + targetSize.height()) > screen.bottom())
	{
		if ((cursorRect.top() - targetSize.height()) < screen.top())
			dest.setY(screen.bottom() - targetSize.height());
		else
			dest.setY(cursorRect.top() - targetSize.height());
	}
	else
	{
		dest.setY(cursorRect.bottom());
	}

	setGeometry(dest.x(), dest.y(), targetSize.width(), targetSize.height());
	QFrame::show();
}
