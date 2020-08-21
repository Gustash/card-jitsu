use card_jitsu::utils;

#[test]
fn it_deals_five_cards() {
    let hand = utils::deal_hand();
    assert_eq!(hand.len(), 5);
}
