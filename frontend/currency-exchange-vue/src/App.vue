<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { RatesResponse, ConversionResult, HistoryItem } from '../types';
import { getCurrencyName, getCountryCode, handleFlagError } from './utils/currencies.js';
import { languages, defaultLocale } from './utils/languages.js';

const API_BASE = 'http://localhost:3000/api';

const currentLocale = ref(defaultLocale);
const t = computed(() => languages[currentLocale.value]);
const isLangOpen = ref(false);

const rates = ref<RatesResponse | null>(null);
const amount = ref<number>(100);
const sourceCurrency = ref<string>('MYR');
const targetCurrency = ref<string>('USD');
const loading = ref<boolean>(true);
const error = ref<string | null>(null);
const history = ref<HistoryItem[]>([]);
const lastResult = ref<ConversionResult | null>(null);
const userDetectedCurrency = ref<string | null>(null);

const fetchRates = async () => {
  try {
    loading.value = true;
    const response = await fetch(`${API_BASE}/rates`);
    if (!response.ok) throw new Error('Backend server is not reachable');
    const data: RatesResponse = await response.json();
    rates.value = data;
    error.value = null;
  } catch (err) {
    error.value = 'Failed to connect to the Rust backend. Ensure it is running on port 3000.';
    console.error(err);
  } finally {
    loading.value = false;
  }
};

const handleConvert = async () => {
  if (!amount.value || amount.value <= 0) return;
  
  try {
    loading.value = true;
    const url = `${API_BASE}/convert?amount=${amount.value}&from=${sourceCurrency.value}&to=${targetCurrency.value}`;
    const response = await fetch(url);
    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(errorText || 'Conversion failed');
    }
    
    const data: ConversionResult = await response.json();
    lastResult.value = data;
    
    history.value.unshift({
      id: Math.random().toString(36).substring(2, 11),
      from: data.from, 
      to: data.to,
      amount: data.amount,
      result: data.result,
      timestamp: Date.now()
    });
    
    if (history.value.length > 10) history.value.pop();
    error.value = null;
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Error performing conversion.';
  } finally {
    loading.value = false;
  }
};

const sortedCurrencies = computed(() => {
  if (!rates.value) return [];
  return Object.keys(rates.value.rates).sort();
});

const topRates = computed(() => {
  if (!rates.value) return [];
  const important = ['USD', 'EUR', 'GBP', 'JPY', 'CNY'];
  return important.map(curr => ({
    code: curr,
    rate: rates.value?.rates[curr]
  })).filter(i => i.rate !== undefined);
});

const detectUserCurrency = async () => {
  try {
    const response = await fetch(`${API_BASE}/detect-currency`);
    if (response.ok) {
      const data = await response.json();
      userDetectedCurrency.value = data.detected_currency;
      
      if (data.available && data.detected_currency) {
        sourceCurrency.value = data.detected_currency;
        if (data.detected_currency !== 'USD') {
          targetCurrency.value = 'USD';
        }
      }
    }
  } catch (err) {
    console.warn('Could not detect user currency:', err);
  }
};

const selectLang = (code: string) => {
  currentLocale.value = code;
  isLangOpen.value = false;
};

// Close dropdown when clicking outside
const closeDropdown = (e: MouseEvent) => {
  if (!(e.target as HTMLElement).closest('.lang-switcher')) {
    isLangOpen.value = false;
  }
};

onMounted(async () => {
  await fetchRates();
  await detectUserCurrency(); 
  window.addEventListener('click', closeDropdown);
});

onUnmounted(() => {
  window.removeEventListener('click', closeDropdown);
});
</script>

<template>
  <div class="min-h-screen bg-slate-50 text-slate-900 pb-12">
    <!-- Navigation -->
    <nav class="bg-white border-b border-slate-200 sticky top-0 z-50">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16 items-center">
          <div class="flex items-center gap-3">
            <div class="bg-indigo-600 p-2 rounded-lg">
              <i class="fa-solid fa-coins text-white text-xl"></i>
            </div>
            <h1 class="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-indigo-600 to-blue-600">
              {{ t.converter }}
            </h1>
          </div>
          <div class="flex items-center gap-4">
            <!-- Custom Language Selector -->
            <div class="relative lang-switcher">
              <button 
                @click.stop="isLangOpen = !isLangOpen"
                class="flex items-center gap-2 px-4 py-2 bg-slate-100 hover:bg-slate-200 rounded-full text-xs font-bold text-slate-700 transition-colors focus:ring-2 focus:ring-indigo-100 outline-none"
              >
                <i class="fa-solid fa-globe text-indigo-500"></i>
                {{ languages[currentLocale].name }}
                <i class="fa-solid fa-chevron-down text-[10px] transition-transform" :class="{'rotate-180': isLangOpen}"></i>
              </button>
              
              <transition name="dropdown">
                <div v-if="isLangOpen" class="absolute right-0 mt-2 w-48 bg-white border border-slate-100 rounded-2xl shadow-xl py-2 z-[60]">
                  <button 
                    v-for="(lang, code) in languages" 
                    :key="code"
                    @click="selectLang(code)"
                    class="w-full text-left px-4 py-2.5 text-xs font-semibold flex items-center justify-between transition-colors hover:bg-indigo-50"
                    :class="currentLocale === code ? 'text-indigo-600 bg-indigo-50/50' : 'text-slate-600'"
                  >
                    {{ lang.name }}
                    <i v-if="currentLocale === code" class="fa-solid fa-check text-[10px]"></i>
                  </button>
                </div>
              </transition>
            </div>

            <div v-if="rates" class="hidden sm:flex items-center gap-2 px-3 py-1 bg-green-50 text-green-700 rounded-full text-[10px] font-bold uppercase tracking-wider">
              <span class="w-1.5 h-1.5 bg-green-500 rounded-full animate-pulse"></span>
              {{ t.refresh }}
            </div>
            <button @click="fetchRates" class="w-8 h-8 flex items-center justify-center text-slate-400 hover:text-indigo-600 transition-colors rounded-full hover:bg-slate-100">
              <i class="fa-solid fa-arrows-rotate" :class="{'animate-spin': loading && !rates}"></i>
            </button>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 mt-8">
      <!-- Error Alert -->
      <div v-if="error" class="mb-6 p-4 bg-red-50 border border-red-200 text-red-700 rounded-2xl flex items-center gap-3 animate-bounce">
        <i class="fa-solid fa-triangle-exclamation"></i>
        <p>{{ error }}</p>
      </div>

      <div class="grid grid-cols-1 lg:grid-cols-12 gap-8">
        
        <!-- Left Column: Converter -->
        <div class="lg:col-span-8 space-y-8">
          <div class="glass-card rounded-[2.5rem] p-8 shadow-xl shadow-slate-200/50">
            <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
              <span class="w-8 h-8 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center text-sm">
                <i class="fa-solid fa-calculator"></i>
              </span>
              {{ t.convert }}
            </h2>
            
            <div class="grid grid-cols-1 md:grid-cols-12 gap-6 items-end">
              <!-- From Currency -->
              <div class="md:col-span-5 space-y-3">
                <label class="text-xs font-bold uppercase tracking-wider text-slate-400 ml-1">{{ t.sourceCurrency }}</label>
                <div class="relative">
                  <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none z-10">
                    <div v-if="sourceCurrency" class="w-6 h-6 rounded-full overflow-hidden border border-slate-200 bg-white">
                      <img 
                        :src="`https://flagsapi.com/${getCountryCode(sourceCurrency)}/flat/64.png`" 
                        class="w-full h-full object-cover"
                        :alt="sourceCurrency"
                        @error="handleFlagError"
                      />
                    </div>
                  </div>
                  <select 
                    v-model="sourceCurrency"
                    class="w-full pl-12 pr-10 py-4 bg-white border border-slate-200 rounded-2xl text-slate-700 font-bold focus:ring-4 focus:ring-indigo-100 focus:border-indigo-500 outline-none transition-all cursor-pointer appearance-none shadow-sm relative z-0"
                  >
                    <option v-for="curr in sortedCurrencies" :key="curr" :value="curr">
                      {{ curr }} - {{ getCurrencyName(curr) }}
                    </option>
                  </select>
                  <i class="fa-solid fa-chevron-down absolute right-4 top-1/2 -translate-y-1/2 text-slate-300 text-xs pointer-events-none"></i>
                  
                  <div v-if="userDetectedCurrency && sourceCurrency === userDetectedCurrency" 
                      class="absolute right-10 top-1/2 -translate-y-1/2">
                    <span class="text-[10px] px-2 py-0.5 bg-blue-100 text-blue-600 rounded-full font-bold">
                      {{ t.yourLocation }}
                    </span>
                  </div>
                </div>
              </div>

              <!-- Swap Icon -->
              <div class="md:col-span-2 flex justify-center pb-3">
                 <button 
                  @click="[sourceCurrency, targetCurrency] = [targetCurrency, sourceCurrency]"
                  class="p-4 bg-indigo-50 text-indigo-600 rounded-2xl rotate-90 md:rotate-0 hover:bg-indigo-100 transition-colors active:scale-95"
                 >
                   <i class="fa-solid fa-right-left"></i>
                 </button>
              </div>

              <!-- To Target -->
              <div class="md:col-span-5 space-y-3">
                <label class="text-xs font-bold uppercase tracking-wider text-slate-400 ml-1">{{ t.targetCurrency }}</label>
                <div class="relative">
                  <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none z-10">
                    <div v-if="targetCurrency" class="w-6 h-6 rounded-full overflow-hidden border border-slate-200 bg-white">
                      <img 
                        :src="`https://flagsapi.com/${getCountryCode(targetCurrency)}/flat/64.png`" 
                        class="w-full h-full object-cover"
                        :alt="targetCurrency"
                        @error="handleFlagError"
                      />
                    </div>
                  </div>

                  <select 
                    v-model="targetCurrency"
                    class="w-full pl-12 pr-10 py-4 bg-white border border-slate-200 rounded-2xl text-slate-700 font-bold focus:ring-4 focus:ring-indigo-100 focus:border-indigo-500 outline-none transition-all cursor-pointer appearance-none shadow-sm relative z-0"
                  >
                    <option v-for="curr in sortedCurrencies" :key="curr" :value="curr">
                      {{ curr }} - {{ getCurrencyName(curr) }}
                    </option>
                  </select>
                  <i class="fa-solid fa-chevron-down absolute right-4 top-1/2 -translate-y-1/2 text-slate-300 text-xs pointer-events-none"></i>
                </div>
              </div>

              <!-- Input Amount -->
              <div class="md:col-span-8 space-y-3">
                <label class="text-xs font-bold uppercase tracking-wider text-slate-400 ml-1">{{ t.amountToSend }}</label>
                <div class="relative group">
                  <span class="absolute inset-y-0 left-0 pl-5 flex items-center text-slate-400 font-black group-focus-within:text-indigo-600 transition-colors">
                    {{ sourceCurrency }}
                  </span>
                  <input 
                    type="number" 
                    v-model="amount"
                    class="w-full pl-16 pr-6 py-5 bg-white border-2 border-slate-100 rounded-3xl text-3xl font-black focus:border-indigo-500 focus:ring-0 outline-none transition-all shadow-sm hover:border-slate-200"
                    placeholder="0.00"
                  />
                </div>
              </div>

              <!-- Submit -->
              <div class="md:col-span-4">
                <button 
                  @click="handleConvert"
                  :disabled="loading"
                  class="w-full py-6 bg-indigo-600 hover:bg-indigo-700 text-white font-black rounded-3xl transition-all active:scale-[0.97] disabled:opacity-50 shadow-lg shadow-indigo-200 flex items-center justify-center gap-3 uppercase tracking-widest text-sm"
                >
                  <i v-if="loading" class="fa-solid fa-circle-notch animate-spin"></i>
                  <span v-else>{{ t.calculate }}</span>
                </button>
              </div>
            </div>

            <!-- Result Card -->
            <transition name="fade">
              <div v-if="lastResult" class="mt-10 p-8 bg-gradient-to-br from-indigo-600 to-blue-700 rounded-[2rem] text-white shadow-2xl relative overflow-hidden">
                <div class="absolute -right-10 -bottom-10 opacity-10">
                  <i class="fa-solid fa-coins text-[12rem]"></i>
                </div>
                <div class="relative z-10">
                  <p class="text-indigo-100 text-xs font-bold uppercase tracking-widest mb-4">{{ t.resultTitle }}</p>
                  <div class="flex flex-col gap-1">
                    <span class="text-xl font-medium text-indigo-100/80">{{ lastResult.amount.toLocaleString() }} {{ lastResult.from }} {{ t.isRoughly }}</span>
                    <span class="text-6xl font-black tracking-tight">
                      {{ lastResult.result.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 }) }} <span class="text-3xl font-light opacity-60">{{ lastResult.to }}</span>
                    </span>
                  </div>
                  <div class="mt-8 flex flex-wrap gap-4 items-center">
                    <div class="px-3 py-1 bg-white/10 rounded-full text-[10px] font-mono">
                      1 {{ lastResult.from }} = {{ (lastResult.result / lastResult.amount).toFixed(6) }} {{ lastResult.to }}
                    </div>
                    <div class="px-3 py-1 bg-white/10 rounded-full text-[10px] font-mono">
                      1 {{ lastResult.to }} = {{ (lastResult.amount / lastResult.result).toFixed(6) }} {{ lastResult.from }}
                    </div>
                    <div class="px-3 py-1 bg-white/10 rounded-full text-[10px] font-mono">
                      Ref: {{ lastResult.date }}
                    </div>
                  </div>
                </div>
              </div>
            </transition>
          </div>
        </div>

        <!-- Right Column: Sidebar -->
        <div class="lg:col-span-4 space-y-8">
          <!-- Top Rates -->
          <div class="glass-card rounded-[2rem] p-6 shadow-sm">
            <h3 class="text-sm font-black uppercase tracking-widest text-slate-400 mb-6 px-2">{{ t.topCurrencies }}</h3>
            <div class="space-y-3">
              <div v-for="item in topRates" :key="item.code" class="group flex justify-between items-center p-4 rounded-2xl hover:bg-slate-50 transition-all border border-transparent hover:border-slate-100">
                <div class="flex items-center gap-3">
                   <div class="w-10 h-10 rounded-xl bg-slate-100 flex items-center justify-center font-bold text-slate-600 group-hover:bg-indigo-600 group-hover:text-white transition-colors">
                     {{ item.code[0] }}
                   </div>
                   <div>
                     <p class="font-bold text-slate-800">{{ item.code }}</p>
                     <p class="text-[10px] text-slate-400 font-bold uppercase">{{ t.vs }} 1.00 MYR</p>
                   </div>
                </div>
                <div class="text-right">
                  <p class="font-mono font-bold text-slate-700">{{ item.rate.toFixed(4) }}</p>
                </div>
              </div>
            </div>
          </div>

          <!-- History -->
          <div class="glass-card rounded-[2rem] p-6 shadow-sm">
            <div class="flex justify-between items-center mb-6 px-2">
              <h3 class="text-sm font-black uppercase tracking-widest text-slate-400">{{ t.history }}</h3>
              <button @click="history = []" v-if="history.length" class="text-[10px] font-bold text-indigo-600 hover:text-indigo-800 uppercase tracking-tighter">{{ t.clearAll }}</button>
            </div>
            <div class="space-y-4">
              <div v-if="!history.length" class="text-center py-12">
                <i class="fa-solid fa-clock-rotate-left text-slate-200 text-4xl mb-4"></i>
                <p class="text-xs text-slate-400 font-medium">{{ t.noHistory }}</p>
              </div>
              <transition-group name="list">
                <div v-for="item in history" :key="item.id" class="flex items-center gap-4 p-4 bg-slate-50 rounded-2xl border border-slate-100 group">
                  <div class="flex-shrink-0 w-2 h-2 rounded-full bg-indigo-500 group-hover:scale-150 transition-transform"></div>
                  <div class="flex-grow">
                    <p class="text-sm font-bold text-slate-800">
                      {{ item.amount }} {{ item.from }} <i class="fa-solid fa-arrow-right text-[10px] text-slate-300 mx-1"></i> {{ item.result.toFixed(2) }} {{ item.to }}
                    </p>
                    <p class="text-[10px] text-slate-400">{{ new Date(item.timestamp).toLocaleTimeString() }}</p>
                  </div>
                </div>
              </transition-group>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<style>
.fade-enter-active, .fade-leave-active {
  transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
}
.fade-enter-from, .fade-leave-to {
  opacity: 0;
  transform: translateY(20px);
}

.list-enter-active, .list-leave-active {
  transition: all 0.4s ease;
}
.list-enter-from, .list-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

/* Custom Dropdown Animation */
.dropdown-enter-active, .dropdown-leave-active {
  transition: all 0.2s ease-out;
}
.dropdown-enter-from, .dropdown-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

/* Hide arrow on select but we've added a custom one now */
select {
  background-image: none;
}
</style>
