use crate::model::{Transfer, UserStats};
use anyhow::Result;
use std::collections::HashMap;

struct AggregatedData {
    max_balances: HashMap<String, f64>,
    buy_prices: HashMap<String, Vec<(f64, f64)>>,
    sell_prices: HashMap<String, Vec<(f64, f64)>>,
}

pub fn calculate_balance_history(transfers: &[Transfer]) -> HashMap<String, Vec<(u64, f64)>> {
    let mut balance_history: HashMap<String, Vec<(u64, f64)>> = HashMap::new();
    let mut balances: HashMap<String, f64> = HashMap::new();

    for t in transfers {
        let from_balance = balances.entry(t.address_from.clone()).or_default();

        *from_balance -= t.amount;

        balance_history
            .entry(t.address_from.clone())
            .or_default()
            .push((t.ts, *from_balance));

        let to_balance = balances.entry(t.address_to.clone()).or_default();
        *to_balance += t.amount;

        balance_history
            .entry(t.address_to.clone())
            .or_default()
            .push((t.ts, *to_balance));
    }

    balance_history
}

fn aggregate_transfers(transfers: &[Transfer]) -> AggregatedData {
    let mut balances: HashMap<String, f64> = HashMap::new();
    let mut max_balances: HashMap<String, f64> = HashMap::new();
    let mut buy_prices: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    let mut sell_prices: HashMap<String, Vec<(f64, f64)>> = HashMap::new();

    for t in transfers {
        *balances.entry(t.address_from.clone()).or_default() -= t.amount;
        *balances.entry(t.address_to.clone()).or_default() += t.amount;
        let to_balance = balances.get(&t.address_to).copied().unwrap_or(0.0);
        let from_balance = balances.get(&t.address_from).copied().unwrap_or(0.0);
        let max_to = max_balances.get(&t.address_to).copied().unwrap_or(0.0);

        if to_balance > max_to {
            max_balances.insert(t.address_to.clone(), to_balance);
        }

        let max_from = max_balances.get(&t.address_from).copied().unwrap_or(0.0);

        if from_balance > max_from {
            max_balances.insert(t.address_from.clone(), from_balance);
        }

        buy_prices
            .entry(t.address_to.clone())
            .or_default()
            .push((t.usd_price, t.amount));

        sell_prices
            .entry(t.address_from.clone())
            .or_default()
            .push((t.usd_price, t.amount));
    }

    AggregatedData {
        max_balances,
        buy_prices,
        sell_prices,
    }
}

fn correct_max_balance_for_sellers(agg: &mut AggregatedData) {
    for (addr, sells) in &agg.sell_prices {
        let buys = agg.buy_prices.get(addr).cloned().unwrap_or_default();

        if buys.is_empty() && !sells.is_empty() {
            let sum_sells = sells.iter().map(|(_, amt)| *amt).sum::<f64>();
            let entry = agg.max_balances.entry(addr.clone()).or_insert(0.0);
            *entry = sum_sells;
        } else if !sells.is_empty() {
            let max_sell = sells.iter().map(|(_, amt)| *amt).fold(0.0, f64::max);
            let entry = agg.max_balances.entry(addr.clone()).or_insert(0.0);
            if *entry < max_sell {
                *entry = max_sell;
            }
        }
    }
}

fn weighted_avg(data: &[(f64, f64)]) -> f64 {
    let (sum_px, sum_amt): (f64, f64) = data
        .iter()
        .copied()
        .fold((0.0, 0.0), |acc, (p, a)| (acc.0 + p * a, acc.1 + a));
    if sum_amt > 0.0 {
        sum_px / sum_amt
    } else {
        0.0
    }
}

fn build_user_stats(transfers: &[Transfer], agg: &AggregatedData) -> Vec<UserStats> {
    let all_addresses: std::collections::HashSet<_> = transfers
        .iter()
        .flat_map(|t| vec![t.address_from.clone(), t.address_to.clone()])
        .collect();

    let balance_history = calculate_balance_history(transfers);

    all_addresses
        .into_iter()
        .map(|addr| {
            let buys = agg.buy_prices.get(&addr).cloned().unwrap_or_default();
            let sells = agg.sell_prices.get(&addr).cloned().unwrap_or_default();

            let total_volume: f64 = transfers
                .iter()
                .filter(|t| t.address_from == addr || t.address_to == addr)
                .map(|t| t.amount.max(0.0))
                .sum();

            let max_balance = balance_history
                .get(&addr)
                .map(|hist| hist.iter().map(|(_, bal)| *bal).fold(0.0, f64::max))
                .unwrap_or(0.0);

            UserStats {
                address: addr.clone(),
                total_volume,
                avg_buy_price: weighted_avg(&buys),
                avg_sell_price: weighted_avg(&sells),
                max_balance,
            }
        })
        .collect()
}

pub fn calculate_user_stats(transfers: &[Transfer]) -> Result<Vec<UserStats>> {
    let mut aggregate_data = aggregate_transfers(transfers);
    correct_max_balance_for_sellers(&mut aggregate_data);
    Ok(build_user_stats(transfers, &aggregate_data))
}
