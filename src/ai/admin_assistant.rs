//! Admin AI Assistant - Natural language administrative commands
//! 
//! Provides AI-powered administrative interface with natural language processing.
//! Supports commands for backend control, metrics viewing, system status, etc.

use crate::metrics::MetricsCollector;
use crate::orchestration::BackendOrchestrator;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Admin-specific intents for system management
#[derive(Debug, Clone, PartialEq)]
pub enum AdminIntent {
    /// Backend control: "запусти backend", "останови сервер"
    BackendControl(BackendAction),
    
    /// System status: "статус системы", "как дела?"
    SystemStatus,
    
    /// Metrics query: "покажи метрики", "статистика интентов"
    MetricsQuery(MetricsType),
    
    /// Connection info: "сколько пользователей онлайн"
    ConnectionInfo,
    
    /// AI health: "как работает AI", "проверь AI engine"
    AIHealth,
    
    /// Logs request: "покажи последние ошибки"
    LogsRequest(LogLevel),
    
    /// Performance: "производительность системы"
    Performance,
    
    /// Unknown admin command
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BackendAction {
    Start,
    Stop,
    Restart,
    Status,
    Health,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetricsType {
    Intents,
    ResponseTimes,
    All,
    Dashboard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Errors,
    Warnings,
    All,
}

/// Admin AI Assistant
pub struct AdminAssistant {
    orchestrator: Option<Arc<BackendOrchestrator>>,
    metrics: Arc<RwLock<MetricsCollector>>,
}

impl AdminAssistant {
    /// Create new admin assistant
    pub fn new(
        orchestrator: Option<Arc<BackendOrchestrator>>,
        metrics: Arc<RwLock<MetricsCollector>>,
    ) -> Self {
        Self {
            orchestrator,
            metrics,
        }
    }

    /// Detect admin intent from natural language
    pub fn detect_admin_intent(&self, message: &str) -> AdminIntent {
        let msg = message.to_lowercase();

        // Backend control commands - check more specific first
        if msg.contains("перезапусти") || msg.contains("рестарт") {
            return AdminIntent::BackendControl(BackendAction::Restart);
        }
        if msg.contains("запусти") && (msg.contains("backend") || msg.contains("сервер") || msg.contains("бэкенд")) {
            return AdminIntent::BackendControl(BackendAction::Start);
        }
        if msg.contains("останови") || msg.contains("выключи") {
            return AdminIntent::BackendControl(BackendAction::Stop);
        }
        if msg.contains("статус") && (msg.contains("backend") || msg.contains("сервер") || msg.contains("бэкенд")) {
            return AdminIntent::BackendControl(BackendAction::Status);
        }

        // System status
        if (msg.contains("статус") && msg.contains("систем")) 
            || msg.contains("как дела")
            || msg.contains("все ок")
            || msg.contains("health check") {
            return AdminIntent::SystemStatus;
        }

        // Metrics queries - check more specific first
        if msg.contains("метрик") || msg.contains("статистик") || msg.contains("аналитик") {
            if msg.contains("интент") {
                return AdminIntent::MetricsQuery(MetricsType::Intents);
            }
            if msg.contains("время") || msg.contains("производительность") {
                return AdminIntent::MetricsQuery(MetricsType::ResponseTimes);
            }
            if msg.contains("все") || msg.contains("полн") {
                return AdminIntent::MetricsQuery(MetricsType::All);
            }
            return AdminIntent::MetricsQuery(MetricsType::Dashboard);
        }

        // Connection info
        if (msg.contains("сколько") || msg.contains("количество")) 
            && (msg.contains("пользовател") || msg.contains("подключен") || msg.contains("онлайн")) {
            return AdminIntent::ConnectionInfo;
        }

        // AI health
        if msg.contains("ai") || msg.contains("аи") {
            if msg.contains("работа") || msg.contains("здоровь") || msg.contains("проверь") {
                return AdminIntent::AIHealth;
            }
        }

        // Logs
        if msg.contains("лог") || msg.contains("ошибк") || msg.contains("error") {
            if msg.contains("предупрежден") || msg.contains("warning") {
                return AdminIntent::LogsRequest(LogLevel::Warnings);
            }
            if msg.contains("ошибк") || msg.contains("error") {
                return AdminIntent::LogsRequest(LogLevel::Errors);
            }
            return AdminIntent::LogsRequest(LogLevel::All);
        }

        // Performance
        if msg.contains("производительность") || msg.contains("performance") {
            return AdminIntent::Performance;
        }

        AdminIntent::Unknown(message.to_string())
    }

    /// Process admin command and generate response
    pub async fn process_admin_command(&self, intent: AdminIntent) -> String {
        match intent {
            AdminIntent::BackendControl(action) => self.handle_backend_control(action).await,
            AdminIntent::SystemStatus => self.handle_system_status().await,
            AdminIntent::MetricsQuery(metrics_type) => self.handle_metrics_query(metrics_type).await,
            AdminIntent::ConnectionInfo => self.handle_connection_info().await,
            AdminIntent::AIHealth => self.handle_ai_health().await,
            AdminIntent::LogsRequest(level) => self.handle_logs_request(level).await,
            AdminIntent::Performance => self.handle_performance().await,
            AdminIntent::Unknown(msg) => {
                format!("❓ Не понял команду: \"{}\"\n\n🔧 Доступные команды:\n• запусти/останови/перезапусти backend\n• статус системы\n• покажи метрики\n• сколько пользователей онлайн\n• проверь AI engine\n• покажи ошибки", msg)
            }
        }
    }

    async fn handle_backend_control(&self, action: BackendAction) -> String {
        if self.orchestrator.is_none() {
            return "⚠️ Backend orchestrator не настроен. Включите его в конфигурации.".to_string();
        }

        let orchestrator = self.orchestrator.as_ref().unwrap();

        match action {
            BackendAction::Start => {
                match orchestrator.start().await {
                    Ok(()) => {
                        let info = orchestrator.get_info().await;
                        format!("✅ **Backend запущен успешно**\n\n📊 Информация:\n• PID: {}\n• Статус: {:?}\n• Время работы: {} сек", 
                            info.pid.map(|p| p.to_string()).unwrap_or("N/A".to_string()),
                            info.status,
                            info.uptime_secs.map(|u| u.to_string()).unwrap_or("N/A".to_string()))
                    }
                    Err(e) => format!("❌ Ошибка запуска backend: {}", e),
                }
            }
            BackendAction::Stop => {
                match orchestrator.stop().await {
                    Ok(()) => "✅ **Backend остановлен**".to_string(),
                    Err(e) => format!("❌ Ошибка остановки backend: {}", e),
                }
            }
            BackendAction::Restart => {
                match orchestrator.restart().await {
                    Ok(()) => {
                        let info = orchestrator.get_info().await;
                        format!("🔄 **Backend перезапущен**\n\n📊 Информация:\n• PID: {}\n• Количество рестартов: {}", 
                            info.pid.map(|p| p.to_string()).unwrap_or("N/A".to_string()),
                            info.restart_count)
                    }
                    Err(e) => format!("❌ Ошибка перезапуска backend: {}", e),
                }
            }
            BackendAction::Status => {
                let info = orchestrator.get_info().await;
                let is_running = orchestrator.is_running().await;
                format!("📊 **Статус Backend**\n\n• Статус: {:?}\n• PID: {}\n• Работает: {}\n• Время работы: {} сек\n• Рестартов: {}\n• Последняя проверка: {}", 
                    info.status,
                    info.pid.map(|p| p.to_string()).unwrap_or("N/A".to_string()),
                    if is_running { "✅ Да" } else { "❌ Нет" },
                    info.uptime_secs.map(|u| u.to_string()).unwrap_or("N/A".to_string()),
                    info.restart_count,
                    info.last_health_check.as_ref().map(|s| s.as_str()).unwrap_or("Никогда"))
            }
            BackendAction::Health => {
                let is_running = orchestrator.is_running().await;
                if is_running {
                    "✅ **Backend здоров**\n\nВсе системы работают нормально.".to_string()
                } else {
                    "⚠️ **Backend не работает**\n\nВозможно требуется перезапуск.".to_string()
                }
            }
        }
    }

    async fn handle_system_status(&self) -> String {
        let metrics = self.metrics.read().await;
        let stats = metrics.get_stats();
        
        let backend_status = if let Some(orchestrator) = &self.orchestrator {
            let info = orchestrator.get_info().await;
            format!("✅ Работает (PID: {})", info.pid.map(|p| p.to_string()).unwrap_or("N/A".to_string()))
        } else {
            "⚠️ Не настроен".to_string()
        };

        format!(
            "🖥️ **Статус системы FodiFood Bot v2.2**\n\n\
            📡 **AI Engine**\n\
            • Обработано интентов: {}\n\
            • Активных подключений: {}\n\
            • Среднее время ответа: {:.2} сек\n\n\
            🎯 **Backend Orchestrator**\n\
            • Статус: {}\n\n\
            📊 **Метрики**\n\
            • Успешных ответов: {}%\n\
            • Всего запросов: {}\n\n\
            ✅ **Все системы работают нормально**",
            stats.total_intents,
            stats.active_connections,
            stats.avg_response_time,
            backend_status,
            if stats.total_intents > 0 {
                ((stats.total_intents as f64 - stats.failed_intents as f64) / stats.total_intents as f64 * 100.0).round() as u64
            } else {
                100
            },
            stats.total_intents
        )
    }

    async fn handle_metrics_query(&self, metrics_type: MetricsType) -> String {
        let metrics = self.metrics.read().await;
        
        match metrics_type {
            MetricsType::Intents => {
                let by_type = &metrics.get_stats().intents_by_type;
                let mut response = "📊 **Статистика интентов**\n\n".to_string();
                
                let mut intents: Vec<_> = by_type.iter().collect();
                intents.sort_by(|a, b| b.1.cmp(a.1));
                
                for (intent, count) in intents.iter().take(10) {
                    response.push_str(&format!("• {}: {} запросов\n", intent, count));
                }
                
                response
            }
            MetricsType::ResponseTimes => {
                let stats = metrics.get_stats();
                format!(
                    "⏱️ **Производительность AI**\n\n\
                    • Среднее время ответа: {:.2} сек\n\
                    • Мин: {:.2} сек\n\
                    • Макс: {:.2} сек\n\
                    • Всего запросов: {}",
                    stats.avg_response_time,
                    stats.min_response_time,
                    stats.max_response_time,
                    stats.total_intents
                )
            }
            MetricsType::All | MetricsType::Dashboard => {
                let stats = metrics.get_stats();
                format!(
                    "📊 **Полная статистика системы**\n\n\
                    **Интенты:**\n\
                    • Всего обработано: {}\n\
                    • Неудачных: {}\n\
                    • Успешных: {}%\n\n\
                    **Производительность:**\n\
                    • Среднее время: {:.2} сек\n\
                    • Мин/Макс: {:.2}/{:.2} сек\n\n\
                    **Подключения:**\n\
                    • Активных: {}\n\
                    • Всего за сессию: {}",
                    stats.total_intents,
                    stats.failed_intents,
                    if stats.total_intents > 0 {
                        ((stats.total_intents as f64 - stats.failed_intents as f64) / stats.total_intents as f64 * 100.0).round() as u64
                    } else {
                        100
                    },
                    stats.avg_response_time,
                    stats.min_response_time,
                    stats.max_response_time,
                    stats.active_connections,
                    stats.total_connections
                )
            }
        }
    }

    async fn handle_connection_info(&self) -> String {
        let metrics = self.metrics.read().await;
        let stats = metrics.get_stats();
        
        format!(
            "👥 **Информация о подключениях**\n\n\
            • Активных сейчас: {} пользователей\n\
            • Всего за сессию: {} подключений\n\
            • Средняя активность: {:.1} запросов/пользователь",
            stats.active_connections,
            stats.total_connections,
            if stats.total_connections > 0 {
                stats.total_intents as f64 / stats.total_connections as f64
            } else {
                0.0
            }
        )
    }

    async fn handle_ai_health(&self) -> String {
        let metrics = self.metrics.read().await;
        let stats = metrics.get_stats();
        
        let error_rate = if stats.total_intents > 0 {
            stats.failed_intents as f64 / stats.total_intents as f64
        } else {
            0.0
        };
        
        let health_status = if error_rate < 0.1 {
            "✅ Отлично"
        } else if error_rate < 0.3 {
            "⚠️ Приемлемо"
        } else {
            "❌ Требует внимания"
        };

        format!(
            "🧠 **Статус AI Engine**\n\n\
            • Общее здоровье: {}\n\
            • Обработано интентов: {}\n\
            • Процент успеха: {}%\n\
            • Среднее время обработки: {:.2} сек\n\n\
            💡 **Рекомендации:**\n\
            {}",
            health_status,
            stats.total_intents,
            if stats.total_intents > 0 {
                ((stats.total_intents as f64 - stats.failed_intents as f64) / stats.total_intents as f64 * 100.0).round() as u64
            } else {
                100
            },
            stats.avg_response_time,
            if stats.avg_response_time > 2.0 {
                "• Среднее время ответа высокое, рассмотрите оптимизацию\n"
            } else {
                "• Производительность в норме\n"
            }
        )
    }

    async fn handle_logs_request(&self, _level: LogLevel) -> String {
        "📋 **Логи системы**\n\n⚠️ Для просмотра логов используйте:\n• `cargo shuttle logs` (production)\n• `RUST_LOG=debug cargo run --bin local` (local)\n\n💡 Tip: Логи доступны через Shuttle CLI".to_string()
    }

    async fn handle_performance(&self) -> String {
        let metrics = self.metrics.read().await;
        let stats = metrics.get_stats();
        
        let performance_score = if stats.avg_response_time < 1.0 && stats.failed_intents == 0 {
            "🟢 Отлично (95/100)"
        } else if stats.avg_response_time < 2.0 && (stats.failed_intents as f64 / stats.total_intents.max(1) as f64) < 0.1 {
            "🟡 Хорошо (75/100)"
        } else {
            "🔴 Требует улучшения (50/100)"
        };

        format!(
            "⚡ **Производительность системы**\n\n\
            • Общая оценка: {}\n\n\
            **Метрики времени отклика:**\n\
            • Среднее: {:.2} сек\n\
            • Минимум: {:.2} сек\n\
            • Максимум: {:.2} сек\n\n\
            **Надежность:**\n\
            • Успешных запросов: {}%\n\
            • Неудачных: {}\n\n\
            **Нагрузка:**\n\
            • Всего запросов: {}\n\
            • Активных подключений: {}",
            performance_score,
            stats.avg_response_time,
            stats.min_response_time,
            stats.max_response_time,
            if stats.total_intents > 0 {
                ((stats.total_intents as f64 - stats.failed_intents as f64) / stats.total_intents as f64 * 100.0).round() as u64
            } else {
                100
            },
            stats.failed_intents,
            stats.total_intents,
            stats.active_connections
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_intent_detection() {
        let metrics = Arc::new(RwLock::new(MetricsCollector::new()));
        let assistant = AdminAssistant::new(None, metrics);

        // Backend control
        assert_eq!(
            assistant.detect_admin_intent("запусти backend"),
            AdminIntent::BackendControl(BackendAction::Start)
        );
        assert_eq!(
            assistant.detect_admin_intent("останови сервер"),
            AdminIntent::BackendControl(BackendAction::Stop)
        );
        assert_eq!(
            assistant.detect_admin_intent("перезапусти backend"),
            AdminIntent::BackendControl(BackendAction::Restart)
        );

        // System status
        assert_eq!(
            assistant.detect_admin_intent("статус системы"),
            AdminIntent::SystemStatus
        );
        assert_eq!(
            assistant.detect_admin_intent("как дела?"),
            AdminIntent::SystemStatus
        );

        // Metrics
        assert_eq!(
            assistant.detect_admin_intent("покажи метрики"),
            AdminIntent::MetricsQuery(MetricsType::Dashboard)
        );
        assert_eq!(
            assistant.detect_admin_intent("статистика интентов"),
            AdminIntent::MetricsQuery(MetricsType::Intents)
        );

        // Connection info
        assert_eq!(
            assistant.detect_admin_intent("сколько пользователей онлайн"),
            AdminIntent::ConnectionInfo
        );

        // AI health
        assert_eq!(
            assistant.detect_admin_intent("проверь AI engine"),
            AdminIntent::AIHealth
        );
    }

    #[tokio::test]
    async fn test_system_status_response() {
        let metrics = Arc::new(RwLock::new(MetricsCollector::new()));
        let assistant = AdminAssistant::new(None, metrics.clone());

        let response = assistant.handle_system_status().await;
        assert!(response.contains("Статус системы"));
        assert!(response.contains("AI Engine"));
    }

    #[tokio::test]
    async fn test_metrics_query() {
        let metrics = Arc::new(RwLock::new(MetricsCollector::new()));
        let assistant = AdminAssistant::new(None, metrics.clone());

        let response = assistant.handle_metrics_query(MetricsType::Intents).await;
        assert!(response.contains("Статистика интентов"));
    }
}
