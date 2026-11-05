use crate::logger::level::LogLevel;
use std::io::Error;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

pub struct LoggerHandle {
    file: Option<File>,
    path: String,
}

impl LoggerHandle {
    pub async fn new(path: String) -> Self {
        let result = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .await;
        match result {
            Ok(file) => Self {
                file: Some(file),
                path,
            },
            Err(err) => {
                panic!("无法创建Logger句柄: {}", err);
            }
        }
    }

    async fn fix_log(
        &mut self,
        log_string: String,
        log_level: LogLevel,
        time_str: String,
        err: Error,
    ) {
        let result = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await;

        match result {
            Ok(mut file) => {
                let log_str = format!("{} [{}] {}\n", time_str, log_level, log_string);
                let recovery_msg = format!(
                    "logger_handle炸了,尝试恢复,并写回原log:[{}]\n{}",
                    err, log_str
                );
                match file.write_all(recovery_msg.as_bytes()).await {
                    Ok(_) => {
                        self.file = Some(file);
                    }
                    Err(e) => {
                        eprintln!("logger_handle恢复失败[{}], logger下次启动恢复", e);
                        self.file = None;
                    }
                }
            }
            Err(err) => {
                eprintln!("logger_handle恢复失败[{}], logger下次启动恢复", err);
                self.file = None;
            }
        }
    }

    async fn log(&mut self, log_string: String, log_level: LogLevel) {
        let time_str = super::utils::get_new_time();
        let log_str = format!("{} [{}] {}\n", time_str, log_level, log_string);
        match &mut self.file {
            Some(log) => match log.write_all(log_str.as_bytes()).await {
                Ok(_) => {
                    let result = log.flush().await;
                    if let Err(err) = result {
                        eprintln!("log flush失败[{}]", err);
                    }
                }
                Err(err) => {
                    self.fix_log(log_string, log_level, time_str, err).await;
                }
            },
            None => {
                eprintln!("[logger 已失效] {}", log_str);
            }
        }
    }

    pub async fn info(&mut self, log_string: String) {
        self.log(log_string, LogLevel::Info).await;
    }

    pub async fn warn(&mut self, log_string: String) {
        self.log(log_string, LogLevel::Warn).await;
    }

    pub async fn error(&mut self, log_string: String) {
        self.log(log_string, LogLevel::Error).await;
    }

    pub async fn read_all_logs(&mut self) -> Option<Vec<String>> {
        match tokio::fs::read_to_string(&self.path).await {
            Ok(buffer) => Some(buffer.lines().map(|l| l.to_string()).collect()),
            Err(err) => {
                self.error(format!("读取log失败[{}]", err)).await;
                None
            }
        }
    }

    pub async fn read_lines(&mut self, from: usize, to: usize) -> Option<Vec<String>> {
        match tokio::fs::read_to_string(&self.path).await {
            Ok(buffer) => {
                let lines: Vec<String> = buffer
                    .lines()
                    .skip(from)
                    .take(to.saturating_sub(from))
                    .map(|l| l.to_string())
                    .collect();
                Some(lines)
            }
            Err(err) => {
                self.error(format!("读取log失败[{}]", err)).await;
                None
            }
        }
    }
}
