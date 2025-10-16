// Публичные модули для использования в бинарниках
pub mod config;
pub mod services; // 🌐 External service clients (должен быть ДО ai)
pub mod ai;
pub mod api;
pub mod handlers;
pub mod models;
pub mod orchestration; // 🎯 Backend orchestration
pub mod state;
pub mod metrics;
