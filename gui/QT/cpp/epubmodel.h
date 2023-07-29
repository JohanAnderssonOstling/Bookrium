//
// Created by johan on 2023-07-22.
//

#ifndef CASTAFIORE_EPUBMODEL_H
#define CASTAFIORE_EPUBMODEL_H

#endif //CASTAFIORE_EPUBMODEL_H

#include <QtQmlIntegration>
#include "QtCore/QObject"

#include "rust/cxx.h"
#include "cxx_layer/library/src/epub_cxx.rs.h"

class EpubModel : public  QObject{
    Q_OBJECT
public:
  explicit EpubModel(QObject *parent = nullptr);


public slots:
    void openEpub(const QString &path);
    QString getText();
    QString addParagraph();
    QString addPrevParagraph();
    void nextChapter();
    void prevChapter();
    void goTo(QString href);

    void removeParagraph();
    void removePrevParagraph();
    void resetParagraph();
    void nextParagraphs();
    void prevParagraphs();
private:
    rust::String epub_uuid;
};