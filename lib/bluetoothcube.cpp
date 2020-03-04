#ifdef __APPLE__
#include <CommonCrypto/CommonCryptor.h>
#else
#include <cryptopp/aes.h>
#include <cryptopp/modes.h>
#endif
#include <math.h>
#include "bluetoothcube.h"

using namespace std;


vector<BluetoothCubeType*> BluetoothCubeType::m_types;

string GANCube::m_deviceInfoService = "{0000180a-0000-1000-8000-00805f9b34fb}";
string GANCube::m_versionCharacteristic = "{00002a28-0000-1000-8000-00805f9b34fb}";
string GANCube::m_hardwareCharacteristic = "{00002a23-0000-1000-8000-00805f9b34fb}";
string GANCube::m_dataService = "{0000fff0-0000-1000-8000-00805f9b34fb}";
string GANCube::m_cubeStateCharacteristic = "{0000fff2-0000-1000-8000-00805f9b34fb}";
string GANCube::m_lastMovesCharacteristic = "{0000fff5-0000-1000-8000-00805f9b34fb}";
string GANCube::m_timingCharacteristic = "{0000fff6-0000-1000-8000-00805f9b34fb}";
string GANCube::m_batteryLevelCharacteristic = "{0000fff7-0000-1000-8000-00805f9b34fb}";
uint8_t GANCube::m_solvedState[18] = {
	0x00, 0x00, 0x24, 0x00, 0x49, 0x92, 0x24, 0x49, 0x6d, 0x92, 0xdb, 0xb6, 0x49, 0x92, 0xb6, 0x24, 0x6d, 0xdb};
uint8_t GANCube::m_keys[2][16] = {
	{0xc6, 0xca, 0x15, 0xdf, 0x4f, 0x6e, 0x13, 0xb6, 0x77, 0x0d, 0xe6, 0x59, 0x3a, 0xaf, 0xba, 0xa2},
	{0x43, 0xe2, 0x5b, 0xd6, 0x7d, 0xdc, 0x78, 0xd8, 0x07, 0x60, 0xa3, 0xda, 0x82, 0x3c, 0x01, 0xf1}};


void BluetoothDevice::SetConnectedCallback(const std::function<void()>& connectedFunc)
{
	m_connectedFunc = connectedFunc;
}


void BluetoothDevice::SetErrorCallback(const function<void(const string& msg)>& errorHandler)
{
	m_errorHandler = errorHandler;
}


void BluetoothDevice::Connect()
{
	m_connectedFunc();
}


void BluetoothDevice::Error(const string& msg)
{
	if (m_errorHandler)
		m_errorHandler(msg);
}


void BluetoothCubeClient::AddMove(TimedCubeMove move)
{
	m_moves.moves.push_back(move);
}


void BluetoothCubeClient::Error(const string& msg)
{
	if (m_errorHandler)
		m_errorHandler(msg);
}


void BluetoothCubeClient::SetErrorCallback(const function<void(const string& msg)>& errorHandler)
{
	m_errorHandler = errorHandler;
}


TimedCubeMoveSequence BluetoothCubeClient::GetLatestMoves()
{
	TimedCubeMoveSequence result = m_moves;
	m_moves.moves.clear();
	return result;
}


BluetoothCube::BluetoothCube(BluetoothDevice* dev): m_dev(dev)
{
	dev->SetErrorCallback([this](const string& msg) {
		vector<shared_ptr<BluetoothCubeClient>> clients = m_clients;
		for (auto& i : clients)
			i->Error(msg);
	});
}


BluetoothCube::~BluetoothCube()
{
	delete m_dev;
}


void BluetoothCube::SetReadyCallback(const function<void()>& readyFunc)
{
	m_readyFunc = readyFunc;
}


void BluetoothCube::Ready()
{
	m_readyFunc();
}


void BluetoothCube::AddClient(const shared_ptr<BluetoothCubeClient>& client)
{
	m_clients.push_back(client);
}


void BluetoothCube::RemoveClient(const shared_ptr<BluetoothCubeClient>& client)
{
	for (auto i = m_clients.begin(); i != m_clients.end(); ++i)
	{
		if (*i == client)
		{
			m_clients.erase(i);
			break;
		}
	}
}


void BluetoothCube::AddMove(TimedCubeMove move)
{
	for (auto& i : m_clients)
		i->AddMove(move);
}


GANCube::GANCube(BluetoothDevice* dev): BluetoothCube(dev)
{
	m_dev->SetConnectedCallback([this]() { Connected(); });
	m_dev->SetDecoder([this](const vector<uint8_t>& data) { return Decode(data); });
}


vector<uint8_t> GANCube::Decode(const vector<uint8_t>& data)
{
	vector<uint8_t> result = data;
	if (result.size() < 16)
		return result;

#ifdef __APPLE__
	uint8_t output[16];
	size_t outLen;
	if (result.size() > 16)
	{
		CCCrypt(kCCDecrypt, kCCAlgorithmAES, kCCOptionECBMode, m_deviceKey, 16, nullptr,
			&result[result.size() - 16], 16, output, 16, &outLen);
		memcpy(&result[result.size() - 16], output, 16);
	}
	CCCrypt(kCCDecrypt, kCCAlgorithmAES, kCCOptionECBMode, m_deviceKey, 16, nullptr,
		&result[0], 16, output, 16, &outLen);
	memcpy(&result[0], output, 16);
#else
	CryptoPP::ECB_Mode<CryptoPP::AES>::Decryption d;
	d.SetKey(m_deviceKey, 16);
	uint8_t output[16];
	if (result.size() > 16)
	{
		d.ProcessData(output, &result[result.size() - 16], 16);
		memcpy(&result[result.size() - 16], output, 16);
	}
	d.ProcessData(output, &result[0], 16);
	memcpy(&result[0], output, 16);
#endif

	return result;
}


void GANCube::UpdateBatteryState(const function<void()>& nextFunc)
{
	m_dev->ReadEncodedCharacteristic(m_batteryLevelCharacteristic,
		[=](const vector<uint8_t>& data) {
			if (data.size() < 8)
			{
				m_dev->Error("Invalid battery state data");
				return;
			}

			m_battery.percent = data[7];
			m_battery.charging = data[6] != 0;

			char state[64];
			sprintf(state, "Battery at %d%%, %scharging", m_battery.percent,
				m_battery.charging ? "" : "not ");
			m_dev->DebugMessage(state);

			nextFunc();
		});
}


void GANCube::ReadCubeState(const std::function<void(const Cube3x3& cube)>& resultFunc)
{
	m_dev->ReadEncodedCharacteristic(m_cubeStateCharacteristic,
		[=](const vector<uint8_t>& data) {
			if (data.size() < 18)
			{
				m_dev->Error("Invalid cube state data");
				return;
			}

			// Decode cube data (stored in face color format, 3 bits per color)
			Cube3x3Faces cube;
			for (int faceIdx = 0; faceIdx < 6; faceIdx++)
			{
				uint32_t faceData = ((uint32_t)data[(faceIdx * 3) ^ 1] << 16) |
					((uint32_t)data[((faceIdx * 3) + 1) ^ 1] << 8) |
					(uint32_t)data[((faceIdx * 3) + 2) ^ 1];
				static CubeFace faceMap[6] = {TOP, RIGHT, FRONT, BOTTOM, LEFT, BACK};
				CubeFace face = faceMap[faceIdx];
				for (int row = 0; row < 3; row++)
				{
					for (int col = 0; col < 3; col++)
					{
						if ((row == 1) && (col == 1))
							continue;
						int i = (row * 3) + col;
						if (i >= 4)
							i--;
						i = 7 - i;
						uint32_t colorIndex = (faceData >> (3 * i)) & 7;
						static CubeColor colorMap[8] = {WHITE, RED, GREEN, YELLOW,
							ORANGE, BLUE, WHITE, WHITE};
						cube.SetColor(face, row, col, colorMap[colorIndex]);
					}
				}
			}

			resultFunc(Cube3x3(cube));
		});
}


void GANCube::ResetCubeState(const function<void()>& nextFunc)
{
	vector<uint8_t> state;
	for (size_t i = 0; i < 18; i++)
		state.push_back(m_solvedState[i]);
	m_dev->WriteCharacteristic(m_cubeStateCharacteristic, state, [=]() {
		nextFunc();
	});
}


void GANCube::ReadLastMoveData(const function<void(const GANCubeLastMoveData& data)>& resultFunc)
{
	m_dev->ReadEncodedCharacteristic(m_lastMovesCharacteristic,	[=](const std::vector<uint8_t>& data) {
		if (data.size() != 19)
		{
			m_dev->Error("Invalid last move data");
			return;
		}

		GANCubeLastMoveData decoded;
		memcpy(&decoded, &data[0], 19);
		resultFunc(decoded);
	});
}


void GANCube::Connected()
{
	// Connect to the device information service
	m_dev->ConnectToService(m_deviceInfoService,
		[this]() {
			// First read the protocol version information
			m_dev->ReadCharacteristic(m_versionCharacteristic,
				[this](const vector<uint8_t>& data) {
					if (data.size() < 3)
					{
						m_dev->Error("Invalid version data");
						return;
					}

					// Decode version information (1.0 and 1.1 supported)
					uint32_t majorVersion = data[0];
					uint32_t minorVersion = data[1];
					uint32_t revision = data[2];
					char version[64];
					sprintf(version, "%d.%d.%d", majorVersion, minorVersion, revision);
					m_dev->DebugMessage(string("GAN cube protocol version ") + string(version) + string(" connected"));

					if ((majorVersion != 1) || (minorVersion > 1))
					{
						char version[64];
						sprintf(version, "GAN cube protocol version %d.%d.%d not supported",
							majorVersion, minorVersion, revision);
						m_dev->Error(string("GAN cube version ") + string(version) + string(" not supported"));
						return;
					}

					// Read the hardware information, this is mixed with the key material
					m_dev->ReadCharacteristic(m_hardwareCharacteristic,
						[=](const vector<uint8_t>& data) {
							if (data.size() < 6)
							{
								m_dev->Error("Invalid hardware data");
								return;
							}

							// Calculate device key
							for (size_t i = 0; i < 6; i++)
								m_deviceKey[i] = m_keys[minorVersion][i] + data[5 - i];
							for (size_t i = 6; i < 16; i++)
								m_deviceKey[i] = m_keys[minorVersion][i];

							// Connect to the data service to begin reading cube state
							m_dev->ConnectToService(m_dataService, [this]() {
								// Get the initial battery status
								UpdateBatteryState([this]() {
									m_lastBatteryUpdateTime = chrono::steady_clock::now();

									// Get the initial cube state
									ReadCubeState([this](Cube3x3 cube) {
										m_cube = cube;
										// Read one move data poll to get the initial move count
										ReadLastMoveData([this](const GANCubeLastMoveData& data) {
											m_lastMoveCount = data.moveCount;
											Ready();
										});
									});
								});
							});
						});
				});
		});
}


Cube3x3 GANCube::GetCubeState()
{
	return m_cube;
}


void GANCube::ResetToSolved()
{
	m_resetRequested = true;
	m_cube = Cube3x3();
}


BatteryState GANCube::GetBatteryState()
{
	return m_battery;
}


void GANCube::Update()
{
	if (m_updateInProgress)
		return;

	m_updateInProgress = true;

	if (m_resetRequested)
	{
		m_resetRequested = false;
		ResetCubeState([this]() {
			m_cube = Cube3x3();
			m_updateInProgress = false;
		});
		return;
	}

	if (chrono::duration_cast<chrono::seconds>(chrono::steady_clock::now() - m_lastBatteryUpdateTime).count() >= 5)
	{
		UpdateBatteryState([this]() {
			m_lastBatteryUpdateTime = chrono::steady_clock::now();
			m_updateInProgress = false;
		});
		return;
	}

	ReadLastMoveData([this](const GANCubeLastMoveData& lastMove) {
			if (lastMove.moveCount == m_lastMoveCount)
			{
				m_updateInProgress = false;
				return;
			}

			m_dev->ReadEncodedCharacteristic(m_timingCharacteristic, [=](const std::vector<uint8_t>& data) {
					if (data.size() < 19)
					{
						m_dev->Error("Invalid timestamp data");
						return;
					}

					// Determine how many moves have taken place since the last poll
					uint8_t moves = lastMove.moveCount - m_lastMoveCount;
					if (moves > 6)
					{
						m_dev->Error("Previous move buffer limit exceeded");
						return;
					}

					chrono::time_point<chrono::steady_clock> curTime = chrono::steady_clock::now();
					bool useMoveTimes = true;
					if ((!m_firstMove) && (chrono::duration_cast<chrono::seconds>(curTime - m_lastMoveTime).count() > 30))
					{
						// More than 30 seconds between moves, don't adjust clock ratio to avoid issues
						// with the range of the encoding. Adjust timestamp using real time.
						useMoveTimes = false;
						m_currentTimestamp += (uint64_t)chrono::duration_cast<chrono::milliseconds>(
							curTime - m_lastMoveTime).count();
					}

					// Decode move data
					uint64_t newTicks = 0;
					for (uint8_t i = 0; i < moves; i++)
					{
						static const CubeMove moveTable[18] = {
							MOVE_U, MOVE_U2, MOVE_Up, MOVE_R, MOVE_R2, MOVE_Rp,
							MOVE_F, MOVE_F2, MOVE_Fp, MOVE_D, MOVE_D2, MOVE_Dp,
							MOVE_L, MOVE_L2, MOVE_Lp, MOVE_B, MOVE_B2, MOVE_Bp
						};
						uint8_t move = lastMove.move[(6 - moves) + i];
						if (move >= 18)
						{
							m_dev->Error("Previous move buffer has invalid move");
							return;
						}

						// Calculate the GAN cube's view of the timestamp of this move
						uint8_t timestampIndex = (m_lastMoveCount + i) - (data[0] - 9);
						if (timestampIndex >= 9)
						{
							m_dev->Error("Timestamp for move is not present in buffer");
							return;
						}
						uint16_t timeSinceLastMove = (uint16_t)data[(timestampIndex * 2) + 1] |
							(((uint16_t)data[(timestampIndex * 2) + 2]) << 8);

						if (useMoveTimes)
						{
							newTicks += timeSinceLastMove;
							m_currentTimestamp += (uint64_t)((float)timeSinceLastMove / m_clockRatio);
						}

						AddMove(TimedCubeMove { moveTable[move], m_currentTimestamp });
						m_cube.Move(moveTable[move]);
					}

					// The GAN cubes have wildly variable clock rates so we need to calibrate
					// it to the host system timer (which will be far more accurate). The ratio
					// will converge over time on the actual ratio and eventually give an
					// accurate move time.
					if (m_firstMove)
					{
						m_startTime = chrono::steady_clock::now();
						m_firstMove = false;
						m_clockRatio = 0.95f;
					}
					else if (useMoveTimes)
					{
						m_totalCubeTicks += newTicks;
						m_lastRealTicks = (uint64_t)chrono::duration_cast<chrono::milliseconds>(
							chrono::steady_clock::now() - m_startTime).count() + m_baseRealTicks;
						m_clockRatio = (double)m_totalCubeTicks / (double)m_lastRealTicks;

						// Sanity check ratio and don't let it get out of reasonable range
						if (m_clockRatio < 0.9f)
							m_clockRatio = 0.9f;
						else if (m_clockRatio > 1.0f)
							m_clockRatio = 1.0f;
					}
					else
					{
						// Not using move times for this set of moves, reset real time tracking to
						// skip this set of moves
						m_baseRealTicks = m_lastRealTicks;
						m_startTime = curTime;
					}

					m_lastMoveCount = lastMove.moveCount;
					m_lastMoveTime = curTime;

					m_updateInProgress = false;
				});
		});
}


void BluetoothCubeType::Init()
{
	m_types.push_back(new GANCubeType());
}


BluetoothCubeType* BluetoothCubeType::GetTypeForName(const string& deviceName)
{
	for (auto i : m_types)
	{
		string prefix = i->GetDeviceNamePrefix();
		if ((deviceName.size() >= prefix.size()) &&
			(deviceName.substr(0, prefix.size()) == prefix))
			return i;
	}
	return nullptr;
}


shared_ptr<BluetoothCube> GANCubeType::Create(BluetoothDevice* dev)
{
	return make_shared<GANCube>(dev);
}
