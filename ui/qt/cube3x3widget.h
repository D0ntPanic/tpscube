#pragma once

#include <QtWidgets/QOpenGLWidget>
#include <QtGui/QOpenGLFunctions>
#include <QtGui/QOpenGLShaderProgram>
#include <QtGui/QOpenGLBuffer>
#include "cube3x3.h"

struct Cube3x3Vertex
{
	QVector3D position;
	QVector3D color;
};

class Cube3x3Widget: public QOpenGLWidget, protected QOpenGLFunctions
{
	Cube3x3 m_cube;

	QOpenGLShaderProgram m_program;
	QMatrix4x4 m_projectionMatrix;
	QMatrix4x4 m_modelMatrix, m_viewMatrix;
	QQuaternion m_rotation;
	int m_modelViewProjectionLocation;
	int m_positionLocation;
	int m_colorLocation;

	QOpenGLBuffer* m_vertexArray = nullptr;
	QOpenGLBuffer* m_indexBuffer = nullptr;

	bool m_grabbed = false;
	QPoint m_lastMouseLocation;

protected:
	virtual void initializeGL() override;
	virtual void resizeGL(int width, int height) override;
	virtual void paintGL() override;

	virtual void mousePressEvent(QMouseEvent* event) override;
	virtual void mouseReleaseEvent(QMouseEvent* event) override;
	virtual void mouseMoveEvent(QMouseEvent* event) override;

public:
	Cube3x3Widget();
	~Cube3x3Widget();

	virtual QSize sizeHint() const override;

	Cube3x3& cube() { return m_cube; }
	const Cube3x3& cube() const { return m_cube; }
};
