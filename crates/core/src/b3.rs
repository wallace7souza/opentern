//! B3, Brazil's exchange (`BMFBOVESPA:PETR4`). Prices come from Yahoo
//! (`PETR4.SA`, 15 minutes late); the session and holidays are
//! `calendar::Venue::SaoPaulo`.

use crate::{AssetKind, Symbol};

/// The exchange prefix of B3 symbols, as on TradingView.
pub const EXCHANGE: &str = "BMFBOVESPA";
pub const TIMEZONE: &str = "America/Sao_Paulo";

/// A B3 symbol from its ticker, before its details load.
pub fn symbol(ticker: &str, name: &str, kind: AssetKind) -> Symbol {
    Symbol {
        id: format!("{EXCHANGE}:{ticker}"),
        description: name.into(),
        kind,
        currency: "BRL".into(),
        tick_size: 0.01,
        min_qty: 0.0,
        timezone: TIMEZONE.into(),
    }
}

/// The B3 symbols search knows without asking the network: the Ibovespa's
/// members and the most traded ETFs, real estate funds (FIIs) and BDRs.
/// Any other B3 ticker is found through Yahoo's search or typed as
/// `BMFBOVESPA:TICKER`.
pub fn symbols() -> Vec<Symbol> {
    crate::b3_list::B3
        .iter()
        .map(|(t, name, kind)| symbol(t, name, *kind))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_is_b3_and_unique() {
        let all = symbols();
        assert!(all.len() > 50, "{}", all.len());
        let mut ids: Vec<_> = all.iter().map(|s| s.id.as_str()).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), all.len(), "no ticker twice");
        assert!(
            all.iter()
                .all(|s| s.exchange() == EXCHANGE && !s.description.is_empty())
        );
        let petr4 = all.iter().find(|s| s.ticker() == "PETR4").expect("PETR4 listed");
        assert_eq!(petr4.currency, "BRL");
        assert_eq!(petr4.kind, AssetKind::Stock);
        let bova = all.iter().find(|s| s.ticker() == "BOVA11").expect("BOVA11 listed");
        assert_eq!(bova.kind, AssetKind::Fund);
    }
}
