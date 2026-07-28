use std::{fs, io::Write};

slint::include_modules!();

fn main() -> anyhow::Result<()> {
    let app = MainWindow::new()?;

    app.on_convert(|input, output| {
        let document = match office_oxide::Document::open(input) {
            Ok(doc) => doc,
            Err(_) => return AppError::NoInputFile,
        };

        let markdown = document.to_markdown();

        let mut file = match fs::File::create(output) {
            Ok(file) => file,
            Err(_) => return AppError::NoOutputFile,
        };

        match file.write_all(&markdown.into_bytes()) {
            Ok(_) => AppError::None,
            Err(_) => AppError::FailedConversion,
        }
    });

    app.on_open_input({
        let app = app.as_weak();
        move || {
            if let Some(app) = app.upgrade() {
                slint::spawn_local(async move {
                    if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                        let path = match file.path().to_str() {
                            None => {
                                app.set_error(AppError::NoInputFile);
                                return;
                            }
                            Some(path) => path,
                        };
                        app.set_input(path.into());
                    };
                })
                .unwrap();
            }
        }
    });

    app.on_open_output({
        let app = app.as_weak();
        move || {
            if let Some(app) = app.upgrade() {
                slint::spawn_local(async move {
                    if let Some(file) = rfd::AsyncFileDialog::new().save_file().await {
                        let path = match file.path().to_str() {
                            None => {
                                app.set_error(AppError::NoOutputFile);
                                return;
                            }
                            Some(path) => path,
                        };
                        app.set_output(path.into());
                    };
                })
                .unwrap();
            }
        }
    });

    app.run()?;

    Ok(())
}
