use gemini_rust::{FeeVolumeAPI, GeminiClient};
mod cfg;
use cfg::Cfg;

static _BTCUSD: &str = "btcusd";

fn client() -> GeminiClient {
    let cfg = Cfg::new().unwrap();
    println!("api.key={:?} api.sec={:?}", cfg.api.key, cfg.api.sec);
    GeminiClient::new(&cfg.api.url, &cfg.api.key, &cfg.api.sec)
}

#[test]
fn test_fee_volume_get_notational_volume() {
    let client = client();

    // let notional_volume = client.get_notional_volume().expect("error with request");
    // assert_eq!(notional_volume.date.len(), 10, "date format incorrect");
}

#[test]
fn test_fee_volume_get_trade_volume() {
    let client = client();

    // let _trade_volume = client.get_trade_volume().expect("error with request");
    // assert_ne!(_trade_volume.len(), 0, "zero len trade vol");
    // assert_ne!(_trade_volume[0].len(), 0, "zero len trade vol");
    // assert_eq!(
    //    _trade_volume[0][0].data_date.len(),
    //    10,
    //    "date format incorrect"
    // );
}