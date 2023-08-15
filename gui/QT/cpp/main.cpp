#include <QtGui/QGuiApplication>
#include <QtQml/QQmlApplicationEngine>
#include <QQmlContext>
#include "homemodel.h"
#include "librarymodel.h"
#include "epubmodel.h"
#include <QtWebView/QtWebView>
int main(int argc, char* argv[])
{
    QGuiApplication app(argc, argv);
    QQmlApplicationEngine engine;
    HomeModel homeModel;
    QtWebView::initialize();
    QCoreApplication::setAttribute(Qt::AA_UseSoftwareOpenGL);    engine.rootContext()->setContextProperty("HomeModel", &homeModel);
    qmlRegisterType<LibraryModel>("johandost.LibraryModel", 1, 0, "LibraryModel");
    qmlRegisterType<EpubModel>("johandost.EpubModel", 1, 0, "EpubModel");

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