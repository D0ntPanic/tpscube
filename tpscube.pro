QT += core gui widgets bluetooth

TARGET = tpscube
TEMPLATE = app
CONFIG += c++14

OBJECTS_DIR = obj
MOC_DIR = obj

RESOURCES += ui/qt/tpscube.qrc

HEADERS += \
    $$files(lib/*.h) \
    $$files(ui/qt/*.h)

SOURCES += \
    $$files(lib/*.cpp) \
    $$files(ui/qt/*.cpp)

SOURCES -= ui/qt/qrc_tpscube.cpp

INCLUDEPATH += lib
INCLUDEPATH += ui/qt
LIBS += -lleveldb

macx {
    QMAKE_CXXFLAGS += -I/usr/local/include
    LIBS += -L/usr/local/lib
}

linux {
    LIBS += -ltomcrypt
}
