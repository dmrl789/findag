# Real-Time Financial Data Setup

FinDAG supports real-time financial data from multiple sources with configurable delays. This guide explains how to set up real-time data feeds.

## Available Data Sources

### 1. Alpha Vantage (Recommended for Free Tier)
- **Free Tier**: 5 requests/minute, 12-second delay
- **Paid Tier**: 500+ requests/minute, real-time data
- **Get API Key**: https://www.alphavantage.co/support/#api-key
- **Coverage**: Stocks, Forex, Commodities

### 2. Polygon.io
- **Free Tier**: 5 requests/minute, 2-second delay
- **Paid Tier**: Real-time data with higher limits
- **Get API Key**: https://polygon.io/
- **Coverage**: Stocks, Forex, Crypto

### 3. Finnhub
- **Free Tier**: 60 requests/minute, 1-second delay
- **Paid Tier**: Real-time data with higher limits
- **Get API Key**: https://finnhub.io/
- **Coverage**: Stocks, Forex, Crypto

### 4. Yahoo Finance
- **Free Tier**: 10 requests/minute, 5-second delay
- **No API Key Required** for basic usage
- **Coverage**: Stocks, ETFs

## Setup Instructions

### Step 1: Get API Keys
1. Visit the provider websites listed above
2. Sign up for free accounts
3. Generate API keys
4. Note the rate limits and delays for each service

### Step 2: Configure Environment Variables (Optional)
Create a `.env` file in the `findag-ui` directory for additional configuration:

```bash
# Alpha Vantage API Key (optional - can also be set in UI)
VITE_ALPHA_VANTAGE_API_KEY=your_alpha_vantage_api_key_here

# Polygon.io API Key (optional - can also be set in UI)
VITE_POLYGON_API_KEY=your_polygon_api_key_here

# Finnhub API Key (optional - can also be set in UI)
VITE_FINNHUB_API_KEY=your_finnhub_api_key_here

# Enable real-time data
VITE_ENABLE_REAL_DATA=true

# Data refresh interval (30 seconds)
VITE_DATA_REFRESH_INTERVAL=30000
```

**Note:** API keys can also be configured directly in the UI, which is the recommended approach for easier management.

### Step 3: Configure in the UI
1. Start the development server: `npm run dev`
2. Navigate to the Trading page
3. Click "Data Sources" button
4. Enter your API keys in the configuration panel
5. Test each API key using the "Test" button
6. Save the configuration

## Data Quality and Delays

### Free Tier Limitations
- **Alpha Vantage**: 12-second delay, 5 requests/minute
- **Polygon.io**: 2-second delay, 5 requests/minute
- **Finnhub**: 1-second delay, 60 requests/minute
- **Yahoo Finance**: 5-second delay, 10 requests/minute

### Institutional Considerations
For institutional use, consider:
- **Paid API tiers** for real-time data
- **Multiple data sources** for redundancy
- **Data validation** and quality checks
- **Compliance** with regulatory requirements

## Supported Instruments

### Forex Pairs
- EUR/USD, GBP/USD, USD/JPY
- All major and minor currency pairs
- Real-time exchange rates with delays

### Stocks
- AAPL, MSFT, GOOGL, TSLA
- All major US and international stocks
- Real-time price and volume data

### Commodities
- XAU/USD (Gold), WTI/USD (Oil)
- Silver, Platinum, Natural Gas
- Real-time commodity prices

### Bonds
- US 10-Year Treasury
- Government and corporate bonds
- Yield and price data

## Troubleshooting

### Common Issues

1. **API Key Invalid**
   - Verify the API key is correct
   - Check if the key has expired
   - Ensure the key has proper permissions

2. **Rate Limit Exceeded**
   - Wait for the rate limit window to reset
   - Consider upgrading to a paid tier
   - Use multiple data sources

3. **Data Not Loading**
   - Check browser console for errors
   - Verify network connectivity
   - Test API endpoints directly

4. **Delayed Data**
   - This is normal for free tiers
   - Consider paid tiers for real-time data
   - Check the configured delay settings

### Debug Information
- Check the browser console for detailed error messages
- Use the "Status" page to see data loading information
- Monitor the data source indicators in the UI

## Security Considerations

- **Never commit API keys** to version control
- **Use environment variables** for configuration
- **Rotate API keys** regularly
- **Monitor usage** to prevent abuse
- **Comply with provider terms** of service

## Performance Optimization

- **Cache data** locally when possible
- **Use appropriate refresh intervals**
- **Implement error handling** and fallbacks
- **Monitor API usage** and costs
- **Consider data aggregation** for efficiency

## Compliance and Regulatory

FinDAG is designed for institutional use and includes:
- **Audit trails** for all data access
- **Data source tracking** and validation
- **Compliance reporting** capabilities
- **Regulatory data** requirements support

For production deployment, ensure compliance with:
- **MiFID II** (EU)
- **Dodd-Frank** (US)
- **Basel III** requirements
- **Local regulatory** frameworks 