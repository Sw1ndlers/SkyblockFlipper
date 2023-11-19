export type Instant = {
    nanos: number,
    secs: number,
}

export type AuctionType = {
    auctioneer: string,
    time_remaining: Instant,
    item_name: string,
    price: number,
    lowest_price: number,
    profit: number,
    profit_percent: number,
    uuid: string,
}