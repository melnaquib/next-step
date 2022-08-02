use sp_std::collections::btree_map::BTreeMap;
use sp_std::*;
use sp_std::vec::*;

use bimap::btree::BiBTreeMap;

use super::*;

pub fn dev_accounts<T: Config>() -> BiBTreeMap<types::Str, T::AccountId> {
    BiBTreeMap::from_iter(vec![

        //MoonBeam
        // (types::from_str("Alith"), AccountId::from_ss58check("0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
        // (types::from_str("Baltathar"), AccountId::from_ss58check("0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")),
        // (types::from_str("Charleth"), AccountId::from_ss58check("0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")),
        // (types::from_str("Dorothy"), AccountId::from_ss58check("0x773539d4Ac0e786233D90A233654ccEE26a613D9")),
        // (types::from_str("Ethan"), AccountId::from_ss58check("0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")),
        // (types::from_str("Faith"), AccountId::from_ss58check("0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d")),
        // (types::from_str("Goliath"), AccountId::from_ss58check("0x7BF369283338E12C90514468aa3868A551AB2929")),
        // (types::from_str("Heath"), AccountId::from_ss58check("0x931f3600a299fd9B24cEfB3BfF79388D19804BeA")),
        // (types::from_str("Ida"), AccountId::from_ss58check("0xC41C5F1123ECCd5ce233578B2e7ebd5693869d73")),
        // (types::from_str("Judith"), AccountId::from_ss58check("0x2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423")),
        // 
        // ("Gerald", "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"),


        // (types::from_str("Alice"), AccountId::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap()),
        // (types::from_str("Bob"), AccountId::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap()),
        // (types::from_str("Charlie"), AccountId::from_ss58check("5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y").unwrap()),
        // (types::from_str("Dave"), AccountId::from_ss58check("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap()),
        // (types::from_str("Eve"), AccountId::from_ss58check("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw").unwrap()),
        // (types::from_str("Freddie"), AccountId::from_ss58check("5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL").unwrap()),
    ])
    
}

pub fn dev_accounts_evm() -> Vec<&'static str> {
    vec![
        // &"d43593c715fdd31c61141abd04a99fd6822c8558"
        &"f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac"

        //MoonBeam
        // (types::from_str("Alith"), AccountId::from_ss58check("0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")),
        // (types::from_str("Baltathar"), AccountId::from_ss58check("0x3Cd0A705a2DC65e5b1E1205896BaA2be8A07c6e0")),
        // (types::from_str("Charleth"), AccountId::from_ss58check("0x798d4Ba9baf0064Ec19eB4F0a1a45785ae9D6DFc")),
        // (types::from_str("Dorothy"), AccountId::from_ss58check("0x773539d4Ac0e786233D90A233654ccEE26a613D9")),
        // (types::from_str("Ethan"), AccountId::from_ss58check("0xFf64d3F6efE2317EE2807d223a0Bdc4c0c49dfDB")),
        // (types::from_str("Faith"), AccountId::from_ss58check("0xC0F0f4ab324C46e55D02D0033343B4Be8A55532d")),
        // (types::from_str("Goliath"), AccountId::from_ss58check("0x7BF369283338E12C90514468aa3868A551AB2929")),
        // (types::from_str("Heath"), AccountId::from_ss58check("0x931f3600a299fd9B24cEfB3BfF79388D19804BeA")),
        // (types::from_str("Ida"), AccountId::from_ss58check("0xC41C5F1123ECCd5ce233578B2e7ebd5693869d73")),
        // (types::from_str("Judith"), AccountId::from_ss58check("0x2898FE7a42Be376C8BC7AF536A940F7Fd5aDd423")),
        // 
        // ("Gerald", "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"),


        // (types::from_str("Alice"), AccountId::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap()),
        // (types::from_str("Bob"), AccountId::from_ss58check("5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty").unwrap()),
        // (types::from_str("Charlie"), AccountId::from_ss58check("5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y").unwrap()),
        // (types::from_str("Dave"), AccountId::from_ss58check("5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy").unwrap()),
        // (types::from_str("Eve"), AccountId::from_ss58check("5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw").unwrap()),
        // (types::from_str("Freddie"), AccountId::from_ss58check("5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL").unwrap()),
    ]
    
}
