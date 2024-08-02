struct Asset {
    id: String,
    is_locked: bool,
    ledger: String,
}

struct FungibleAsset {
    core: Box<dyn IAsset>,
    value: u64,
}

struct NonFungibleAsset {
    core: Box<dyn IAsset>,
    signature: u64,
}

struct Money {
    core: Box<dyn IFungibleAsset>,
    currency: String,
}

struct Property {
    core: Box<dyn INonFungibleAsset>,
    latitude: f64,
    longitude: f64,
    size: u64,
}

struct RBIMoney<const ISSUER: &'static str = "RBI", const CURRENCY: &'static str = "INR"> {
    core: Box<dyn IMoney>,
}

struct BRLMoney<const ISSUER: &'static str = "BRL", const CURRENCY: &'static str = "BRL"> {
    core: Box<dyn IMoney>,
}

struct IndianProperty<const COUNTRY: &'static str = "IndianProperty"> {
    core: Box<dyn IProperty>,
}

struct BrazilianProperty<const COUNTRY: &'static str = "BrazilianProperty"> {
    core: Box<dyn IProperty>,
}

struct JKProperty<const STATE: &'static str = "JK"> {
    core: Box<dyn IIndianProperty>,
}

struct KProperty<const STATE: &'static str = "Karnataka"> {
    core: Box<dyn IIndianProperty>,
}

struct ICICIMoney<const REGULATOR: &'static str = "ICICI"> {
    core: Box<dyn IRBIMoney>,
}

trait IAsset {
    fn send(&self, to: &str);
    fn lock(&mut self);
    fn unlock(&mut self);
}

trait IFungibleAsset {
    fn send(&self, to: &str, value: u64);
}

trait INonFungibleAsset {}

trait IMoney {
    fn lend(&self, to: &str, value: u64);
}

trait IProperty {
    fn sell(&self, to: &str, value: u64);
}

trait IIndianProperty {}

trait IBrazilianProperty {}

trait IRBIMoney {
    fn issue(&self, to: &str, value: u64);
}

trait IBRLMoney {
    fn issue(&self, to: &str, value: u64);
}

trait IJKProperty {
    fn register(&self, to: &str, value: u64);
}

trait IKProperty {
    fn register(&self, to: &str, value: u64);
}

struct Execute<T, F, P> {
    data: T,
    func: F,
    participant: P,
    ruleset: Vec<Rule>,
}

struct Rule {
    condition: String,
    action: String,
    resource: String,
    authority: String,
}
