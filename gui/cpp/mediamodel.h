//
// Created by johan on 2023-06-01.
//

#ifndef CASTAFIORE_MEDIAMODEL_H
#define CASTAFIORE_MEDIAMODEL_H


#include <QAbstractListModel>
#include "rust/cxx.h"

class MediaModel : public QAbstractListModel { Q_OBJECT
private:
		rust::String media_uuid;
public: enum Roles {
		NavigationRole = Qt::UserRole,
};
};


#endif //CASTAFIORE_MEDIAMODEL_H
