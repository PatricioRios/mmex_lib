use mmex_lib::api::MmexContext;
use rust_decimal_macros::dec;

#[test]
fn test_currency_integration() {
    let ctx = MmexContext::open_memory().unwrap();
    
    // Setup desde tables.sql
    let schema = "
        CREATE TABLE CURRENCYFORMATS_V1(
            CURRENCYID integer primary key
            , CURRENCYNAME TEXT COLLATE NOCASE NOT NULL UNIQUE
            , PFX_SYMBOL TEXT
            , SFX_SYMBOL TEXT
            , DECIMAL_POINT TEXT
            , GROUP_SEPARATOR TEXT
            , UNIT_NAME TEXT COLLATE NOCASE
            , CENT_NAME TEXT COLLATE NOCASE
            , SCALE integer
            , BASECONVRATE numeric
            , CURRENCY_SYMBOL TEXT COLLATE NOCASE NOT NULL UNIQUE
            , CURRENCY_TYPE TEXT NOT NULL
        );
        INSERT INTO CURRENCYFORMATS_V1 VALUES(1,'US dollar','$','','.',',','Dollar','Cent',100,1.25,'USD','Fiat');
    ";
    ctx.execute_setup(schema).expect("Failed to setup Currencies table");
    
    let service = ctx.currencies();
    
    // 1. Obtener todos
    let all = service.get_all_currencies().expect("Failed to list");
    assert_eq!(all.len(), 1);
    
    // 2. Validar campos
    let usd = &all[0];
    assert_eq!(usd.symbol, "USD");
    assert_eq!(usd.pfx_symbol.as_deref(), Some("$"));
    assert_eq!(usd.base_conv_rate, dec!(1.25));
    
    // 3. Buscar por símbolo
    let found = service.get_currency_by_symbol("USD").expect("Failed to find by symbol");
    assert!(found.is_some());
}
