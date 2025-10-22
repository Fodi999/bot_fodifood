use sqlx::PgPool;
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Analytics metrics operations
pub struct MetricsOps<'a> {
    pool: &'a PgPool,
}

impl<'a> MetricsOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Record a metric
    pub async fn record(
        &self,
        metric_name: &str,
        value: f64,
        labels: Option<serde_json::Value>,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO analytics.metrics (metric_name, value, labels)
             VALUES ($1, $2, $3)
             RETURNING id"
        )
        .bind(metric_name)
        .bind(value)
        .bind(labels)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get metrics by name with time range
    pub async fn get_metrics(
        &self,
        metric_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<Metric>> {
        let metrics = sqlx::query_as::<_, Metric>(
            "SELECT id, metric_name, value, labels, recorded_at
             FROM analytics.metrics
             WHERE metric_name = $1 AND recorded_at BETWEEN $2 AND $3
             ORDER BY recorded_at DESC"
        )
        .bind(metric_name)
        .bind(from)
        .bind(to)
        .fetch_all(self.pool)
        .await?;
        
        Ok(metrics)
    }
    
    /// Get average metric value
    pub async fn get_average(
        &self,
        metric_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<f64> {
        let result: (Option<f64>,) = sqlx::query_as(
            "SELECT AVG(value) FROM analytics.metrics
             WHERE metric_name = $1 AND recorded_at BETWEEN $2 AND $3"
        )
        .bind(metric_name)
        .bind(from)
        .bind(to)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0.unwrap_or(0.0))
    }
    
    /// Get metric statistics
    pub async fn get_stats(
        &self,
        metric_name: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<MetricStats> {
        let stats = sqlx::query_as::<_, MetricStats>(
            "SELECT 
                COUNT(*) as count,
                MIN(value) as min_value,
                MAX(value) as max_value,
                AVG(value) as avg_value,
                STDDEV(value) as stddev_value
             FROM analytics.metrics
             WHERE metric_name = $1 AND recorded_at BETWEEN $2 AND $3"
        )
        .bind(metric_name)
        .bind(from)
        .bind(to)
        .fetch_one(self.pool)
        .await?;
        
        Ok(stats)
    }
}

/// Analytics events operations
pub struct EventsOps<'a> {
    pool: &'a PgPool,
}

impl<'a> EventsOps<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
    
    /// Record an event
    pub async fn record(
        &self,
        event_type: &str,
        user_id: Option<uuid::Uuid>,
        business_id: Option<uuid::Uuid>,
        event_data: serde_json::Value,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "INSERT INTO analytics.events (event_type, user_id, business_id, event_data)
             VALUES ($1, $2, $3, $4)
             RETURNING id"
        )
        .bind(event_type)
        .bind(user_id)
        .bind(business_id)
        .bind(event_data)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
    
    /// Get events by type
    pub async fn get_by_type(
        &self,
        event_type: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            "SELECT id, event_type, user_id, business_id, event_data, created_at
             FROM analytics.events
             WHERE event_type = $1 AND created_at BETWEEN $2 AND $3
             ORDER BY created_at DESC
             LIMIT $4"
        )
        .bind(event_type)
        .bind(from)
        .bind(to)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;
        
        Ok(events)
    }
    
    /// Get events by user
    pub async fn get_by_user(
        &self,
        user_id: uuid::Uuid,
        limit: i64,
    ) -> Result<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            "SELECT id, event_type, user_id, business_id, event_data, created_at
             FROM analytics.events
             WHERE user_id = $1
             ORDER BY created_at DESC
             LIMIT $2"
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;
        
        Ok(events)
    }
    
    /// Get events by business
    pub async fn get_by_business(
        &self,
        business_id: uuid::Uuid,
        limit: i64,
    ) -> Result<Vec<Event>> {
        let events = sqlx::query_as::<_, Event>(
            "SELECT id, event_type, user_id, business_id, event_data, created_at
             FROM analytics.events
             WHERE business_id = $1
             ORDER BY created_at DESC
             LIMIT $2"
        )
        .bind(business_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await?;
        
        Ok(events)
    }
    
    /// Get event count by type
    pub async fn count_by_type(
        &self,
        event_type: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<i64> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM analytics.events
             WHERE event_type = $1 AND created_at BETWEEN $2 AND $3"
        )
        .bind(event_type)
        .bind(from)
        .bind(to)
        .fetch_one(self.pool)
        .await?;
        
        Ok(result.0)
    }
}

// Data structures

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Metric {
    pub id: i64,
    pub metric_name: String,
    pub value: f64,
    pub labels: Option<serde_json::Value>,
    pub recorded_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct MetricStats {
    pub count: Option<i64>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub avg_value: Option<f64>,
    pub stddev_value: Option<f64>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Event {
    pub id: i64,
    pub event_type: String,
    pub user_id: Option<uuid::Uuid>,
    pub business_id: Option<uuid::Uuid>,
    pub event_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}
