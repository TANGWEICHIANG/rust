// Currency helper functions for the currency converter app

/**
 * Get the full name of a currency from its code
 * @param {string} code - Currency code (e.g., 'USD', 'EUR')
 * @returns {string} - Full currency name
 */
export const getCurrencyName = (code) => {
  const names = {
    'USD': 'US Dollar',
    'EUR': 'Euro',
    'GBP': 'British Pound',
    'MYR': 'Malaysian Ringgit',
    'JPY': 'Japanese Yen',
    'CNY': 'Chinese Yuan',
    'SGD': 'Singapore Dollar',
    'AUD': 'Australian Dollar',
    'CAD': 'Canadian Dollar',
    'INR': 'Indian Rupee',
    'KRW': 'South Korean Won',
    'THB': 'Thai Baht',
    'IDR': 'Indonesian Rupiah',
    'PHP': 'Philippine Peso',
    'VND': 'Vietnamese Dong',
    'CHF': 'Swiss Franc',
    'HKD': 'Hong Kong Dollar',
    'NZD': 'New Zealand Dollar',
    'SEK': 'Swedish Krona',
    'NOK': 'Norwegian Krone',
    'DKK': 'Danish Krone',
    'MXN': 'Mexican Peso',
    'BRL': 'Brazilian Real',
    'ZAR': 'South African Rand',
    'RUB': 'Russian Ruble',
    'TRY': 'Turkish Lira',
    'AED': 'UAE Dirham',
    'SAR': 'Saudi Riyal',
  };
  return names[code] || code;
};

/**
 * Get the country/region code for a currency code (for flag display)
 * @param {string} currencyCode - Currency code
 * @returns {string} - Country/region code for flag API
 */
export const getCountryCode = (currencyCode) => {
  const mapping = {
    'USD': 'US',
    'EUR': 'EU',
    'GBP': 'GB',
    'MYR': 'MY',
    'JPY': 'JP',
    'CNY': 'CN',
    'SGD': 'SG',
    'AUD': 'AU',
    'CAD': 'CA',
    'INR': 'IN',
    'KRW': 'KR',
    'THB': 'TH',
    'IDR': 'ID',
    'PHP': 'PH',
    'VND': 'VN',
    'CHF': 'CH',
    'HKD': 'HK',
    'NZD': 'NZ',
    'SEK': 'SE',
    'NOK': 'NO',
    'DKK': 'DK',
    'MXN': 'MX',
    'BRL': 'BR',
    'ZAR': 'ZA',
    'RUB': 'RU',
    'TRY': 'TR',
    'AED': 'AE',
    'SAR': 'SA',
  };
  return mapping[currencyCode] || 'EU';
};

/**
 * Handle flag image loading errors by hiding the broken image
 * @param {Event} event - The error event
 */
export const handleFlagError = (event) => {
  const img = event.target;
  img.style.display = 'none';
  
  // Optional: Add a fallback icon
  const parent = img.parentElement;
  if (parent) {
    const fallback = document.createElement('div');
    fallback.className = 'w-6 h-6 rounded-full bg-gray-200 flex items-center justify-center text-xs font-bold';
    fallback.textContent = img.alt ? img.alt.substring(0, 2) : '??';
    parent.appendChild(fallback);
  }
};

/**
 * Format a number as currency with proper localization
 * @param {number} amount - Amount to format
 * @param {string} currencyCode - Currency code
 * @param {string} locale - Locale string (defaults to 'en-US')
 * @returns {string} - Formatted currency string
 */
export const formatCurrency = (amount, currencyCode, locale = 'en-US') => {
  try {
    return new Intl.NumberFormat(locale, {
      style: 'currency',
      currency: currencyCode,
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    }).format(amount);
  } catch (error) {
    // Fallback formatting if Intl.NumberFormat fails
    return `${currencyCode} ${amount.toLocaleString(locale, {
      minimumFractionDigits: 2,
      maximumFractionDigits: 2,
    })}`;
  }
};

/**
 * Get the symbol for a currency code
 * @param {string} currencyCode - Currency code
 * @returns {string} - Currency symbol
 */
export const getCurrencySymbol = (currencyCode) => {
  const symbols = {
    'USD': '$',
    'EUR': '€',
    'GBP': '£',
    'MYR': 'RM',
    'JPY': '¥',
    'CNY': '¥',
    'SGD': 'S$',
    'AUD': 'A$',
    'CAD': 'C$',
    'INR': '₹',
    'KRW': '₩',
    'THB': '฿',
    'PHP': '₱',
    'VND': '₫',
    'CHF': 'CHF',
    'HKD': 'HK$',
    'NZD': 'NZ$',
  };
  return symbols[currencyCode] || currencyCode;
};

/**
 * Get flag URL for a currency code
 * @param {string} currencyCode - Currency code
 * @param {string} style - Flag style: 'flat', 'shiny', etc. (default: 'flat')
 * @param {number} size - Flag size: 16, 24, 32, 48, 64 (default: 64)
 * @returns {string} - Flag image URL
 */
export const getFlagUrl = (currencyCode, style = 'flat', size = 64) => {
  const countryCode = getCountryCode(currencyCode);
  return `https://flagsapi.com/${countryCode}/${style}/${size}.png`;
};

/**
 * Check if a currency code is valid/exists in our mapping
 * @param {string} currencyCode - Currency code to check
 * @returns {boolean} - Whether the currency code is valid
 */
export const isValidCurrency = (currencyCode) => {
  const allCurrencies = [
    'USD', 'EUR', 'GBP', 'MYR', 'JPY', 'CNY', 'SGD', 'AUD', 'CAD', 'INR',
    'KRW', 'THB', 'IDR', 'PHP', 'VND', 'CHF', 'HKD', 'NZD', 'SEK', 'NOK',
    'DKK', 'MXN', 'BRL', 'ZAR', 'RUB', 'TRY', 'AED', 'SAR'
  ];
  return allCurrencies.includes(currencyCode.toUpperCase());
};

// Optional: Export all functions as a default object
export default {
  getCurrencyName,
  getCountryCode,
  handleFlagError,
  formatCurrency,
  getCurrencySymbol,
  getFlagUrl,
  isValidCurrency,
};