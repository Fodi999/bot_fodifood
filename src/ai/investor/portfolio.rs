// 💼 Portfolio Management - Управление инвестиционным портфелем
//
// Позиции, пассивный доход (revenue share, profit share, staking)

use serde::{Deserialize, Serialize};

/// 📊 Инвестиционная позиция в проекте
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Position {
    /// Символ токена проекта
    pub project_symbol: String,
    /// Название проекта
    pub project_name: String,
    /// Количество токенов
    pub tokens: f64,
    /// Цена покупки за токен (USD)
    pub buy_price: f64,
    /// Доля от выручки проекта (0.0..1.0, например 0.02 = 2%)
    pub revenue_share: f64,
    /// Доля от прибыли проекта (0.0..1.0, например 0.03 = 3%)
    pub profit_share: f64,
    /// APR стейкинга (годовая ставка, например 0.10 = 10%)
    pub staking_apr: f64,
    /// Дата открытия позиции
    pub opened_at: chrono::DateTime<chrono::Utc>,
    /// Накопленные дивиденды (USD)
    pub accumulated_dividends: f64,
}

impl Position {
    /// Создать новую позицию
    pub fn new(
        project_symbol: String,
        project_name: String,
        tokens: f64,
        buy_price: f64,
    ) -> Self {
        Self {
            project_symbol,
            project_name,
            tokens,
            buy_price,
            revenue_share: 0.02,  // 2% по умолчанию
            profit_share: 0.03,   // 3% по умолчанию
            staking_apr: 0.10,    // 10% годовых по умолчанию
            opened_at: chrono::Utc::now(),
            accumulated_dividends: 0.0,
        }
    }

    /// Установить параметры дохода
    pub fn with_yield_params(mut self, revenue_share: f64, profit_share: f64, staking_apr: f64) -> Self {
        self.revenue_share = revenue_share;
        self.profit_share = profit_share;
        self.staking_apr = staking_apr;
        self
    }

    /// Текущая стоимость позиции
    pub fn current_value(&self, current_price: f64) -> f64 {
        self.tokens * current_price
    }

    /// P&L позиции (прибыль/убыток)
    pub fn pnl(&self, current_price: f64) -> f64 {
        self.current_value(current_price) - (self.tokens * self.buy_price)
    }

    /// P&L в процентах
    pub fn pnl_percent(&self, current_price: f64) -> f64 {
        if self.buy_price > 0.0 {
            ((current_price - self.buy_price) / self.buy_price) * 100.0
        } else {
            0.0
        }
    }

    /// Добавить дивиденды
    pub fn add_dividend(&mut self, amount: f64) {
        self.accumulated_dividends += amount;
    }
}

/// 💼 Инвестиционный портфель
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Portfolio {
    /// Наличные (USD)
    pub cash_usd: f64,
    /// Открытые позиции
    pub positions: Vec<Position>,
    /// История закрытых позиций
    pub closed_positions: Vec<Position>,
    /// Общая накопленная прибыль
    pub total_realized_pnl: f64,
}

impl Portfolio {
    /// Создать новый портфель
    pub fn new(initial_cash: f64) -> Self {
        Self {
            cash_usd: initial_cash,
            positions: Vec::new(),
            closed_positions: Vec::new(),
            total_realized_pnl: 0.0,
        }
    }

    /// Общая стоимость портфеля (cash + позиции)
    pub fn total_value(&self, price_fn: &impl Fn(&str) -> f64) -> f64 {
        let mut value = self.cash_usd;
        for pos in &self.positions {
            value += pos.current_value(price_fn(&pos.project_symbol));
        }
        value
    }

    /// Стоимость всех позиций
    pub fn positions_value(&self, price_fn: &impl Fn(&str) -> f64) -> f64 {
        self.positions.iter()
            .map(|p| p.current_value(price_fn(&p.project_symbol)))
            .sum()
    }

    /// Общий P&L по всем открытым позициям
    pub fn total_pnl(&self, price_fn: &impl Fn(&str) -> f64) -> f64 {
        self.positions.iter()
            .map(|p| p.pnl(price_fn(&p.project_symbol)))
            .sum()
    }

    /// Общий P&L в процентах
    pub fn total_pnl_percent(&self, price_fn: &impl Fn(&str) -> f64) -> f64 {
        let invested: f64 = self.positions.iter()
            .map(|p| p.tokens * p.buy_price)
            .sum();
        
        if invested > 0.0 {
            (self.total_pnl(price_fn) / invested) * 100.0
        } else {
            0.0
        }
    }

    /// Найти позицию по символу
    pub fn find_position(&self, symbol: &str) -> Option<&Position> {
        self.positions.iter()
            .find(|p| p.project_symbol == symbol)
    }

    /// Найти позицию по символу (мутабельно)
    pub fn find_position_mut(&mut self, symbol: &str) -> Option<&mut Position> {
        self.positions.iter_mut()
            .find(|p| p.project_symbol == symbol)
    }

    /// Открыть новую позицию
    pub fn open_position(&mut self, position: Position) -> Result<(), String> {
        let cost = position.tokens * position.buy_price;
        
        if cost > self.cash_usd {
            return Err(format!(
                "Недостаточно средств: требуется ${:.2}, доступно ${:.2}",
                cost, self.cash_usd
            ));
        }

        self.cash_usd -= cost;
        self.positions.push(position);
        Ok(())
    }

    /// Закрыть позицию
    pub fn close_position(&mut self, symbol: &str, current_price: f64) -> Result<f64, String> {
        let pos_index = self.positions.iter()
            .position(|p| p.project_symbol == symbol)
            .ok_or_else(|| format!("Позиция {} не найдена", symbol))?;

        let mut position = self.positions.remove(pos_index);
        let proceeds = position.current_value(current_price);
        let pnl = position.pnl(current_price);

        self.cash_usd += proceeds + position.accumulated_dividends;
        self.total_realized_pnl += pnl;

        position.accumulated_dividends = 0.0; // Выплачены
        self.closed_positions.push(position);

        Ok(pnl)
    }

    /// Добавить дивиденды к позиции
    pub fn add_dividend_to_position(&mut self, symbol: &str, amount: f64) -> Result<(), String> {
        let position = self.find_position_mut(symbol)
            .ok_or_else(|| format!("Позиция {} не найдена", symbol))?;
        
        position.add_dividend(amount);
        Ok(())
    }

    /// Получить сводку портфеля как структуру
    pub fn get_summary(&self, price_fn: &impl Fn(&str) -> f64) -> PortfolioSummary {
        let total_value = self.total_value(price_fn);
        let total_invested = self.positions.iter().map(|p| p.tokens * p.buy_price).sum();
        let unrealized_pnl = self.total_pnl(price_fn);
        let total_dividends = self.positions.iter().map(|p| p.accumulated_dividends).sum();
        let return_pct = if total_invested > 0.0 {
            ((unrealized_pnl + total_dividends) / total_invested) * 100.0
        } else {
            0.0
        };

        PortfolioSummary {
            cash_usd: self.cash_usd,
            positions_count: self.positions.len(),
            total_value,
            total_invested,
            unrealized_pnl,
            total_dividends,
            return_pct,
        }
    }

    /// Получить сводку портфеля для вывода
    pub fn summary(&self, price_fn: &impl Fn(&str) -> f64) {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  💼 Инвестиционный портфель                                ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let total_value = self.total_value(price_fn);
        let positions_value = self.positions_value(price_fn);
        let total_pnl = self.total_pnl(price_fn);
        let total_dividends: f64 = self.positions.iter()
            .map(|p| p.accumulated_dividends)
            .sum();

        println!("💰 **Баланс:**");
        println!("   • Наличные: ${:.2}", self.cash_usd);
        println!("   • В позициях: ${:.2}", positions_value);
        println!("   • Общая стоимость: ${:.2}", total_value);
        println!();

        println!("📈 **P&L:**");
        println!("   • Нереализованный: ${:.2} ({:+.1}%)", 
            total_pnl, 
            self.total_pnl_percent(price_fn)
        );
        println!("   • Реализованный: ${:.2}", self.total_realized_pnl);
        println!("   • Накопленные дивиденды: ${:.2}", total_dividends);
        println!();

        if !self.positions.is_empty() {
            println!("📊 **Открытые позиции ({}):**\n", self.positions.len());

            for pos in &self.positions {
                let current_price = price_fn(&pos.project_symbol);
                let current_value = pos.current_value(current_price);
                let pnl = pos.pnl(current_price);
                let pnl_pct = pos.pnl_percent(current_price);

                println!("   🔹 {} ({})", pos.project_name, pos.project_symbol);
                println!("      • Токенов: {:.2}", pos.tokens);
                println!("      • Цена входа: ${:.2} → Текущая: ${:.2}", 
                    pos.buy_price, current_price);
                println!("      • Стоимость: ${:.2}", current_value);
                println!("      • P&L: ${:.2} ({:+.1}%)", pnl, pnl_pct);
                println!("      • Дивиденды: ${:.2}", pos.accumulated_dividends);
                println!("      • Доходность: Rev {:.1}% | Profit {:.1}% | APR {:.1}%",
                    pos.revenue_share * 100.0,
                    pos.profit_share * 100.0,
                    pos.staking_apr * 100.0
                );
                println!();
            }
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    }
}

/// 📊 Portfolio summary structure
#[derive(Debug, Clone)]
pub struct PortfolioSummary {
    pub cash_usd: f64,
    pub positions_count: usize,
    pub total_value: f64,
    pub total_invested: f64,
    pub unrealized_pnl: f64,
    pub total_dividends: f64,
    pub return_pct: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_pnl() {
        let pos = Position::new(
            "TEST".to_string(),
            "Test Project".to_string(),
            100.0,
            1.0,
        );

        assert_eq!(pos.current_value(1.5), 150.0);
        assert_eq!(pos.pnl(1.5), 50.0);
        assert_eq!(pos.pnl_percent(1.5), 50.0);
    }

    #[test]
    fn test_portfolio_open_position() {
        let mut portfolio = Portfolio::new(1000.0);
        let position = Position::new(
            "TEST".to_string(),
            "Test".to_string(),
            100.0,
            5.0,
        );

        portfolio.open_position(position).unwrap();
        assert_eq!(portfolio.cash_usd, 500.0);
        assert_eq!(portfolio.positions.len(), 1);
    }
}
