# System18

![System18](./assets/system18.png)

**A professional, fully automated trading bot platform that backs up decisions with data, adapts strategy to market conditions, manages risk intelligently, and runs 24/7 on autopilot.**

---

## Core Features

### 1. Strategy Engine

System18 features a flexible, multi-strategy framework that adapts to different market conditions.

**Strategy Types:**
- **Momentum** — Follows strong market trends for directional trades  
- **Mean Reversion** — Trades pullbacks toward statistical averages  
- **Volatility-Based** — Dynamically adapts to market movement intensity  

**AI-Assisted Signals:**  
System18 integrates lightweight machine learning to improve trade accuracy and adapt to market behavior.

- **Predicts price trends using historical data and simple models like linear regression**  
  *Example:* If a stock has been steadily rising, the bot predicts a short-term continuation and prepares to enter a buy position.  
- **Optimizes entry and exit points to reduce risk and maximize potential profits**  
  *Example:* Instead of buying immediately at a specific price, the bot calculates a slightly higher or lower point for safer entry.  
- **Learns from performance over time and adapts to market changes**  
  *Example:* If a pattern stops reliably predicting moves, the bot updates its rules to avoid entering at that pattern.

**Auto Strategy Selection:**
- Automatically selects the optimal strategy for each market  
- Based on real-time performance metrics  
- Adapts as market conditions change

---

### 2. Backtesting & Simulation

Test strategies against historical data before risking capital.

**Capabilities:**
- Downloads and stores comprehensive historical market data  
- Simulates trades over past periods with high accuracy  
- Generates detailed performance reports including:
  - Win Rate
  - Return on Investment (ROI)
  - Sharpe Ratio
  - Maximum Drawdown
  - Other key metrics

**Benefits:**
- Validate strategy effectiveness before live deployment  
- Compare multiple strategies side-by-side  
- Build confidence in your trading approach

---

### 3. Live Trading & Deployment

System18 operates as a production-ready trading system.

**Production Setup:**
- Hosted on Virtual Private Server (VPS) for maximum uptime  
- Runs continuously 24/7 without interruption  
- Real-time market data via WebSocket connections  
- Automatic trade execution with minimal latency  
- Email notifications for critical events

**Deployment Features:**
- Easy start/stop controls  
- Configurable trading parameters  
- Multi-market support  
- Fault-tolerant operation

---

### 4. Risk Management

Intelligent risk controls protect your capital and optimize position sizing.

**Risk Systems:**
- **Kelly Criterion** for mathematically optimal position sizing  
- **Automatic Stop Loss & Take Profit** on every trade  
- **Portfolio-Level Limits** to cap total exposure  
- **Drawdown Protection** to pause trading during adverse conditions

**Safety Features:**
- Per-trade risk caps  
- Daily loss limits  
- Maximum position concentration controls

---

### 5. Real-Time Monitoring

Complete visibility into bot operations through an intuitive dashboard.

**Dashboard Features:**
- Live view of active positions  
- Real-time P&L (Profit & Loss) tracking  
- Performance charts and visualizations  
- Complete trade history logs  
- Bot health status indicators  
- Start/Stop controls  
- Strategy performance breakdown

**Access:**
- Simple HTML/JS interface  
- No complex setup required  
- Works on any modern browser

---

### 6. Database & Analytics

Comprehensive data tracking for performance analysis and optimization.

**Data Storage:**
- SQLite database for reliability and portability  
- Stores all trade entries and exits  
- Complete P&L history  
- Strategy performance metrics over time

**Analytics Capabilities:**
- Identify best trading times  
- Discover highest-performing assets  
- Analyze strategy strengths and weaknesses  
- Track improvement over time  
- Generate custom performance reports

---

### 7. Notifications

Stay informed with intelligent alerts delivered to your inbox.

**Notification Triggers:**
- Trade opens or closes  
- Stop losses are triggered  
- Performance crosses key thresholds  
- Bot starts or stops  
- Error conditions or warnings

**Delivery:**
- Email-based for reliability

---

## Performance Metrics

System18 tracks comprehensive metrics including:
- Total Return & ROI  
- Win Rate & Profit Factor  
- Sharpe Ratio & Sortino Ratio  
- Maximum Drawdown  
- Average Win vs. Average Loss  
- Best/Worst Trades  
- Strategy-specific performance

---

## Support & Notifications

System18 keeps you informed with email alerts for all critical events. Configure your SMTP settings to receive:
- Trade confirmations  
- Risk alerts  
- Performance updates  
- System status notifications

---

## Disclaimer

**Trading involves substantial risk of loss.** System18 is a tool to assist with trading decisions, but past performance does not guarantee future results. Only trade with capital you can afford to lose. Always understand the strategies being deployed and monitor system performance regularly.

---

## License

DTFYW

---

**System18** — Trading bot
