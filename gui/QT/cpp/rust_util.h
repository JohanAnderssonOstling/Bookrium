//
// Created by johan on 2023-05-22.
//

#pragma once

#include <QString>
#include "rust/cxx.h"
static QString asQStr(const rust::String& s) {
    return QString::fromUtf8(s.data(), s.size());
}

static rust::String asRustStr(const QString& qstr) {
    return rust::String(qstr.toUtf8().constData(), qstr.toUtf8().size());
}