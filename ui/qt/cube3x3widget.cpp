#include <QtGui/QMouseEvent>
#include <math.h>
#include "cube3x3widget.h"
#include "theme.h"


Cube3x3Widget::Cube3x3Widget()
{
	m_rotation = QQuaternion::fromAxisAndAngle(1, 0, 0, 30) *
		QQuaternion::fromAxisAndAngle(0, 1, 0, 45);

	m_viewMatrix.setToIdentity();
	m_viewMatrix.translate(0, 0, -5.0f);
}


Cube3x3Widget::~Cube3x3Widget()
{
	makeCurrent();
	if (m_vertexArray)
	{
		m_vertexArray->destroy();
		delete m_vertexArray;
	}
	if (m_indexBuffer)
	{
		m_indexBuffer->destroy();
		delete m_indexBuffer;
	}
	doneCurrent();
}


QSize Cube3x3Widget::sizeHint() const
{
	return QSize(300, 300);
}


void Cube3x3Widget::initializeGL()
{
	initializeOpenGLFunctions();
	glClearColor(Theme::backgroundWindow.redF(), Theme::backgroundWindow.greenF(),
		Theme::backgroundWindow.blueF(), 1.0f);

	m_program.addShaderFromSourceFile(QOpenGLShader::Vertex, ":/shaders/vertex.glsl");
	m_program.addShaderFromSourceFile(QOpenGLShader::Fragment, ":/shaders/fragment.glsl");
	m_program.link();
	m_program.bind();

	m_modelViewProjectionLocation = m_program.uniformLocation("u_modelViewProjection");
	m_positionLocation = m_program.attributeLocation("a_position");
	m_colorLocation = m_program.attributeLocation("a_color");

	glEnable(GL_DEPTH_TEST);
	glEnable(GL_CULL_FACE);

	m_vertexArray = new QOpenGLBuffer(QOpenGLBuffer::VertexBuffer);
	m_vertexArray->create();
	m_indexBuffer = new QOpenGLBuffer(QOpenGLBuffer::IndexBuffer);
	m_indexBuffer->create();

	static Cube3x3Vertex verticies[] = {
		{QVector3D(-1, -1, 1), QVector3D(0, 0, 1)},
		{QVector3D(1, -1, 1), QVector3D(0, 0, 1)},
		{QVector3D(-1, 1, 1), QVector3D(0, 0, 1)},
		{QVector3D(1, 1, 1), QVector3D(0, 0, 1)},
		{QVector3D(1, -1, 1), QVector3D(1, 0, 0)},
		{QVector3D(1, -1, -1), QVector3D(1, 0, 0)},
		{QVector3D(1, 1, 1), QVector3D(1, 0, 0)},
		{QVector3D(1, 1, -1), QVector3D(1, 0, 0)},
		{QVector3D(1, -1, -1), QVector3D(0, 1, 0)},
		{QVector3D(-1, -1, -1), QVector3D(0, 1, 0)},
		{QVector3D(1, 1, -1), QVector3D(0, 1, 0)},
		{QVector3D(-1, 1, -1), QVector3D(0, 1, 0)},
		{QVector3D(-1, -1, -1), QVector3D(1, 0.5f, 0)},
		{QVector3D(-1, -1, 1), QVector3D(1, 0.5f, 0)},
		{QVector3D(-1, 1, -1), QVector3D(1, 0.5f, 0)},
		{QVector3D(-1, 1, 1), QVector3D(1, 0.5f, 0)},
		{QVector3D(-1, -1, -1), QVector3D(1, 1, 0)},
		{QVector3D(1, -1, -1), QVector3D(1, 1, 0)},
		{QVector3D(-1, -1, 1), QVector3D(1, 1, 0)},
		{QVector3D(1, -1, 1), QVector3D(1, 1, 0)},
		{QVector3D(-1, 1, 1), QVector3D(1, 1, 1)},
		{QVector3D(1, 1, 1), QVector3D(1, 1, 1)},
		{QVector3D(-1, 1, -1), QVector3D(1, 1, 1)},
		{QVector3D(1, 1, -1), QVector3D(1, 1, 1)}
	};
	static unsigned short indicies[] = {
		0, 1, 2, 3, 3,
		4, 4, 5, 6, 7, 7,
		8, 8, 9, 10, 11, 11,
		12, 12, 13, 14, 15, 15,
		16, 16, 17, 18, 19, 19,
		20, 20, 21, 22, 23
	};

	m_vertexArray->bind();
	m_vertexArray->allocate(verticies, sizeof(verticies));
	m_indexBuffer->bind();
	m_indexBuffer->allocate(indicies, sizeof(indicies));
}


void Cube3x3Widget::resizeGL(int width, int height)
{
	if (height == 0)
		return;

	float aspect = (float)width / (float)height;
	m_projectionMatrix.setToIdentity();
	m_projectionMatrix.perspective(45, aspect, 1.0f, 10.0f);
}


void Cube3x3Widget::paintGL()
{
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	m_modelMatrix.setToIdentity();
	m_modelMatrix.rotate(m_rotation);

	m_program.setUniformValue(m_modelViewProjectionLocation,
		m_projectionMatrix * m_viewMatrix * m_modelMatrix);

	m_vertexArray->bind();
	m_indexBuffer->bind();
	m_program.enableAttributeArray(m_positionLocation);
	m_program.setAttributeBuffer(m_positionLocation, GL_FLOAT, 0, 3, sizeof(Cube3x3Vertex));
	m_program.enableAttributeArray(m_colorLocation);
	m_program.setAttributeBuffer(m_colorLocation, GL_FLOAT, sizeof(QVector3D), 3, sizeof(Cube3x3Vertex));

	glDrawElements(GL_TRIANGLE_STRIP, 34, GL_UNSIGNED_SHORT, nullptr);
}


void Cube3x3Widget::mousePressEvent(QMouseEvent* event)
{
	m_grabbed = true;
	m_lastMouseLocation = event->pos();
	grabMouse();
}


void Cube3x3Widget::mouseReleaseEvent(QMouseEvent*)
{
	m_grabbed = false;
	releaseMouse();
}


void Cube3x3Widget::mouseMoveEvent(QMouseEvent* event)
{
	if (m_grabbed)
	{
		int dx = event->x() - m_lastMouseLocation.x();
		int dy = event->y() - m_lastMouseLocation.y();
		m_lastMouseLocation = event->pos();

		m_rotation = QQuaternion::fromAxisAndAngle(0, 1, 0, dx / 2.0f) * m_rotation;
		m_rotation = QQuaternion::fromAxisAndAngle(1, 0, 0, dy / 2.0f) * m_rotation;
		update();
	}
}
