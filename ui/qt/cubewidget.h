#pragma once

#include <QtWidgets/QOpenGLWidget>
#include <QtGui/QOpenGLFunctions>
#include <QtGui/QOpenGLShaderProgram>
#include <QtGui/QOpenGLBuffer>
#include <QtCore/QTimer>
#include <chrono>
#include <queue>
#include "cubecommon.h"

struct CubeModelVertex
{
	QVector3D position;
	QVector3D normal;
	int face;
};

struct CubeVertex
{
	QVector3D position;
	QVector3D normal;
	QVector3D color;
	float roughness;
};

struct CubeVertexRange
{
	size_t startIndex;
	CubeModelVertex* modelVerts;
	size_t count;
	int face;
};

struct QueuedCubeMove
{
	CubeMove move;
	float tps;
};

class CubeWidget: public QOpenGLWidget, protected QOpenGLFunctions
{
	Q_OBJECT

protected:
	QOpenGLShaderProgram m_program;
	QMatrix4x4 m_projectionMatrix;
	QMatrix4x4 m_modelMatrix, m_viewMatrix;
	int m_viewProjectionMatrixLocation, m_modelMatrixLocation, m_normalMatrixLocation;
	int m_cameraPositionLocation, m_lightPositionLocation, m_lightColorLocation;
	int m_positionLocation, m_normalLocation, m_colorLocation, m_roughnessLocation;

	QVector3D m_cameraPosition;
	float m_yaw, m_pitch;
	QQuaternion m_rotation;
	QVector3D m_lightPosition, m_lightColor;

	QOpenGLBuffer* m_vertexArray = nullptr;
	QOpenGLBuffer* m_animVertexArray = nullptr;
	QOpenGLBuffer* m_indexBuffer = nullptr;
	QOpenGLBuffer* m_animFixedIndexBuffer = nullptr;
	QOpenGLBuffer* m_animMovingIndexBuffer = nullptr;

	bool m_grabbed = false;
	QPoint m_lastMouseLocation;

	std::vector<CubeVertex> m_verts;
	std::vector<CubeVertex> m_animVerts;
	std::vector<unsigned short> m_index;
	std::map<CubeFace, std::vector<unsigned short>> m_animFixedIndex;
	std::map<CubeFace, std::vector<unsigned short>> m_animMovingIndex;
	std::vector<CubeVertexRange> m_vertRanges;

	QVector3D m_cubeModelOffset;
	float m_cubeModelScale;
	bool m_cubeNeedsUpdate = true;

	bool m_movementActive = false;
	std::vector<CubeColor> m_movementColors;
	CubeFace m_movementFace;
	QVector3D m_movementAxis;
	float m_movementAngle;
	float m_movementTimePassed, m_movementLength;
	QTimer* m_animationTimer;
	std::chrono::time_point<std::chrono::steady_clock> m_lastFrameTime;

	std::queue<QueuedCubeMove> m_movementQueue;

	static QVector3D m_faceColors[6];
	static QVector3D m_innerColor;

	static CubeModelVertex* m_cornerVertices;
	static size_t m_cornerVertexCount;
	static unsigned short* m_cornerIndex;
	static size_t m_cornerIndexCount;
	static CubeModelVertex* m_edgeVertices;
	static size_t m_edgeVertexCount;
	static unsigned short* m_edgeIndex;
	static size_t m_edgeIndexCount;
	static CubeModelVertex* m_centerVertices;
	static size_t m_centerVertexCount;
	static unsigned short* m_centerIndex;
	static size_t m_centerIndexCount;

	virtual void initializeGL() override;
	virtual void resizeGL(int width, int height) override;
	virtual void paintGL() override;

	virtual void mousePressEvent(QMouseEvent* event) override;
	virtual void mouseReleaseEvent(QMouseEvent* event) override;
	virtual void mouseMoveEvent(QMouseEvent* event) override;
	virtual void wheelEvent(QWheelEvent* event) override;
	void adjustAngle(float dx, float dy);

	CubeVertexRange& vertRange(CubeFace face, int row, int col);

	void addCorner(int x, int y, int z, int xRot, int zRot,
		CubeFace firstFace, int firstRow, int firstCol,
		CubeFace secondFace, int secondRow, int secondCol,
		CubeFace thirdFace, int thirdRow, int thirdCol);
	void addEdge(int x, int y, int z, int xRot, int zRot,
		CubeFace firstFace, int firstRow, int firstCol,
		CubeFace secondFace, int secondRow, int secondCol);
	void addCenter(int x, int y, int z, int xRot, int yRot, int zRot,
		CubeFace face, int row, int col);

	void startAnimation(CubeMove move, float tps);
	void updateCubeModelColors();

	virtual void applyMove(CubeMove move) = 0;

private slots:
	void animate();

public:
	CubeWidget();
	~CubeWidget();

	virtual QSize sizeHint() const override;
	virtual int cubeSize() const = 0;
	virtual std::vector<CubeColor> cubeFaceColors() const = 0;

	void apply(const CubeMoveSequence& moves, float tps);
	void applyImmediate(const CubeMoveSequence& moves);
};
