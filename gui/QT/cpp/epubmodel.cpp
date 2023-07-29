//
// Created by johan on 2023-07-22.
//

#include "epubmodel.h"
#include "rust_util.h"

EpubModel::EpubModel(QObject *parent) : QObject(parent) {
}

void EpubModel::openEpub(const QString &path) {
  epub_uuid = open_epub(asRustStr("/home/johan/Hem/Downloads/PDFBöcker/[9781732102200] John Ousterhout - A Philosophy of Software Design (0) - libgen.li.epub"));
}

QString EpubModel::getText()      { return asQStr (get_text (epub_uuid));}
QString EpubModel::addParagraph() { return asQStr (add_paragraph ( epub_uuid ) );}
void EpubModel::nextChapter()     { next_chapter(epub_uuid);}
void EpubModel::removeParagraph() { remove_paragraph(epub_uuid);}
void EpubModel::resetParagraph()  { reset_paragraph(epub_uuid);}
void EpubModel::nextParagraphs()  { next_paragraphs(epub_uuid);}
QString EpubModel::addPrevParagraph() { return asQStr(add_prev_paragraph(epub_uuid));}
void EpubModel::prevParagraphs()      { prev_paragraphs(epub_uuid);}
void EpubModel::removePrevParagraph() { remove_prev_paragraph(epub_uuid);}

void EpubModel::prevChapter() {  prev_chapter(epub_uuid); }

void EpubModel::goTo(QString href) {go_to(epub_uuid, asRustStr(href));}

