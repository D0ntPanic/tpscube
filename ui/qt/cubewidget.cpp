#include <QtGui/QMouseEvent>
#include <QtGui/QWheelEvent>
#include <QtCore/QEasingCurve>
#include <math.h>
#include "cubewidget.h"
#include "theme.h"

using namespace std;


QVector3D CubeWidget::m_faceColors[6] = {
	QVector3D(1, 1, 1),
	QVector3D(0.003f, 0.5f, 0.017f),
	QVector3D(0.9f, 0.003f, 0.003f),
	QVector3D(0.016f, 0.04f, 0.55f),
	QVector3D(1, 0.167f, 0.0025f),
	QVector3D(1, 1, 0.04f)
};

QVector3D CubeWidget::m_innerColor = QVector3D(0.02f, 0.02f, 0.02f);


CubeWidget::CubeWidget()
{
	m_cameraPosition = QVector3D(0, 0, 5);
	m_lightPosition = QVector3D(0, 2, 4);
	m_lightColor = QVector3D(40, 40, 40);
	m_pitch = 30;
	m_yaw = -35;
	m_rotation = QQuaternion::fromAxisAndAngle(1, 0, 0, m_pitch) *
		QQuaternion::fromAxisAndAngle(0, 1, 0, m_yaw);

	m_lastFrameTime = chrono::steady_clock::now();

	m_animationTimer = new QTimer(this);
	m_animationTimer->setSingleShot(false);
	m_animationTimer->setInterval(1000 / 60);
	connect(m_animationTimer, &QTimer::timeout, this, &CubeWidget::animate);
}


CubeWidget::~CubeWidget()
{
	makeCurrent();
	if (m_vertexArray)
	{
		m_vertexArray->destroy();
		delete m_vertexArray;
	}
	if (m_animVertexArray)
	{
		m_animVertexArray->destroy();
		delete m_animVertexArray;
	}
	if (m_indexBuffer)
	{
		m_indexBuffer->destroy();
		delete m_indexBuffer;
	}
	if (m_animFixedIndexBuffer)
	{
		m_animFixedIndexBuffer->destroy();
		delete m_animFixedIndexBuffer;
	}
	if (m_animMovingIndexBuffer)
	{
		m_animMovingIndexBuffer->destroy();
		delete m_animMovingIndexBuffer;
	}
	doneCurrent();
}


QSize CubeWidget::sizeHint() const
{
	return QSize(300, 300);
}


void CubeWidget::initializeGL()
{
	initializeOpenGLFunctions();
	glClearColor(Theme::backgroundWindow.redF(), Theme::backgroundWindow.greenF(),
		Theme::backgroundWindow.blueF(), 1.0f);

	m_program.addShaderFromSourceFile(QOpenGLShader::Vertex, ":/shaders/vertex.glsl");
	m_program.addShaderFromSourceFile(QOpenGLShader::Fragment, ":/shaders/fragment.glsl");
	m_program.link();
	m_program.bind();

	m_viewProjectionMatrixLocation = m_program.uniformLocation("u_viewProjectionMatrix");
	m_modelMatrixLocation = m_program.uniformLocation("u_modelMatrix");
	m_normalMatrixLocation = m_program.uniformLocation("u_normalMatrix");
	m_cameraPositionLocation = m_program.uniformLocation("u_cameraPosition");
	m_lightPositionLocation = m_program.uniformLocation("u_lightPosition");
	m_lightColorLocation = m_program.uniformLocation("u_lightColor");
	m_positionLocation = m_program.attributeLocation("a_position");
	m_normalLocation = m_program.attributeLocation("a_normal");
	m_colorLocation = m_program.attributeLocation("a_color");
	m_roughnessLocation = m_program.attributeLocation("a_roughness");

	glEnable(GL_DEPTH_TEST);
	glEnable(GL_CULL_FACE);
	glEnable(GL_MULTISAMPLE);

	m_vertexArray = new QOpenGLBuffer(QOpenGLBuffer::VertexBuffer);
	m_vertexArray->setUsagePattern(QOpenGLBuffer::DynamicDraw);
	m_vertexArray->create();
	m_animVertexArray = new QOpenGLBuffer(QOpenGLBuffer::VertexBuffer);
	m_animVertexArray->setUsagePattern(QOpenGLBuffer::DynamicDraw);
	m_animVertexArray->create();
	m_indexBuffer = new QOpenGLBuffer(QOpenGLBuffer::IndexBuffer);
	m_indexBuffer->create();
	m_animFixedIndexBuffer = new QOpenGLBuffer(QOpenGLBuffer::IndexBuffer);
	m_animFixedIndexBuffer->setUsagePattern(QOpenGLBuffer::DynamicDraw);
	m_animFixedIndexBuffer->create();
	m_animMovingIndexBuffer = new QOpenGLBuffer(QOpenGLBuffer::IndexBuffer);
	m_animMovingIndexBuffer->setUsagePattern(QOpenGLBuffer::DynamicDraw);
	m_animMovingIndexBuffer->create();

	m_vertRanges.resize(6 * cubeSize() * cubeSize());
	for (int i = 0; i < 6 * cubeSize() * cubeSize(); i++)
		m_vertRanges[i].count = 0;

	m_cubeModelOffset = QVector3D(1 - cubeSize(), 1 - cubeSize(), 1 - cubeSize());
	m_cubeModelScale = 1.0f / (float)cubeSize();

	addCorner(0, 0, 0, 1, -1, BOTTOM, 1, 0, BACK, 1, 1, LEFT, 1, 0);
	addCorner(1, 0, 0, 1, 0, BOTTOM, 1, 1, RIGHT, 1, 1, BACK, 1, 0);
	addCorner(0, 1, 0, -1, 2, TOP, 0, 0, LEFT, 0, 0, BACK, 0, 1);
	addCorner(1, 1, 0, -1, 1, TOP, 0, 1, BACK, 0, 0, RIGHT, 0, 1);
	addCorner(0, 0, 1, 1, 2, BOTTOM, 0, 0, LEFT, 1, 1, FRONT, 1, 0);
	addCorner(1, 0, 1, 1, 1, BOTTOM, 0, 1, FRONT, 1, 1, RIGHT, 1, 0);
	addCorner(0, 1, 1, -1, -1, TOP, 1, 0, FRONT, 0, 0, LEFT, 0, 1);
	addCorner(1, 1, 1, -1, 0, TOP, 1, 1, RIGHT, 0, 0, FRONT, 0, 1);

	int e = cubeSize() - 1;
	for (int i = 1; i < (cubeSize() - 1); i++)
	{
		addEdge(i, e, e, -1, -1, TOP, e, i, FRONT, 0, i);
		addEdge(0, e - i, e, 0, 2, FRONT, i, 0, LEFT, i, e);
		addEdge(e, e - i, e, 0, 0, FRONT, i, e, RIGHT, i, 0);
		addEdge(i, 0, e, 1, 1, BOTTOM, 0, i, FRONT, e, i);
		addEdge(e, e, i, -1, 0, TOP, i, e, RIGHT, 0, e - i);
		addEdge(e, e - i, 0, 2, 0, BACK, i, 0, RIGHT, i, e);
		addEdge(e, 0, i, 1, 0, BOTTOM, i, e, RIGHT, e, i);
		addEdge(e - i, e, 0, -1, 1, TOP, 0, e - i, BACK, 0, i);
		addEdge(e - i, 0, 0, 1, -1, BOTTOM, e, e - i, BACK, e, i);
		addEdge(0, e, i, -1, 2, TOP, i, 0, LEFT, 0, i);
		addEdge(0, e - i, 0, 2, 2, BACK, i, e, LEFT, i, 0);
		addEdge(0, 0, i, 1, 2, BOTTOM, e - i, 0, LEFT, e, i);
	}

	for (int row = 1; row < (cubeSize() - 1); row++)
	{
		for (int col = 1; col < (cubeSize() - 1); col++)
		{
			addCenter(col, e, row, -1, 0, 0, TOP, row, col);
			addCenter(col, e - row, e, 0, 0, 0, FRONT, row, col);
			addCenter(e, e - row, e - col, 0, 1, 0, RIGHT, row, col);
			addCenter(e - col, e - row, 0, 2, 0, 0, BACK, row, col);
			addCenter(0, e - row, col, 0, -1, 0, LEFT, row, col);
			addCenter(col, 0, e - row, 1, 0, 0, BOTTOM, row, col);
		}
	}

	m_animVerts = m_verts;

	m_vertexArray->bind();
	m_vertexArray->allocate(&m_verts[0], sizeof(CubeVertex) * m_verts.size());
	m_animVertexArray->bind();
	m_animVertexArray->allocate(&m_animVerts[0], sizeof(CubeVertex) * m_animVerts.size());
	m_indexBuffer->bind();
	m_indexBuffer->allocate(&m_index[0], sizeof(unsigned short) * m_index.size());
	m_animFixedIndexBuffer->bind();
	m_animFixedIndexBuffer->allocate(&m_index[0], sizeof(unsigned short) * m_index.size());
	m_animMovingIndexBuffer->bind();
	m_animMovingIndexBuffer->allocate(&m_index[0], sizeof(unsigned short) * m_index.size());
}


void CubeWidget::resizeGL(int width, int height)
{
	if (height == 0)
		return;

	float aspect = (float)width / (float)height;
	m_projectionMatrix.setToIdentity();
	m_projectionMatrix.perspective(45, aspect, 1.0f, 10.0f);
}


void CubeWidget::paintGL()
{
	glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

	m_viewMatrix.setToIdentity();
	m_viewMatrix.translate(-m_cameraPosition);
	m_modelMatrix.setToIdentity();
	m_modelMatrix.rotate(m_rotation);
	m_modelMatrix.scale(m_cubeModelScale);
	m_modelMatrix.translate(m_cubeModelOffset);

	m_program.setUniformValue(m_viewProjectionMatrixLocation, m_projectionMatrix * m_viewMatrix);
	m_program.setUniformValue(m_modelMatrixLocation, m_modelMatrix);
	m_program.setUniformValue(m_normalMatrixLocation, m_modelMatrix.normalMatrix());
	m_program.setUniformValue(m_cameraPositionLocation, m_cameraPosition);
	m_program.setUniformValue(m_lightPositionLocation, m_lightPosition);
	m_program.setUniformValue(m_lightColorLocation, m_lightColor);

	if (m_movementActive)
	{
		chrono::time_point<chrono::steady_clock> curTime = chrono::steady_clock::now();
		double dt = chrono::duration<double>(curTime - m_lastFrameTime).count();
		m_movementTimePassed += (float)dt;
		m_lastFrameTime = curTime;
		if (m_movementTimePassed >= m_movementLength)
		{
			m_movementActive = false;
			if (m_movementQueue.empty())
				m_animationTimer->stop();
		}
	}
	else if (!m_movementQueue.empty())
	{
		float tps = m_movementQueue.front().tps;
		if (m_movementQueue.front().allowAccel)
			tps *= m_movementQueue.size();
		startAnimation(m_movementQueue.front().move, tps);
		m_movementQueue.pop();
	}

	if (m_cubeNeedsUpdate)
		updateCubeModelColors();

	if (m_movementActive)
	{
		m_animVertexArray->bind();
		m_animFixedIndexBuffer->bind();
		int offset = 0;
		m_program.enableAttributeArray(m_positionLocation);
		m_program.setAttributeBuffer(m_positionLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_normalLocation);
		m_program.setAttributeBuffer(m_normalLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_colorLocation);
		m_program.setAttributeBuffer(m_colorLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_roughnessLocation);
		m_program.setAttributeBuffer(m_roughnessLocation, GL_FLOAT, offset, 1, sizeof(CubeVertex));

		glDrawElements(GL_TRIANGLES, m_animFixedIndex[m_movementFace].size(), GL_UNSIGNED_SHORT, nullptr);

		QEasingCurve angleCurve(QEasingCurve::InOutQuad);
		QMatrix4x4 animModelMatrix;
		animModelMatrix.setToIdentity();
		animModelMatrix.rotate(m_rotation);
		animModelMatrix.rotate(-m_movementAngle * angleCurve.valueForProgress(
			m_movementTimePassed / m_movementLength), m_movementAxis);
		animModelMatrix.scale(m_cubeModelScale);
		animModelMatrix.translate(m_cubeModelOffset);
		m_program.setUniformValue(m_modelMatrixLocation, animModelMatrix);
		m_program.setUniformValue(m_normalMatrixLocation, animModelMatrix.normalMatrix());

		m_animMovingIndexBuffer->bind();
		glDrawElements(GL_TRIANGLES, m_animMovingIndex[m_movementFace].size(), GL_UNSIGNED_SHORT, nullptr);
	}
	else
	{
		m_vertexArray->bind();
		m_indexBuffer->bind();
		int offset = 0;
		m_program.enableAttributeArray(m_positionLocation);
		m_program.setAttributeBuffer(m_positionLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_normalLocation);
		m_program.setAttributeBuffer(m_normalLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_colorLocation);
		m_program.setAttributeBuffer(m_colorLocation, GL_FLOAT, offset, 3, sizeof(CubeVertex));
		offset += sizeof(QVector3D);
		m_program.enableAttributeArray(m_roughnessLocation);
		m_program.setAttributeBuffer(m_roughnessLocation, GL_FLOAT, offset, 1, sizeof(CubeVertex));

		glDrawElements(GL_TRIANGLES, m_index.size(), GL_UNSIGNED_SHORT, nullptr);
	}
}


void CubeWidget::mousePressEvent(QMouseEvent* event)
{
	m_grabbed = true;
	m_lastMouseLocation = event->pos();
	grabMouse();
}


void CubeWidget::mouseReleaseEvent(QMouseEvent*)
{
	m_grabbed = false;
	releaseMouse();
}


void CubeWidget::mouseMoveEvent(QMouseEvent* event)
{
	if (m_grabbed)
	{
		int dx = event->x() - m_lastMouseLocation.x();
		int dy = event->y() - m_lastMouseLocation.y();
		m_lastMouseLocation = event->pos();

		adjustAngle(dx / 2.0f, dy / 2.0f);
	}
}


void CubeWidget::wheelEvent(QWheelEvent* event)
{
	if (event->pixelDelta().isNull())
	{
		adjustAngle(event->angleDelta().x() / 25.0f,
			event->angleDelta().y() / 25.0f);
	}
	else
	{
		adjustAngle(event->pixelDelta().x() / 2.0f,
			event->pixelDelta().y() / 2.0f);
	}

	m_yaw = fmodf(m_yaw, 360);
	m_pitch = fmodf(m_pitch, 360);
	if (m_yaw < 0)
		m_yaw += 360;
	if (m_pitch < 0)
		m_pitch += 360;

	m_rotation = QQuaternion::fromAxisAndAngle(1, 0, 0, m_pitch) *
		QQuaternion::fromAxisAndAngle(0, 1, 0, m_yaw);
	update();
}


void CubeWidget::adjustAngle(float dx, float dy)
{
	if ((m_pitch > 90) && (m_pitch < 270))
		m_yaw -= dx;
	else
		m_yaw += dx;
	m_pitch += dy;

	m_yaw = fmodf(m_yaw, 360);
	m_pitch = fmodf(m_pitch, 360);
	if (m_yaw < 0)
		m_yaw += 360;
	if (m_pitch < 0)
		m_pitch += 360;

	m_rotation = QQuaternion::fromAxisAndAngle(1, 0, 0, m_pitch) *
		QQuaternion::fromAxisAndAngle(0, 1, 0, m_yaw);
	update();
}


CubeVertexRange& CubeWidget::vertRange(CubeFace face, int row, int col)
{
	return m_vertRanges[(face * cubeSize() * cubeSize()) + (row * cubeSize()) + col];
}


void CubeWidget::addCorner(int x, int y, int z, int xRot, int zRot,
	CubeFace firstFace, int firstRow, int firstCol,
	CubeFace secondFace, int secondRow, int secondCol,
	CubeFace thirdFace, int thirdRow, int thirdCol)
{
	QMatrix4x4 rotation;
	rotation.rotate(xRot * 90, QVector3D(1, 0, 0));
	rotation.rotate(zRot * 90, QVector3D(0, 0, 1));

	size_t startIndex = m_verts.size();
	for (size_t i = 0; i < m_cornerVertexCount; i++)
	{
		CubeVertex vertex;
		vertex.position = rotation.map(m_cornerVertices[i].position) +
			QVector3D(x * (cubeSize() - 1) * 2, y * (cubeSize() - 1) * 2, z * (cubeSize() - 1) * 2);
		vertex.normal = rotation.map(m_cornerVertices[i].normal);
		if (m_cornerVertices[i].face == -1)
		{
			vertex.color = m_innerColor;
			vertex.roughness = 0.3f;
		}
		else
		{
			vertex.color = QVector3D(1, 0, 1);
			vertex.roughness = 0.4f;
		}
		m_verts.push_back(vertex);
	}

	for (size_t i = 0; i < m_cornerIndexCount; i++)
	{
		unsigned short idx = (unsigned short)(m_cornerIndex[i] + startIndex);
		m_index.push_back(idx);
		m_animMovingIndex[firstFace].push_back(idx);
		m_animMovingIndex[secondFace].push_back(idx);
		m_animMovingIndex[thirdFace].push_back(idx);
		for (size_t j = 0; j < 6; j++)
		{
			if ((firstFace != (CubeFace)j) && (secondFace != (CubeFace)j) &&
				(thirdFace != (CubeFace)j))
				m_animFixedIndex[(CubeFace)j].push_back(idx);
		}
	}

	vertRange(firstFace, firstRow * (cubeSize() - 1), firstCol * (cubeSize() - 1)) =
		CubeVertexRange { startIndex, m_cornerVertices, m_cornerVertexCount, 0 };
	vertRange(secondFace, secondRow * (cubeSize() - 1), secondCol * (cubeSize() - 1)) =
		CubeVertexRange { startIndex, m_cornerVertices, m_cornerVertexCount, 1 };
	vertRange(thirdFace, thirdRow * (cubeSize() - 1), thirdCol * (cubeSize() - 1)) =
		CubeVertexRange { startIndex, m_cornerVertices, m_cornerVertexCount, 2 };
}


void CubeWidget::addEdge(int x, int y, int z, int xRot, int zRot,
	CubeFace firstFace, int firstRow, int firstCol,
	CubeFace secondFace, int secondRow, int secondCol)
{
	QMatrix4x4 rotation;
	rotation.rotate(xRot * 90, QVector3D(1, 0, 0));
	rotation.rotate(zRot * 90, QVector3D(0, 0, 1));

	size_t startIndex = m_verts.size();
	for (size_t i = 0; i < m_edgeVertexCount; i++)
	{
		CubeVertex vertex;
		vertex.position = rotation.map(m_edgeVertices[i].position) +
			QVector3D(x * 2, y * 2, z * 2);
		vertex.normal = rotation.map(m_edgeVertices[i].normal);
		if (m_edgeVertices[i].face == -1)
		{
			vertex.color = m_innerColor;
			vertex.roughness = 0.3f;
		}
		else
		{
			vertex.color = QVector3D(1, 0, 1);
			vertex.roughness = 0.4f;
		}
		m_verts.push_back(vertex);
	}

	for (size_t i = 0; i < m_edgeIndexCount; i++)
	{
		unsigned short idx = (unsigned short)(m_edgeIndex[i] + startIndex);
		m_index.push_back(idx);
		m_animMovingIndex[firstFace].push_back(idx);
		m_animMovingIndex[secondFace].push_back(idx);
		for (size_t j = 0; j < 6; j++)
		{
			if ((firstFace != (CubeFace)j) && (secondFace != (CubeFace)j))
				m_animFixedIndex[(CubeFace)j].push_back(idx);
		}
	}

	vertRange(firstFace, firstRow, firstCol) =
		CubeVertexRange { startIndex, m_edgeVertices, m_edgeVertexCount, 0 };
	vertRange(secondFace, secondRow, secondCol) =
		CubeVertexRange { startIndex, m_edgeVertices, m_edgeVertexCount, 1 };
}


void CubeWidget::addCenter(int x, int y, int z, int xRot, int yRot, int zRot,
	CubeFace face, int row, int col)
{
	QMatrix4x4 rotation;
	rotation.rotate(xRot * 90, QVector3D(1, 0, 0));
	rotation.rotate(yRot * 90, QVector3D(0, 1, 0));
	rotation.rotate(zRot * 90, QVector3D(0, 0, 1));

	size_t startIndex = m_verts.size();
	for (size_t i = 0; i < m_centerVertexCount; i++)
	{
		CubeVertex vertex;
		vertex.position = rotation.map(m_centerVertices[i].position) +
			QVector3D(x * 2, y * 2, z * 2);
		vertex.normal = rotation.map(m_centerVertices[i].normal);
		if (m_centerVertices[i].face == -1)
		{
			vertex.color = m_innerColor;
			vertex.roughness = 0.3f;
		}
		else
		{
			vertex.color = QVector3D(1, 0, 1);
			vertex.roughness = 0.4f;
		}
		m_verts.push_back(vertex);
	}

	for (size_t i = 0; i < m_centerIndexCount; i++)
	{
		unsigned short idx = (unsigned short)(m_centerIndex[i] + startIndex);
		m_index.push_back(idx);
		m_animMovingIndex[face].push_back(idx);
		for (size_t j = 0; j < 6; j++)
		{
			if (face != (CubeFace)j)
				m_animFixedIndex[(CubeFace)j].push_back(idx);
		}
	}

	vertRange(face, row, col) = CubeVertexRange { startIndex, m_centerVertices, m_centerVertexCount, 0 };
}


void CubeWidget::startAnimation(CubeMove move, float tps)
{
	m_movementLength = 1.0f / tps;
	m_movementTimePassed = 0;
	m_movementColors = cubeFaceColors();
	m_movementActive = true;
	m_lastFrameTime = chrono::steady_clock::now();

	switch (move)
	{
	case MOVE_U:
		m_movementFace = TOP;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(0, 1, 0);
		break;
	case MOVE_Up:
		m_movementFace = TOP;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(0, 1, 0);
		break;
	case MOVE_U2:
		m_movementFace = TOP;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(0, 1, 0);
		break;
	case MOVE_F:
		m_movementFace = FRONT;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(0, 0, 1);
		break;
	case MOVE_Fp:
		m_movementFace = FRONT;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(0, 0, 1);
		break;
	case MOVE_F2:
		m_movementFace = FRONT;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(0, 0, 1);
		break;
	case MOVE_R:
		m_movementFace = RIGHT;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(1, 0, 0);
		break;
	case MOVE_Rp:
		m_movementFace = RIGHT;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(1, 0, 0);
		break;
	case MOVE_R2:
		m_movementFace = RIGHT;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(1, 0, 0);
		break;
	case MOVE_B:
		m_movementFace = BACK;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(0, 0, -1);
		break;
	case MOVE_Bp:
		m_movementFace = BACK;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(0, 0, -1);
		break;
	case MOVE_B2:
		m_movementFace = BACK;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(0, 0, -1);
		break;
	case MOVE_L:
		m_movementFace = LEFT;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(-1, 0, 0);
		break;
	case MOVE_Lp:
		m_movementFace = LEFT;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(-1, 0, 0);
		break;
	case MOVE_L2:
		m_movementFace = LEFT;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(-1, 0, 0);
		break;
	case MOVE_D:
		m_movementFace = BOTTOM;
		m_movementAngle = 90;
		m_movementAxis = QVector3D(0, -1, 0);
		break;
	case MOVE_Dp:
		m_movementFace = BOTTOM;
		m_movementAngle = -90;
		m_movementAxis = QVector3D(0, -1, 0);
		break;
	case MOVE_D2:
		m_movementFace = BOTTOM;
		m_movementAngle = 180;
		m_movementAxis = QVector3D(0, -1, 0);
		break;
	}

	applyMove(move);
	m_cubeNeedsUpdate = true;
	m_animationTimer->start();

	m_animFixedIndexBuffer->bind();
	m_animFixedIndexBuffer->write(0, &m_animFixedIndex[m_movementFace][0],
		sizeof(unsigned short) * m_animFixedIndex[m_movementFace].size());
	m_animMovingIndexBuffer->bind();
	m_animMovingIndexBuffer->write(0, &m_animMovingIndex[m_movementFace][0],
		sizeof(unsigned short) * m_animMovingIndex[m_movementFace].size());
}


void CubeWidget::updateCubeModelColors()
{
	vector<CubeColor> colors = cubeFaceColors();
	for (size_t i = 0; i < colors.size(); i++)
	{
		if (i >= m_vertRanges.size())
			break;

		QVector3D color = m_faceColors[colors[i]];

		for (size_t j = 0; j < m_vertRanges[i].count; j++)
		{
			if (m_vertRanges[i].modelVerts[j].face != m_vertRanges[i].face)
				continue;
			m_verts[m_vertRanges[i].startIndex + j].color = color;
		}
	}

	m_vertexArray->bind();
	m_vertexArray->write(0, &m_verts[0], sizeof(CubeVertex) * m_verts.size());

	if (m_movementActive)
	{
		for (size_t i = 0; i < m_movementColors.size(); i++)
		{
			if (i >= m_vertRanges.size())
				break;

			QVector3D color = m_faceColors[m_movementColors[i]];

			for (size_t j = 0; j < m_vertRanges[i].count; j++)
			{
				if (m_vertRanges[i].modelVerts[j].face != m_vertRanges[i].face)
					continue;
				m_animVerts[m_vertRanges[i].startIndex + j].color = color;
			}
		}

		m_animVertexArray->bind();
		m_animVertexArray->write(0, &m_animVerts[0], sizeof(CubeVertex) * m_animVerts.size());
	}
}


void CubeWidget::apply(const CubeMoveSequence& moves, float tps, bool accel)
{
	for (auto& i : moves.moves)
		m_movementQueue.push(QueuedCubeMove { i, tps, accel });
	m_animationTimer->start();
}


void CubeWidget::applyImmediate(const CubeMoveSequence& moves)
{
	while (!m_movementQueue.empty())
	{
		applyMove(m_movementQueue.front().move);
		m_movementQueue.pop();
	}
	m_movementActive = false;

	for (auto& i : moves.moves)
		applyMove(i);

	m_cubeNeedsUpdate = true;
	m_animationTimer->stop();
}


void CubeWidget::animate()
{
	update();
}
