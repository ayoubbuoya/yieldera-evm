# YieldAI 🚀

> AI-Powered Liquidity Management for Polygon DeFi

YieldAI is an intelligent liquidity optimization platform built for the Polygon Buildathon. It leverages Google Gemini AI to analyze market data and suggest optimal liquidity ranges for concentrated liquidity pools, helping users maximize their yield farming returns.

## 🌟 Overview

YieldAI combines real-time market data from CoinGecko with advanced AI analysis to provide data-driven liquidity range suggestions for DeFi protocols on Polygon. The platform supports multiple strategies (narrow and wide) to accommodate different risk profiles and market conditions.

## ✨ Key Features

- **AI-Powered Analysis**: Utilizes Google Gemini Flash to analyze OHLCV (Open, High, Low, Close, Volume) data and suggest optimal liquidity ranges
- **Multiple Strategies**: 
  - **Narrow Strategy**: Concentrated liquidity positions for higher capital efficiency
  - **Wide Strategy**: Broader liquidity ranges for reduced impermanent loss risk
- **Real-Time Market Data**: Integrates with CoinGecko API for up-to-date pool analytics
- **Confidence Scoring**: Each suggestion includes a confidence level and detailed reasoning
- **RESTful API**: Easy-to-integrate HTTP endpoints with comprehensive Swagger documentation
- **Polygon Native**: Built specifically for Polygon PoS network (Chain ID: 137)

## 🏗️ Architecture

YieldAI is built with Rust using the Actix-web framework for high performance and reliability:

```
backend/
├── src/
│   ├── api/           # HTTP endpoints and request handlers
│   ├── core/          # Core functionality (AI agent, CoinGecko integration)
│   ├── strategies/    # Liquidity strategies (narrow, wide)
│   ├── config.rs      # Configuration management
│   ├── state.rs       # Application state
│   ├── types.rs       # Data structures and schemas
│   └── main.rs        # Application entry point
```

## 🚀 Getting Started

### Prerequisites

- Rust 1.70+ (edition 2024)
- CoinGecko API Key
- Google Gemini API Key

### Installation

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd yieldera-evm/backend
   ```

2. **Set up environment variables**
   
   Create a `.env` file in the `backend` directory:
   ```env
   COINGECKO_API_KEY=your_coingecko_api_key_here
   GOOGLE_API_KEY=your_gemini_api_key_here
   ```

3. **Build the project**
   ```bash
   cargo build --release
   ```

4. **Run the server**
   ```bash
   cargo run --release
   ```

The server will start on `http://127.0.0.1:8090`

## 📚 API Documentation

### Endpoints

#### Health Check
```
GET /health
```
Returns the health status of the API.

#### Get Pool CoinGecko Data
```
GET /coingecko/{pool_address}
```
Fetches OHLCV data for a specific pool from CoinGecko.

**Parameters:**
- `pool_address`: The address of the liquidity pool on Polygon

**Response:**
```json
{
  "data": {
    "id": "string",
    "attributes": {
      "ohlcv_list": [
        [timestamp, open, high, low, close, volume]
      ]
    }
  }
}
```

#### Suggest Liquidity Range
```
GET /pools/{pool_address}/liquidity/{strategy}/suggest
```
Get AI-powered liquidity range suggestions for a pool.

**Parameters:**
- `pool_address`: The address of the liquidity pool on Polygon
- `strategy`: Either `narrow` or `wide`

**Response:**
```json
{
  "up_price": 1234.56,
  "down_price": 1000.00,
  "confidence": 0.85,
  "reason": "Based on recent volatility patterns and volume analysis...",
  "strategy": "Narrow"
}
```

### Swagger UI

Interactive API documentation is available at:
```
http://127.0.0.1:8090/swagger-ui/
```

## 🎯 How It Works

1. **Data Collection**: YieldAI fetches historical OHLCV data from CoinGecko for the specified pool
2. **AI Analysis**: The data is sent to Google Gemini AI with strategy-specific prompts
3. **Range Calculation**: The AI analyzes market patterns, volatility, and trends to suggest optimal price ranges
4. **Confidence Scoring**: Each suggestion includes a confidence score and detailed reasoning
5. **Response**: The API returns actionable liquidity range recommendations

## 🛠️ Technology Stack

- **Backend Framework**: Actix-web (Rust)
- **AI/ML**: Google Gemini Flash (via rig-core)
- **Blockchain**: Alloy (Ethereum/Polygon interaction)
- **Data Source**: CoinGecko API
- **API Documentation**: Utoipa + Swagger UI
- **Logging**: Tracing + Tracing-subscriber
- **Async Runtime**: Tokio

## 🔧 Configuration

The application uses the following configuration constants (defined in `config.rs`):

- **Network**: Polygon PoS (`polygon_pos`)
- **RPC URL**: `https://polygon-rpc.com`
- **Chain ID**: 137

## 📊 Strategies Explained

### Narrow Strategy
- **Best for**: Stable pairs, low volatility markets
- **Characteristics**: Tight price ranges, higher capital efficiency
- **Risk**: Higher impermanent loss if price moves outside range
- **Reward**: Maximum fee generation when price stays in range

### Wide Strategy
- **Best for**: Volatile pairs, uncertain markets
- **Characteristics**: Broader price ranges, lower capital efficiency
- **Risk**: Lower impermanent loss risk
- **Reward**: More consistent fee generation across price movements

## 🔐 Security Considerations

- API keys are loaded from environment variables
- CORS is configured to allow cross-origin requests
- All external API calls include proper error handling
- Structured logging for audit trails