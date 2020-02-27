#include <QtGui/QPainter>
#include <algorithm>
#include "graphwidget.h"
#include "theme.h"

using namespace std;


GraphWidget::GraphWidget()
{
}


void GraphWidget::paintEvent(QPaintEvent*)
{
	QPainter p(this);

	if (m_plots.size() < 2)
	{
		QFont italicFont(fontOfRelativeSize(1.0f, QFont::Normal, true));
		QFontMetrics metrics(italicFont);
		p.setFont(italicFont);
		p.drawText(16, 16 + metrics.ascent(), m_message);
		return;
	}

	QVector<QLineF> lines;
	float dataX = 16.0f;
	float dataY = 16.0f;
	float dataW = (float)width() - 32.0f;
	float dataH = (float)height() - 32.0f;
	for (size_t i = 1; i < m_plots.size(); i++)
	{
		float relStartX = (float)(i - 1) / (float)(m_plots.size() - 1);
		float relEndX = (float)i / (float)(m_plots.size() - 1);
		float relStartY = 1.0f - (float)(m_plots[i - 1].total - m_minY) / (float)m_maxY;
		float relEndY = 1.0f - (float)(m_plots[i].total - m_minY) / (float)m_maxY;
		QLineF line(dataX + relStartX * dataW, dataY + relStartY * dataH,
			dataX + relEndX * dataW, dataY + relEndY * dataH);
		lines.append(line);
	}
	p.setPen(Theme::content);
	p.drawLines(lines);
}


void GraphWidget::setPlots(const vector<GraphPlot>& plots, size_t valuesPerPlot)
{
	m_plots = plots;
	m_valuesPerPlot = valuesPerPlot;
	sort(m_plots.begin(), m_plots.end(), [](const GraphPlot& a, const GraphPlot& b) {
		return a.date < b.date;
	});

	m_minY = m_plots[0].total;
	m_maxY = m_plots[0].total;
	for (auto& i : m_plots)
	{
		i.total = 0;
		for (size_t j = 0; j < valuesPerPlot; j++)
			i.total += i.value[j];
		if (i.total < m_minY)
			m_minY = i.total;
		if (i.total > m_maxY)
			m_maxY = i.total;
	}

	update();
}
