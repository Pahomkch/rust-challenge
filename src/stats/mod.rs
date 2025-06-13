use crate::model::{Transfer, UserStats};
use std::collections::HashMap;

pub fn calculate_user_stats(transfers: &[Transfer]) -> Vec<UserStats> {
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

    for (addr, sells) in &sell_prices {
        let buys = buy_prices.get(addr).cloned().unwrap_or_default();

        if buys.is_empty() && !sells.is_empty() {
            let sum_sells = sells.iter().map(|(_, amt)| *amt).sum::<f64>();
            let entry = max_balances.entry(addr.clone()).or_insert(0.0);
            *entry = sum_sells;
        } else if !sells.is_empty() {
            let max_sell = sells.iter().map(|(_, amt)| *amt).fold(0.0, f64::max);
            let entry = max_balances.entry(addr.clone()).or_insert(0.0);

            if *entry < max_sell {
                *entry = max_sell;
            }
        }
    }

    let all_addresses: std::collections::HashSet<_> = transfers
        .iter()
        .flat_map(|t| vec![t.address_from.clone(), t.address_to.clone()])
        .collect();

    all_addresses
        .into_iter()
        .map(|addr| {
            let buys = buy_prices.get(&addr).cloned().unwrap_or_default();
            let sells = sell_prices.get(&addr).cloned().unwrap_or_default();

            let total_volume: f64 = transfers
                .iter()
                .filter(|t| t.address_from == addr || t.address_to == addr)
                .map(|t| t.amount.max(0.0))
                .sum();

            let avg = |data: &[(f64, f64)]| {
                let (sum_px, sum_amt): (f64, f64) = data
                    .iter()
                    .copied()
                    .fold((0.0, 0.0), |acc, (p, a)| (acc.0 + p * a, acc.1 + a));

                if sum_amt > 0.0 {
                    sum_px / sum_amt
                } else {
                    0.0
                }
            };

            UserStats {
                address: addr.clone(),
                total_volume,
                avg_buy_price: avg(&buys),
                avg_sell_price: avg(&sells),
                max_balance: *max_balances.get(&addr).unwrap_or(&0.0),
            }
        })
        .collect()
}
