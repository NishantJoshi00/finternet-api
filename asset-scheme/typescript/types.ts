type Asset = {
  readonly id: string;
  isLocked: boolean;
  readonly ledger: string;
};

type FungibleAsset = Asset & {
  readonly type: "fungible";
  readonly value: number;
};

type NonFungibleAsset = Asset & {
  readonly type: "non-fungible";
  readonly signature: string;
};

type Money = FungibleAsset & {
  readonly currency: string;
};

type Property = NonFungibleAsset & {
  readonly lat: number;
  readonly long: number;
  readonly size: number;
};

type RBIMoney = Money & {
  readonly issuer: "RBI";
  readonly currency: "INR";
};

type BRLMoney = Money & {
  readonly issuer: "BRL";
  readonly currency: "BRL";
};

type IndianProperty = Property & {
  readonly country: "India";
};

type BrazilianProperty = Property & {
  readonly country: "Brazil";
};

type ICICIMoney = RBIMoney & {
  readonly regulator: "ICICI";
};

type HDFCMoney = RBIMoney & {
  readonly regulator: "HDFC";
};

type JKProperty = IndianProperty & {
  readonly state: "Jammu and Kashmir";
};

type KarnatakaProperty = IndianProperty & {
  readonly state: "Karnataka";
};

type Participant = {
  readonly publicKey: string;
  readonly id: string;
};

interface AssetAction {
  send: (asset: Asset, to: Participant) => void;
  lock: (asset: Asset) => void;
  unlock: (asset: Asset) => void;
}

interface MoneyAction extends AssetAction {
  sendMoney: (money: Money, amount: number, to: Participant) => void;
  lendMoney: (money: Money, amount: number, to: Participant) => void;
}

interface PropertyAction extends AssetAction {
  sell: (property: Property, to: Participant) => void;
  buy: (property: Property, from: Participant) => void;
  lease: (property: Property, to: Participant) => void;
}

interface RBIAction extends MoneyAction {
  issueMoney: (money: Money, amount: number) => RBIMoney;
}

interface BRLAction extends MoneyAction {
  issueMoney: (money: Money, amount: number) => BRLMoney;
}

interface ICICIAction extends MoneyAction {
  regulateMoney: (money: RBIMoney) => ICICIMoney;
}

interface HDFCAction extends MoneyAction {
  regulateMoney: (money: RBIMoney) => HDFCMoney;
}

interface IndianPropertyAction extends PropertyAction {
  register: (property: Property) => IndianProperty;
}

interface BrazilianPropertyAction extends PropertyAction {
  register: (property: Property) => BrazilianProperty;
}

interface JKPropertyAction extends IndianPropertyAction {
  register: (property: Property) => JKProperty;
}

interface KarnatakaPropertyAction extends IndianPropertyAction {
  register: (property: Property) => KarnatakaProperty;
}

