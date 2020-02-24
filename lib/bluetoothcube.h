#pragma once

#include <chrono>
#include "cube3x3.h"

struct Quaternion
{
	float x, y, z, w;
};

struct BatteryState
{
	int percent;
	bool charging;
};

class BluetoothCube;

class BluetoothDevice
{
	std::function<void()> m_connectedFunc;
	std::function<void(const std::string& msg)> m_errorHandler;

public:
	virtual ~BluetoothDevice() {}

	void SetConnectedCallback(const std::function<void()>& connectedFunc);
	void SetErrorCallback(const std::function<void(const std::string& msg)>& errorHandler);
	void Connect();

	virtual std::string GetName() = 0;
	virtual void ConnectToService(const std::string& uuid,
		const std::function<void()>& serviceConnectedFunc) = 0;
	virtual void ReadCharacteristic(const std::string& uuid,
		const std::function<void(const std::vector<uint8_t>& data)>& resultFunc) = 0;
	virtual void ReadEncodedCharacteristic(const std::string& uuid,
		const std::function<void(const std::vector<uint8_t>& data)>& resultFunc) = 0;
	virtual void SetDecoder(const std::function<std::vector<uint8_t>(const std::vector<uint8_t>&)>& decodeFunc) = 0;
	virtual void WriteCharacteristic(const std::string& uuid, const std::vector<uint8_t>& data,
		const std::function<void()>& doneFunc) = 0;
	virtual void EnableNotifications(const std::string& uuid, const std::function<void()>& doneFunc) = 0;
	void Error(const std::string& msg);
	virtual void DebugMessage(const std::string& msg) { (void)msg; }
};

class BluetoothCubeClient
{
	TimedCubeMoveSequence m_moves;
	std::function<void(const std::string& msg)> m_errorHandler;

public:
	TimedCubeMoveSequence GetLatestMoves();
	void SetErrorCallback(const std::function<void(const std::string& msg)>& errorHandler);

	void AddMove(TimedCubeMove move);
	void Error(const std::string& msg);
};

class BluetoothCube
{
protected:
	BluetoothDevice* m_dev;
	std::function<void()> m_readyFunc;
	std::vector<std::shared_ptr<BluetoothCubeClient>> m_clients;

	void AddMove(TimedCubeMove move);

public:
	BluetoothCube(BluetoothDevice* dev);
	virtual ~BluetoothCube();

	BluetoothDevice* GetDevice() const { return m_dev; }

	void SetReadyCallback(const std::function<void()>& readyFunc);
	void Ready();

	void AddClient(const std::shared_ptr<BluetoothCubeClient>& client);
	void RemoveClient(const std::shared_ptr<BluetoothCubeClient>& client);

	virtual Cube3x3 GetCubeState() = 0;
	virtual void ResetToSolved() = 0;
	virtual bool HasOrientation() = 0;
	virtual Quaternion GetOrientation() = 0;
	virtual BatteryState GetBatteryState() = 0;

	virtual void Update() = 0;
};

struct GANCubeLastMoveData
{
	int16_t orientation[3];
	uint8_t faceRotation[6];
	uint8_t moveCount;
	uint8_t move[6];
};

class GANCube: public BluetoothCube
{
	Cube3x3 m_cube;
	BatteryState m_battery;
	bool m_hasOrientation = false;
	Quaternion m_orientation = {0, 0, 0, 1};

	uint8_t m_lastMoveCount;
	bool m_firstMove = true;

	uint64_t m_currentTimestamp = 0;
	uint64_t m_totalCubeTicks = 0;
	uint64_t m_lastRealTicks = 0;
	uint64_t m_baseRealTicks = 0;
	float m_clockRatio = 0.95f;
	std::chrono::time_point<std::chrono::steady_clock> m_startTime, m_lastMoveTime;

	std::chrono::time_point<std::chrono::steady_clock> m_lastBatteryUpdateTime;

	bool m_updateInProgress = false;
	bool m_resetRequested = false;

	uint8_t m_deviceKey[16];

	static std::string m_deviceInfoService;
	static std::string m_versionCharacteristic;
	static std::string m_hardwareCharacteristic;
	static std::string m_dataService;
	static std::string m_cubeStateCharacteristic;
	static std::string m_lastMovesCharacteristic;
	static std::string m_timingCharacteristic;
	static std::string m_batteryLevelCharacteristic;
	static uint8_t m_solvedState[18];
	static uint8_t m_keys[2][16];

	void Connected();
	std::vector<uint8_t> Decode(const std::vector<uint8_t>& data);

	void UpdateBatteryState(const std::function<void()>& nextFunc);
	void ReadCubeState(const std::function<void(const Cube3x3& cube)>& resultFunc);
	void ResetCubeState(const std::function<void()>& nextFunc);
	void ReadLastMoveData(const std::function<void(const GANCubeLastMoveData& data)>& resultFunc);

public:
	GANCube(BluetoothDevice* dev);

	virtual Cube3x3 GetCubeState() override;
	virtual void ResetToSolved() override;
	virtual bool HasOrientation() override;
	virtual Quaternion GetOrientation() override;
	virtual BatteryState GetBatteryState() override;

	virtual void Update() override;
};

class BluetoothCubeType
{
	static std::vector<BluetoothCubeType*> m_types;

public:
	virtual ~BluetoothCubeType() {}
	virtual std::string GetDeviceNamePrefix() = 0;
	virtual std::shared_ptr<BluetoothCube> Create(BluetoothDevice* dev) = 0;

	static void Init();
	static BluetoothCubeType* GetTypeForName(const std::string& deviceName);
};

class GANCubeType: public BluetoothCubeType
{
public:
	virtual std::string GetDeviceNamePrefix() override { return "GAN"; }
	virtual std::shared_ptr<BluetoothCube> Create(BluetoothDevice* dev) override;
};
