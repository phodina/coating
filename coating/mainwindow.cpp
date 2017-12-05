#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "coating.h"
#include <QDebug>

MainWindow::MainWindow(QWidget *parent) :
    QMainWindow(parent),
    ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    msg = new_msg();

    qDebug () << "Rust says: " << msg;
}

MainWindow::~MainWindow()
{
    delete ui;
    free_msg(msg);
}
