#include <QtGui/QPainter>
#include <QtCore/QDateTime>
#include <algorithm>
#include "graphwidget.h"
#include "theme.h"
#include "historymode.h"

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

	p.setRenderHint(QPainter::HighQualityAntialiasing);

	QFont axisLabelFont(fontOfRelativeSize(1.0f, QFont::Thin));
	QFontMetricsF axisLabelMetrics(axisLabelFont);
	float axisLabelHeight = axisLabelMetrics.height();

	QFont axisNumberFont(fontOfRelativeSize(1.0f));
	QFontMetricsF axisNumberMetric(axisNumberFont);
	float axisNumberHeight = axisNumberMetric.height();

	float dataX = 32.0f + axisLabelHeight;
	float dataY = 16.0f;
	float dataW = (float)width() - (48.0f + axisLabelHeight);
	float dataH = (float)height() - (48.0f + axisLabelHeight + axisNumberHeight);

	// Determine Y axis marking values and text width
	int maxYInt = (int)m_totalMaxY;
	int maxLabels = (int)(dataH / (axisNumberHeight * 2.5f));
	if (maxLabels < 10)
		maxLabels = 10;
	int minStep = maxYInt / maxLabels;
	int maxStep = maxYInt / 4;
	int step = maxStep;
	if (step < 1)
		step = 1;
	else if ((minStep < 2) && (maxStep > 2))
		step = 2;
	else if ((minStep < 5) && (maxStep > 5))
		step = 5;
	else if ((minStep < 10) && (maxStep > 10))
		step = 10;
	else if (m_valuesAreTimes && (minStep < 15) && (maxStep > 15))
		step = 15;
	else if ((minStep < 20) && (maxStep > 20))
		step = 20;
	else if (m_valuesAreTimes && (minStep < 30) && (maxStep > 30))
		step = 30;
	else if (!m_valuesAreTimes && (minStep < 50) && (maxStep > 50))
		step = 50;
	else if (m_valuesAreTimes && (minStep < 60) && (maxStep > 60))
		step = 60;
	else if (!m_valuesAreTimes && (minStep < 100) && (maxStep > 100))
		step = 100;
	float maxXAxisLabelW = 0;
	for (int i = step; i < maxYInt; i += step)
	{
		QString text = stringForValue(i);
		float w = axisNumberMetric.horizontalAdvance(text);
		if (w > maxXAxisLabelW)
			maxXAxisLabelW = w;
	}

	dataX += maxXAxisLabelW;
	dataW -= maxXAxisLabelW;

	// Draw Y axis labels
	for (int i = step; i <= maxYInt; i += step)
	{
		float y = dataY + (1.0f - (float)i / (float)m_totalMaxY) * dataH;
		QString text = stringForValue(i);
		p.setPen(QPen(Theme::content, 2.0f));
		p.drawLine(dataX - 4, y, dataX, y);
		p.drawText(dataX - (12 + axisNumberMetric.horizontalAdvance(text)),
			(y - axisNumberMetric.height() * 0.5f) + axisNumberMetric.ascent(), text);
		p.setPen(QPen(Theme::backgroundWindow, 1.0f));
		p.drawLine(dataX, y, dataX + dataW, y);
	}

	// Draw X axis labels. First try to draw axis markings at the beginning of days.
	QString lastTimeStr = stringForDay(m_plots[0].date);
	float minX = 0;
	float maxX = (float)width();
	vector<GraphAxisLabel> labels;
	for (size_t i = 1; i < m_plots.size(); i++)
	{
		float x = dataX + ((float)i / (float)(m_plots.size() - 1)) * dataW;
		QString curTimeStr = stringForDay(m_plots[i].date);
		if (curTimeStr != lastTimeStr)
		{
			float w = axisNumberMetric.horizontalAdvance(curTimeStr);
			float textX = x - (w * 0.5f);
			if ((textX >= minX) && ((textX + w) < maxX))
			{
				labels.push_back(GraphAxisLabel { textX, x, curTimeStr });
				minX = textX + w + 24.0f;
			}
			lastTimeStr = curTimeStr;
		}
	}

	if (labels.size() < 3)
	{
		// Not enough labels to label by day, draw axis markings wherever they can fit
		minX = 0;
		labels.clear();
		for (size_t i = 0; i < m_plots.size(); i++)
		{
			float x = dataX + ((float)i / (float)(m_plots.size() - 1)) * dataW;
			QString curTimeStr = HistoryMode::shortStringForDate(m_plots[i].date);
			float w = axisNumberMetric.horizontalAdvance(curTimeStr);
			float textX = x - (w * 0.5f);
			if ((textX >= minX) && ((textX + w) < maxX))
			{
				labels.push_back(GraphAxisLabel { textX, x, curTimeStr });
				minX = textX + w + 24.0f;
			}
		}
	}

	for (auto& i : labels)
	{
		p.setPen(QPen(Theme::content, 2.0f));
		p.drawLine(i.center, dataY + dataH, i.center, dataY + dataH + 4);
		p.drawText(i.start, dataY + dataH + 6 + axisNumberMetric.ascent(), i.text);
	}

	// Draw axis labels
	p.setFont(axisLabelFont);
	p.setPen(Theme::disabled);
	QTextOption option;
	option.setAlignment(Qt::AlignCenter);
	p.drawText(QRectF(dataX, dataY + dataH + axisNumberHeight + 16.0f, dataW, axisLabelHeight),
		"Solves by Date of Solve", option);
	p.save();
	p.rotate(-90);
	p.drawText(QRectF(-dataY - dataH, dataX - (axisLabelHeight + maxXAxisLabelW + 24.0f), dataH, axisLabelHeight),
		m_yAxisLabel, option);
	p.restore();

	// Draw plot
	QVector<QPointF> points[4];
	for (size_t valueIdx = 0; valueIdx < m_valuesPerPlot; valueIdx++)
	{
		float minY = 0;
		if (valueIdx > 0)
			minY = m_minY[valueIdx - 1];
		minY = dataY + (1.0f - (float)minY / (float)m_totalMaxY) * dataH;
		float maxY = dataY + (1.0f - (float)m_maxY[valueIdx] / (float)m_totalMaxY) * dataH;

		for (size_t i = 0; i < m_plots.size(); i++)
		{
			float x = (float)((m_plots.size() - 1) - i) / (float)(m_plots.size() - 1);
			float y = 0;
			if (valueIdx > 0)
			{
				for (size_t j = 0; j < valueIdx; j++)
					y += m_plots[(m_plots.size() - 1) - i].value[j];
			}
			y = 1.0f - y / (float)m_totalMaxY;
			points[valueIdx].append(QPointF(dataX + x * dataW, dataY + y * dataH));
		}

		for (size_t i = 0; i < m_plots.size(); i++)
		{
			float x = (float)i / (float)(m_plots.size() - 1);
			float y = m_plots[i].value[valueIdx];
			if (valueIdx > 0)
			{
				for (size_t j = 0; j < valueIdx; j++)
					y += m_plots[i].value[j];
			}
			y = 1.0f - y / (float)m_totalMaxY;
			points[valueIdx].append(QPointF(dataX + x * dataW, dataY + y * dataH));
		}

		QLinearGradient fill(QPointF(0, maxY), QPointF(0, minY));
		QColor dataColor = MixColor(Theme::background, m_colors[valueIdx], 224);
		dataColor.setAlpha(224);
		fill.setColorAt(0, dataColor);
		dataColor = MixColor(Theme::background, m_colors[valueIdx], 128);
		dataColor.setAlpha(128);
		fill.setColorAt(1, dataColor);
		p.setPen(QPen(Theme::disabled, 1.0f));
		p.setBrush(QBrush(fill));
		p.drawPolygon(QPolygonF(points[valueIdx]));
	}

	for (size_t valueIdx = 0; valueIdx < m_valuesPerPlot; valueIdx++)
	{
		p.setPen(QPen(m_colors[valueIdx], 2.0f));
		p.drawPolyline(QPolygonF(points[valueIdx].mid(m_plots.size())));
	}

	// Draw axis lines
	p.setPen(QPen(Theme::content, 3.0f));
	QVector<QPointF> axisPoints;
	axisPoints.append(QPointF(dataX, dataY));
	axisPoints.append(QPointF(dataX, dataY + dataH));
	axisPoints.append(QPointF(dataX + dataW, dataY + dataH));
	p.drawPolyline(QPolygonF(axisPoints));
}


void GraphWidget::setPlots(const vector<GraphPlot>& plots, size_t valuesPerPlot,
	const std::vector<QColor>& colors, size_t averageSize)
{
	// Sort plots by time
	m_rawPlots = plots;
	m_valuesPerPlot = valuesPerPlot;
	m_averageSize = averageSize;
	sort(m_rawPlots.begin(), m_rawPlots.end(), [](const GraphPlot& a, const GraphPlot& b) {
		return a.date < b.date;
	});

	for (size_t i = 0; i < valuesPerPlot; i++)
		m_colors[i] = colors[i];

	// Compute averages for plots
	m_plots.clear();
	vector<float> avgValues;
	if (m_rawPlots.size() >= m_averageSize)
	{
		for (size_t i = 0; i <= (m_rawPlots.size() - m_averageSize); i++)
		{
			GraphPlot plot;
			plot.date = m_rawPlots[i + m_averageSize - 1].date;
			for (size_t j = 0; j < valuesPerPlot; j++)
			{
				avgValues.resize(m_averageSize);
				for (size_t k = 0; k < m_averageSize; k++)
					avgValues[k] = m_rawPlots[i + k].value[j];
				sort(avgValues.begin(), avgValues.end());
				avgValues.erase(avgValues.begin());
				avgValues.erase(avgValues.end() - 1);
				float value = 0;
				for (auto k : avgValues)
					value += k;
				value /= (float)avgValues.size();
				plot.value[j] = value;
			}
			m_plots.push_back(plot);
		}
	}

	// Compute plot extents
	if (m_plots.size() == 0)
	{
		m_totalMaxY = 2.0f;
	}
	else
	{
		m_plots[0].total = 0;
		for (size_t i = 0; i < valuesPerPlot; i++)
		{
			m_plots[0].total += m_plots[0].value[i];
			float val = m_plots[0].value[i];
			for (size_t j = 0; j < i; j++)
				val += m_plots[0].value[j];
			m_minY[i] = val;
			m_maxY[i] = val;
		}

		m_totalMaxY = m_plots[0].total;

		for (auto& i : m_plots)
		{
			i.total = 0;
			for (size_t j = 0; j < valuesPerPlot; j++)
			{
				i.total += i.value[j];
				float val = i.value[j];
				for (size_t k = 0; k < j; k++)
					val += i.value[k];
				if (val < m_minY[j])
					m_minY[j] = val;
				if (val > m_maxY[j])
					m_maxY[j] = val;
			}
			if (i.total > m_totalMaxY)
				m_totalMaxY = i.total;
		}

		if (m_totalMaxY < 2.0f)
			m_totalMaxY = 2.0f;
	}

	update();
}


QString GraphWidget::stringForDay(time_t date)
{
	QDateTime dt = QDateTime::fromTime_t(date);
	QDateTime now = QDateTime::currentDateTime();
	if (dt.daysTo(now) == 0)
		return "Today";
	else if (dt.daysTo(now) < 7)
		return dt.toString("ddd");
	else if (dt.daysTo(now) < 365)
		return dt.toString("MMM d");
	return dt.toString("MMM d, yyyy");
}


QString GraphWidget::stringForValue(int value)
{
	if (m_valuesAreTimes)
	{
		if (value >= 60)
			return QString::asprintf("%d:%.2d", value / 60, value % 60);
	}
	return QString::asprintf("%d", value);
}
