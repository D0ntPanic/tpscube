#pragma once

#include <QtWidgets/QWidget>

struct GraphPlot
{
	time_t date;
	float value[4];
	float total;
};

struct GraphAxisLabel
{
	float start, center;
	QString text;
};

class GraphWidget: public QWidget
{
	QString m_message, m_yAxisLabel;
	std::vector<GraphPlot> m_rawPlots;
	std::vector<GraphPlot> m_plots;
	size_t m_averageSize;
	size_t m_valuesPerPlot;
	bool m_valuesAreTimes = true;
	float m_totalMaxY;
	float m_minY[4], m_maxY[4];
	QColor m_colors[4];

protected:
	virtual void paintEvent(QPaintEvent* event) override;

public:
	GraphWidget();

	void setMessage(const QString& msg) { m_message = msg; }
	void setYAxisLabel(const QString& label) { m_yAxisLabel = label; }
	void setValuesAreTimes(bool t) { m_valuesAreTimes = t; }
	void setPlots(const std::vector<GraphPlot>& plots, size_t valuesPerPlot,
		const std::vector<QColor>& colors, size_t averageSize);

	static QString stringForDay(time_t date);
	QString stringForValue(int value);
};
