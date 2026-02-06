use tauri_plugin_notification::NotificationExt;
use tauri::AppHandle;

pub fn notify_hard_pause(app: &AppHandle, label: &str, time_to_return: &str) {
    let _ = app.notification()
        .builder()
        .title("Pausa Obrigatória")
        .body(&format!("Pausa para: {}, retorno em: {}", label, time_to_return))
        .show();
}

pub fn notify_soft_pause(app: &AppHandle, label: &str, time_to_return: &str) {
    let _ = app.notification()
        .builder()
        .title("Pausa Leve")
        .body(&format!("Pausa leve: {}, retorno em: {}", label, time_to_return))
        .show();
}

pub fn notify_finished(app: &AppHandle) {
    let _ = app.notification()
        .builder()
        .title("Dia Concluído")
        .body("Parabéns! Você concluiu seu dia de trabalho.")
        .show();
}

pub fn notify_back_to_work(app: &AppHandle) {
    let _ = app.notification()
        .builder()
        .title("Fim da Pausa")
        .body("Hora de voltar ao trabalho!")
        .show();
}