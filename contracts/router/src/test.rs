#![cfg(test)]

use crate::{Router, RouterClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{symbol_short, vec, Address, Env, IntoVal, TryFromVal, Val, Vec};

#[test]
fn test_returned_values() {
    let e: Env = Env::from_ledger_snapshot_file("../../snapshots.json");
    let contract_id: Address = e.register(Router, ());
    let client: RouterClient = RouterClient::new(&e, &contract_id);

    let xbull_swap_contract: Address = Address::from_str(
        &e,
        "CB3JAPDEIMA3OOSALUHLYRGM2QTXGVD3EASALPFMVEU2POLLULJBT2XN",
    );

    let asset_xlm: Address = Address::from_str(
        &e,
        "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA",
    );
    let asset_usdx: Address = Address::from_str(
        &e,
        "CDIKURWHYS4FFTR5KOQK6MBFZA2K3E26WGBQI6PXBYWZ4XIOPJHDFJKP",
    );
    let asset_eurx: Address = Address::from_str(
        &e,
        "CBN3NCJSMOQTC6SPEYK3A44NU4VS3IPKTARJLI3Y77OH27EWBY36TP7U",
    );
    let asset_gbpx: Address = Address::from_str(
        &e,
        "CBCO65UOWXY2GR66GOCMCN6IU3Y45TXCPBY3FLUNL4AOUMOCKVIVV6JC",
    );
    let asset_usdc: Address = Address::from_str(
        &e,
        "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
    );
    let asset_eurc: Address = Address::from_str(
        &e,
        "CDTKPWPLOURQA2SGTKTUQOWRCBZEORB4BWBOMJ3D3ZTQQSGE5F6JBQLV",
    );

    let balances: Vec<Val> = client.exec(
        &Address::generate(&e),
        &vec![
            &e,
            (
                asset_xlm,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
            (
                asset_usdx,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
            (
                asset_eurx,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
            (
                asset_gbpx,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
            (
                asset_usdc,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
            (
                asset_eurc,
                symbol_short!("balance"),
                vec![&e, xbull_swap_contract.into_val(&e)],
            ),
        ],
    );

    assert_eq!(
        544759634i128,
        i128::try_from_val(&e, &balances.get(0).unwrap()).unwrap()
    );
    assert_eq!(
        161401360i128,
        i128::try_from_val(&e, &balances.get(1).unwrap()).unwrap()
    );
    assert_eq!(
        583214421i128,
        i128::try_from_val(&e, &balances.get(2).unwrap()).unwrap()
    );
    assert_eq!(
        296723247i128,
        i128::try_from_val(&e, &balances.get(3).unwrap()).unwrap()
    );
    assert_eq!(
        515925866i128,
        i128::try_from_val(&e, &balances.get(4).unwrap()).unwrap()
    );
    assert_eq!(
        6192131i128,
        i128::try_from_val(&e, &balances.get(5).unwrap()).unwrap()
    );
}
