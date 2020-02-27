#pragma once

#include <QtWidgets/QWidget>

struct GraphPlot
{
	time_t date;
	float value[4];
	float total;
};

class GraphWidget: public QWidget
{
	QString m_message;
	std::vector<GraphPlot> m_plots;
	size_t m_valuesPerPlot;
	float m_minY, m_maxY;

protected:
	virtual void paintEvent(QPaintEvent* event) override;

public:
	GraphWidget();

	void setMessage(const QString& msg) { m_message = msg; }
	void setPlots(const std::vector<GraphPlot>& plots, size_t valuesPerPlot);
};
