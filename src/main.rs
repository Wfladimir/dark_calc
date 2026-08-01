slint::slint! {
    export component MainWindow inherits Window {
        width: 360px;
        height: 640px;
        title: "Dark Calc";
        background: #0a0a0a;

        VerticalLayout {
            padding: 20px;
            spacing: 15px;

            // Экран калькулятора (полированный чёрный монолит)
            Rectangle {
                height: 80px;
                background: #121212;
                border-radius: 4px;
                border-width: 1px;
                border-color: #262626;

                Text {
                    text: "0";
                    color: #e5e5e5;
                    font-size: 28px;
                    horizontal-alignment: right;
                    vertical-alignment: center;
                }
            }

            // Сетка кнопок будет здесь
        }
    }
}

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
