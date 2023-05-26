#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QQmlContext>
#include "homemodel.h"
#include "librarymodel.h"
int main(int argc, char* argv[])
{
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;
    HomeModel homeModel;
    engine.rootContext()->setContextProperty("HomeModel", &homeModel);
    qmlRegisterType<LibraryModel>("johandost.LibraryModel", 1, 0, "LibraryModel");

    const QUrl url(QStringLiteral("../qml/main.qml"));
    QObject::connect(&engine,&QQmlApplicationEngine::objectCreated,&app,
            [url](QObject* obj, const QUrl& objUrl) {
                if (!obj && url == objUrl)
                    QCoreApplication::exit(-1);
            },
            Qt::QueuedConnection);



    engine.load(url);

    return app.exec();
}